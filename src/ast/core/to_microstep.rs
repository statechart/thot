use ast::conversion_error::Errors;
use ast::core;
use ast::location::Location;
use ast::microstep::*;

impl Into<Result<Microstep, Errors>> for core::Core {
    fn into(self) -> Result<Microstep, Errors> {
        let core::Core {
            states,
            transitions,
            loc,
        } = self;
        let configuration_size = states.len();
        let errors = vec![];

        let microstep = Microstep {
            configuration_size,
            init: gen_init(&states, &transitions, loc),
            next: gen_select_transitions(&states, &transitions, loc),
            loc,
            ..Default::default()
        };

        if errors.len() > 0 {
            Err(errors)
        } else {
            Ok(microstep)
        }
    }
}

const CONFIGURATION_PREFIX: &'static str = "c";
const INITIALIZED_PREFIX: &'static str = "i";
const HISTORY_PREFIX: &'static str = "h";
const ENTRY_PREFIX: &'static str = "e";
const TRANS_PREFIX: &'static str = "t";
const EXIT_PREFIX: &'static str = "x";
const AVAILABLE_TRANS_PREFIX: &'static str = "a";
const ENTRY_GUARD_PREFIX: &'static str = "g";

fn gen_init(
    states: &Vec<core::State>,
    transitions: &Vec<core::Transition>,
    loc: Location,
) -> Function {
    let mut body = vec![];
    body.append(&mut gen_empty_configuration(
        CONFIGURATION_PREFIX.to_string(),
        gen_bool(false, loc),
        &states,
    ));
    body.append(&mut gen_empty_configuration(
        INITIALIZED_PREFIX.to_string(),
        gen_bool(false, loc),
        &states,
    ));
    body.append(&mut gen_empty_configuration(
        HISTORY_PREFIX.to_string(),
        gen_bool(false, loc),
        &states,
    ));
    body.append(&mut gen_empty_configuration(
        ENTRY_PREFIX.to_string(),
        gen_bool(false, loc),
        &states,
    ));
    body.append(&mut gen_empty_configuration(
        TRANS_PREFIX.to_string(),
        gen_bool(false, loc),
        &states,
    ));
    body.append(&mut gen_empty_configuration(
        EXIT_PREFIX.to_string(),
        gen_bool(false, loc),
        &states,
    ));

    body.append(&mut gen_establish_entryset(&states, &transitions, loc));

    Function {
        params: vec![],
        body,
        loc,
    }
}

fn gen_select_transitions(
    states: &Vec<core::State>,
    transitions: &Vec<core::Transition>,
    loc: Location,
) -> Function {
    let configuration_ident = Identifier {
        name: CONFIGURATION_PREFIX.to_string(),
        loc,
    };
    let initialized_ident = Identifier {
        name: INITIALIZED_PREFIX.to_string(),
        loc,
    };
    let history_ident = Identifier {
        name: HISTORY_PREFIX.to_string(),
        loc,
    };
    let has_event = Identifier {
        name: "has_event".to_string(),
        loc,
    };
    let is_stable_ident = Identifier {
        name: "is_stable".to_string(),
        loc,
    };

    let mut body = vec![];
    body.push(gen_destruct(&configuration_ident, &states));
    body.push(gen_destruct(&initialized_ident, &states));
    body.push(gen_destruct(&history_ident, &states));
    body.append(&mut gen_empty_configuration(
        ENTRY_PREFIX.to_string(),
        gen_bool(false, loc),
        &states,
    ));
    body.append(&mut gen_empty_configuration(
        EXIT_PREFIX.to_string(),
        gen_bool(false, loc),
        &states,
    ));
    body.append(&mut gen_empty_configuration(
        AVAILABLE_TRANS_PREFIX.to_string(),
        gen_bool(true, loc),
        &states,
    ));
    body.push(gen_var(is_stable_ident.clone(), gen_bool(true, loc), loc));
    body.append(&mut gen_transition_select(
        &transitions,
        &has_event,
        &is_stable_ident,
    ));

    body.push(Statement::ReturnStatement(ReturnStatement {
        argument: Expression::MicrostepResult(MicrostepResult {
            configuration: SimpleExpression::Identifier(configuration_ident.clone()),
            initialized: SimpleExpression::Identifier(initialized_ident.clone()),
            history: SimpleExpression::Identifier(history_ident.clone()),
            is_stable: gen_bool(true, loc).to_simple(),
            loc,
        }),
        guard: Some(Expression::Identifier(is_stable_ident)),
        loc,
    }));

    // TODO remember history
    body.append(&mut gen_establish_entryset(&states, &transitions, loc));

    Function {
        params: vec![
            Expression::Identifier(configuration_ident),
            Expression::Identifier(initialized_ident),
            Expression::Identifier(history_ident),
            Expression::Identifier(has_event),
        ],
        body,
        loc,
    }
}

fn gen_establish_entryset(
    states: &Vec<core::State>,
    transitions: &Vec<core::Transition>,
    loc: Location,
) -> Vec<Statement> {
    let mut body = vec![];

    body.append(&mut gen_entryset_entry_ancestors(states));
    body.append(&mut gen_entryset_entry_descendants(states, transitions));
    body.append(&mut gen_entryset_exit_states(states));
    body.append(&mut gen_entryset_take_transitions(transitions));
    body.append(&mut gen_entryset_enter_states(states));

    body.push(Statement::ReturnStatement(ReturnStatement {
        argument: Expression::MicrostepResult(MicrostepResult {
            configuration: gen_construct(ENTRY_PREFIX, states).to_simple(),
            initialized: gen_construct(INITIALIZED_PREFIX, states).to_simple(),
            history: gen_construct(HISTORY_PREFIX, states).to_simple(),
            is_stable: gen_bool(false, loc).to_simple(),
            loc,
        }),
        guard: None,
        loc,
    }));

    body
}

fn gen_entryset_entry_ancestors(states: &Vec<core::State>) -> Vec<Statement> {
    let mut statements = vec![];
    let num_states = states.len() - 1;
    for state in states.iter().rev() {
        let loc = state.loc;
        let descendants = &state.descendants;
        let id = Identifier {
            name: format!("{}{}", ENTRY_PREFIX, state.idx),
            loc,
        };
        match descendants.len() {
            len if len == num_states => {
                statements.push(gen_assign(id, gen_bool(true, loc), loc));
            }
            0 => (),
            _ => {
                statements.push(gen_assign(
                    id,
                    Expression::LogicalExpression(LogicalExpression {
                        operator: LogicalOperator::Or,
                        arguments: state
                            .children
                            .iter()
                            .map(|d| {
                                Expression::Identifier(Identifier {
                                    name: format!("{}{}", ENTRY_PREFIX, d),
                                    loc,
                                })
                            })
                            .collect(),
                        loc,
                    }),
                    loc,
                ));
            }
        }
    }
    statements
}

fn gen_entryset_entry_descendants(
    states: &Vec<core::State>,
    transitions: &Vec<core::Transition>,
) -> Vec<Statement> {
    let mut statements = vec![];
    for state in states {
        let loc = state.loc;
        let id = Identifier {
            name: format!("{}{}", ENTRY_PREFIX, state.idx),
            loc,
        };
        match state.t {
            core::StateType::Parallel => {
                statements.append(&mut gen_union(
                    &Expression::Identifier(id),
                    &ENTRY_PREFIX.to_string(),
                    &state.initial,
                    loc,
                ));
            }
            // TODO implement other types
            _ => (),
        }
    }
    statements
}

fn gen_entryset_exit_states(states: &Vec<core::State>) -> Vec<Statement> {
    let mut statements = vec![];
    for state in states.iter().rev() {
        let loc = state.loc;
        let idx = state.idx;

        for id in &state.on_exit {
            statements.push(Statement::ExecuteStatement(ExecuteStatement {
                id: *id,
                guard: Some(gen_and(
                    Expression::Identifier(Identifier {
                        name: format!("{}{}", CONFIGURATION_PREFIX, idx),
                        loc,
                    }),
                    Expression::Identifier(Identifier {
                        name: format!("{}{}", EXIT_PREFIX, idx),
                        loc,
                    }),
                    loc,
                )),
                loc,
            }));
        }
    }
    statements
}

fn gen_entryset_take_transitions(transitions: &Vec<core::Transition>) -> Vec<Statement> {
    let mut statements = vec![];
    for transition in transitions {
        let loc = transition.loc;
        let idx = transition.idx;
        for id in &transition.on_transition {
            statements.push(Statement::ExecuteStatement(ExecuteStatement {
                id: *id,
                guard: Some(Expression::Identifier(Identifier {
                    name: format!("{}{}", TRANS_PREFIX, idx),
                    loc,
                })),
                loc,
            }));
        }
    }
    statements
}

fn gen_entryset_enter_states(states: &Vec<core::State>) -> Vec<Statement> {
    let mut statements = vec![];
    for state in states {
        let loc = state.loc;
        let idx = state.idx;
        let guard_ident = Identifier {
            name: format!("{}{}", ENTRY_GUARD_PREFIX, idx),
            loc,
        };
        let entry = Expression::Identifier(Identifier {
            name: format!("{}{}", ENTRY_PREFIX, idx),
            loc,
        });

        if state.on_init.len() + state.on_enter.len() > 0 {
            statements.push(gen_var(
                guard_ident.clone(),
                gen_and(
                    gen_not(
                        Expression::Identifier(Identifier {
                            name: format!("{}{}", CONFIGURATION_PREFIX, idx),
                            loc,
                        }),
                        loc,
                    ),
                    entry.clone(),
                    loc,
                ),
                loc,
            ));
        }

        let initital_ident = Identifier {
            name: format!("{}{}", INITIALIZED_PREFIX, idx),
            loc,
        };

        for id in &state.on_init {
            statements.push(Statement::ExecuteStatement(ExecuteStatement {
                id: *id,
                guard: Some(gen_and(
                    gen_not(Expression::Identifier(initital_ident.clone()), loc),
                    Expression::Identifier(guard_ident.clone()),
                    loc,
                )),
                loc,
            }));
        }

        statements.push(gen_assign(
            initital_ident.clone(),
            gen_or(entry, Expression::Identifier(initital_ident), loc),
            loc,
        ));

        for id in &state.on_enter {
            statements.push(Statement::ExecuteStatement(ExecuteStatement {
                id: *id,
                guard: Some(Expression::Identifier(guard_ident.clone())),
                loc,
            }));
        }
    }
    statements
}

fn gen_destruct(configuration: &Identifier, states: &Vec<core::State>) -> Statement {
    let left = (0..states.len())
        .map(|index| {
            let loc = states[index].loc;
            Expression::Identifier(Identifier {
                name: format!("{}{}", configuration.name, index),
                loc,
            })
        })
        .collect();

    Statement::ConfigurationDestructureDeclaration(ConfigurationDestructureDeclaration {
        left,
        right: Expression::Identifier(configuration.clone()),
        loc: states[0].loc,
    })
}

fn gen_construct(prefix: &'static str, states: &Vec<core::State>) -> Expression {
    let arguments = (0..states.len())
        .map(|index| {
            let loc = states[index].loc;
            Expression::Identifier(Identifier {
                name: format!("{}{}", prefix, index),
                loc,
            })
        })
        .collect();

    Expression::ConfigurationCreateExpression(ConfigurationCreateExpression {
        arguments,
        loc: states[0].loc,
    })
}

fn gen_empty_configuration(
    name: String,
    init: Expression,
    states: &Vec<core::State>,
) -> Vec<Statement> {
    (0..states.len())
        .map(|index| {
            let loc = states[index].loc;
            gen_var(
                Identifier {
                    name: format!("{}{}", name, index),
                    loc,
                },
                init.clone(),
                loc,
            )
        })
        .collect()
}

fn gen_transition_select(
    transitions: &Vec<core::Transition>,
    has_event: &Identifier,
    is_stable: &Identifier,
) -> Vec<Statement> {
    let mut statements = vec![];
    for transition in transitions {
        match transition.t {
            core::TransitionType::History | core::TransitionType::Initial => (),
            _ => {
                let loc = transition.loc;
                let transition_ident = Identifier {
                    name: format!("{}{}", TRANS_PREFIX, transition.idx),
                    loc,
                };

                let guard = Expression::Identifier(transition_ident.clone());

                statements.push(gen_var(
                    transition_ident,
                    Expression::LogicalExpression(LogicalExpression {
                        operator: LogicalOperator::And,
                        arguments: vec![
                            gen_is_transition_available(&transition),
                            gen_is_transition_active(&transition),
                            gen_is_transition_applicable(&transition, has_event),
                            gen_is_transition_enabled(&transition),
                        ],
                        loc,
                    }),
                    loc,
                ));

                statements.append(&mut gen_union(
                    &guard,
                    &ENTRY_PREFIX.to_string(),
                    &transition.targets,
                    loc,
                ));
                statements.append(&mut gen_union(
                    &guard,
                    &EXIT_PREFIX.to_string(),
                    &transition.exits,
                    loc,
                ));
                statements.append(&mut gen_intersection(
                    &gen_not(guard.clone(), loc),
                    &AVAILABLE_TRANS_PREFIX.to_string(),
                    &transition.conflicts,
                    loc,
                ));
                statements.push(gen_assign(
                    is_stable.clone(),
                    gen_and(
                        gen_not(guard, loc),
                        Expression::Identifier(is_stable.clone()),
                        loc,
                    ),
                    loc,
                ));
            }
        }
    }

    statements
}

fn gen_is_transition_active(transition: &core::Transition) -> Expression {
    let config_check = Expression::Identifier(Identifier {
        name: format!("{}{}", CONFIGURATION_PREFIX, transition.source),
        loc: transition.loc,
    });
    match transition.t {
        // TODO handle spontaneous
        core::TransitionType::Spontaneous => config_check,
        _ => config_check,
    }
}

fn gen_is_transition_available(transition: &core::Transition) -> Expression {
    Expression::Identifier(Identifier {
        name: format!("{}{}", AVAILABLE_TRANS_PREFIX, transition.idx),
        loc: transition.loc,
    })
}

fn gen_is_transition_applicable(
    transition: &core::Transition,
    has_event: &Identifier,
) -> Expression {
    // TODO if we have an event and this has an event and the event_id matches
    Expression::Identifier(Identifier {
        name: format!("{}{}", AVAILABLE_TRANS_PREFIX, transition.idx),
        loc: transition.loc,
    })
}

fn gen_is_transition_enabled(transition: &core::Transition) -> Expression {
    let loc = transition.loc;
    match transition.condition {
        Some(id) => Expression::ConditionExpression(ConditionExpression { id, loc }),
        None => gen_bool(true, loc),
    }
}

fn gen_union(
    guard: &Expression,
    prefix: &String,
    ids: &Vec<core::StateId>,
    loc: Location,
) -> Vec<Statement> {
    gen_merge(guard, LogicalOperator::Or, prefix, ids, loc)
}

fn gen_intersection(
    guard: &Expression,
    prefix: &String,
    ids: &Vec<core::StateId>,
    loc: Location,
) -> Vec<Statement> {
    gen_merge(guard, LogicalOperator::And, prefix, ids, loc)
}

fn gen_not(expr: Expression, loc: Location) -> Expression {
    Expression::LogicalExpression(LogicalExpression {
        operator: LogicalOperator::Not,
        arguments: vec![expr],
        loc,
    })
}

fn gen_or(lhs: Expression, rhs: Expression, loc: Location) -> Expression {
    Expression::LogicalExpression(LogicalExpression {
        operator: LogicalOperator::Or,
        arguments: vec![lhs.clone(), rhs.clone()],
        loc,
    })
}

fn gen_and(lhs: Expression, rhs: Expression, loc: Location) -> Expression {
    Expression::LogicalExpression(LogicalExpression {
        operator: LogicalOperator::And,
        arguments: vec![lhs.clone(), rhs.clone()],
        loc,
    })
}

fn gen_bool(value: bool, loc: Location) -> Expression {
    Expression::BooleanLiteral(BooleanLiteral { value, loc })
}

fn gen_var(id: Identifier, init: Expression, loc: Location) -> Statement {
    Statement::VariableDeclaration(VariableDeclaration {
        id: VariableDeclarationId::Identifier(id),
        init,
        loc,
    })
}

fn gen_assign(left: Identifier, right: Expression, loc: Location) -> Statement {
    Statement::AssignmentStatement(AssignmentStatement {
        left: AssignmentStatementLeft::Identifier(left),
        right,
        loc,
    })
}

fn gen_merge(
    guard: &Expression,
    operator: LogicalOperator,
    prefix: &String,
    ids: &Vec<core::StateId>,
    loc: Location,
) -> Vec<Statement> {
    (0..ids.len())
        .map(|index| {
            let id = Identifier {
                name: format!("{}{}", prefix, index),
                loc,
            };
            gen_assign(
                id.clone(),
                Expression::LogicalExpression(LogicalExpression {
                    operator,
                    arguments: vec![guard.clone(), Expression::Identifier(id)],
                    loc,
                }),
                loc,
            )
        })
        .collect()
}

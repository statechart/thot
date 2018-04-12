use ast::conversion_error::{ConversionError, Errors};
use ast::core;
use ast::microstep::*;

impl Into<Result<Microstep, Errors>> for core::Core {
    fn into(self) -> Result<Microstep, Errors> {
        let core::Core {
            states,
            transitions,
        } = self;
        let configuration_size = states.len();
        let errors = vec![];

        let microstep = Microstep {
            configuration_size,
            select_transitions: gen_select_transitions(&states, &transitions),
            ..Default::default()
        };

        if errors.len() > 0 {
            Err(errors)
        } else {
            Ok(microstep)
        }
    }
}

fn gen_select_transitions(
    states: &Vec<core::State>,
    transitions: &Vec<core::Transition>,
) -> Function {
    let loc = states[0].loc;

    let configuration_ident = Identifier {
        name: "c".to_string(),
        loc,
    };
    let mut configuration_destruct = gen_destruct(&configuration_ident, &states);

    let initialized_ident = Identifier {
        name: "i".to_string(),
        loc,
    };
    let mut initialized_destruct = gen_destruct(&initialized_ident, &states);

    let history_ident = Identifier {
        name: "h".to_string(),
        loc,
    };
    let mut history_destruct = gen_destruct(&history_ident, &states);

    let is_stable_ident = Identifier {
        name: "is_stable".to_string(),
        loc,
    };

    let mut entry_set = gen_empty_configuration("e".to_string(), &states);
    let mut trans_set = gen_empty_configuration("t".to_string(), &states);
    let mut exit_set = gen_empty_configuration("x".to_string(), &states);
    let mut conflict_set = gen_empty_configuration("v".to_string(), &states);

    let mut select_transitions_body = vec![];
    select_transitions_body.append(&mut configuration_destruct);
    select_transitions_body.append(&mut initialized_destruct);
    select_transitions_body.append(&mut history_destruct);
    select_transitions_body.append(&mut entry_set);
    select_transitions_body.append(&mut trans_set);
    select_transitions_body.append(&mut exit_set);
    select_transitions_body.append(&mut conflict_set);
    select_transitions_body.push(Statement::VariableDeclaration(VariableDeclaration {
        id: VariableDeclarationId::Identifier(is_stable_ident),
        init: Expression::BooleanLiteral(BooleanLiteral { value: false, loc }),
        loc,
    }));

    for transition in transitions {
        match transition.t {
            core::TransitionType::History | core::TransitionType::Initial => (),
            _ => {
                println!("{:?}", transition);
            }
        }
    }

    Function {
        params: vec![
            Expression::Identifier(configuration_ident),
            Expression::Identifier(initialized_ident),
            Expression::Identifier(history_ident),
        ],
        body: select_transitions_body,
        loc,
    }
}

fn gen_destruct(configuration: &Identifier, states: &Vec<core::State>) -> Vec<Statement> {
    (0..states.len())
        .map(|index| {
            let loc = states[index].loc;
            Statement::VariableDeclaration(VariableDeclaration {
                id: VariableDeclarationId::Identifier(Identifier {
                    name: format!("{}_{}", configuration.name, index),
                    loc,
                }),
                init: Expression::ConfigurationIndexExpression(ConfigurationIndexExpression {
                    configuration: SimpleExpression::Identifier(configuration.clone()),
                    index,
                    loc,
                }),
                loc,
            })
        })
        .collect()
}

fn gen_empty_configuration(name: String, states: &Vec<core::State>) -> Vec<Statement> {
    (0..states.len())
        .map(|index| {
            let loc = states[index].loc;
            Statement::VariableDeclaration(VariableDeclaration {
                id: VariableDeclarationId::Identifier(Identifier {
                    name: format!("{}_{}", name, index),
                    loc,
                }),
                init: Expression::BooleanLiteral(BooleanLiteral { value: false, loc }),
                loc,
            })
        })
        .collect()
}

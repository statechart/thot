use ast::core;
use ast::statechart;
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConversionError {
    pub message: String,
    pub fatal: bool,
    pub source: String,
    pub position: statechart::Position,
}

pub type Errors = Vec<ConversionError>;

impl Into<Result<core::Core, Errors>> for statechart::Statechart {
    fn into(self) -> Result<core::Core, Errors> {
        let mut errors = vec![];
        let mut states = vec![];
        let mut ancestors = vec![];
        let mut transitions = vec![];
        let mut targets = HashMap::new();
        let mut state_ids = HashMap::new();
        let mut binding = statechart::Binding::Late;

        for event in statechart::Node::Statechart(self).iter() {
            match event {
                statechart::IteratorEvent::Enter(node) => match node.as_ref() {
                    statechart::Node::Statechart(node) => {
                        let idx = states.len();
                        binding = node.binding;
                        states.push(core::State {
                            idx,
                            t: core::StateType::Compound,
                            ..Default::default()
                        });
                        ancestors.push(idx);
                    }
                    statechart::Node::State(node) => {
                        let idx = states.len();
                        let statechart::State { id, .. } = node;
                        states.push(core::State {
                            idx,
                            id: id.clone(),
                            t: core::StateType::Compound,
                            parent: *ancestors.last().unwrap(),
                            ancestors: ancestors.clone(),
                            ..Default::default()
                        });
                        ancestors.push(idx);
                    }
                    statechart::Node::Parallel(node) => {
                        let idx = states.len();
                        let statechart::Parallel { id, .. } = node;
                        states.push(core::State {
                            idx,
                            id: id.clone(),
                            t: core::StateType::Parallel,
                            parent: *ancestors.last().unwrap(),
                            ancestors: ancestors.clone(),
                            ..Default::default()
                        });
                        ancestors.push(idx);
                    }
                    statechart::Node::Transition(node) => {
                        let source = *ancestors.last().unwrap();
                        let idx = transitions.len();
                        states[source].transitions.push(idx);
                        targets.insert(idx, node.targets.clone());
                        let transition = core::Transition {
                            idx,
                            t: core::TransitionType::External,
                            source,
                            events: node.events.clone(),
                            condition: node.condition,
                            ..Default::default()
                        };
                        transitions.push(transition);
                    }
                    statechart::Node::OnEvent(node) => {
                        let source = *ancestors.last().unwrap();
                        let idx = transitions.len();
                        let mut events = node.events.clone();
                        if events.len() == 0 {
                            events.push("*".to_string());
                        }
                        states[source].transitions.push(idx);
                        let transition = core::Transition {
                            idx,
                            t: core::TransitionType::OnEvent,
                            source,
                            events,
                            condition: node.condition,
                            ..Default::default()
                        };
                        transitions.push(transition);
                    }
                    statechart::Node::Initial(_node) => {
                        let idx = states.len();
                        states.push(core::State {
                            idx,
                            t: core::StateType::Initial,
                            parent: *ancestors.last().unwrap(),
                            ancestors: ancestors.clone(),
                            ..Default::default()
                        });
                        ancestors.push(idx);
                    }
                    statechart::Node::Final(node) => {
                        let idx = states.len();
                        let statechart::Final { id, .. } = node;
                        states.push(core::State {
                            idx,
                            id: id.clone(),
                            t: core::StateType::Final,
                            parent: *ancestors.last().unwrap(),
                            ancestors: ancestors.clone(),
                            ..Default::default()
                        });
                        ancestors.push(idx);
                    }
                    statechart::Node::OnInit(node) => match binding {
                        statechart::Binding::Early => {
                            states[0].on_init.append(&mut node.children.clone());
                        }
                        statechart::Binding::Late => {
                            let &idx = ancestors.last().unwrap();
                            states[idx].on_init.append(&mut node.children.clone());
                        }
                    },
                    statechart::Node::OnEntry(node) => {
                        let &idx = ancestors.last().unwrap();
                        states[idx].on_enter.append(&mut node.children.clone());
                    }
                    statechart::Node::OnExit(node) => {
                        let &idx = ancestors.last().unwrap();
                        states[idx].on_exit.append(&mut node.children.clone());
                    }
                    statechart::Node::History(node) => {
                        let idx = states.len();
                        let statechart::History { id, .. } = node;
                        states.push(core::State {
                            idx,
                            id: id.clone(),
                            t: match node.t {
                                statechart::HistoryType::Shallow => core::StateType::HistoryShallow,
                                statechart::HistoryType::Deep => core::StateType::HistoryDeep,
                            },
                            parent: *ancestors.last().unwrap(),
                            ancestors: ancestors.clone(),
                            ..Default::default()
                        });
                        ancestors.push(idx);
                    }
                    statechart::Node::Invoke(node) => {
                        let &idx = ancestors.last().unwrap();
                        states[idx].invocations.push(node.id);
                    }
                },
                statechart::IteratorEvent::Exit(node) => match node.as_ref() {
                    statechart::Node::Statechart(_)
                    | statechart::Node::State(_)
                    | statechart::Node::Parallel(_)
                    | statechart::Node::Initial(_)
                    | statechart::Node::Final(_)
                    | statechart::Node::History(_) => {
                        let &idx = ancestors.last().unwrap();
                        ancestors.pop();

                        // add us to ancestor descendants
                        for &ancestor in ancestors.iter() {
                            states[ancestor].descendants.push(idx);
                        }

                        // add us to the parent state
                        if let Some(&parent) = ancestors.last() {
                            states[parent].children.push(idx);
                        }

                        // set the type to atomic if no children
                        if states[idx].t == core::StateType::Compound
                            && states[idx].children.len() == 0
                        {
                            states[idx].t = core::StateType::Atomic;
                        }

                        // compute initial children
                        match states[idx].t {
                            core::StateType::Parallel => {
                                states[idx].initial = states[idx].children.clone();
                            }
                            core::StateType::Compound => {
                                // TODO
                                states[idx].initial = vec![*states[idx].children.first().unwrap()];
                            }
                            core::StateType::HistoryShallow => {
                                // TODO filter history children
                                states[idx].initial = states[idx].children.clone();
                            }
                            core::StateType::HistoryDeep => {
                                // TODO filter history descendants
                                states[idx].initial = states[idx].descendants.clone();
                            }
                            _ => (),
                        }

                        // check aliases
                        if let Some(ref id_s) = states[idx].id {
                            if state_ids.contains_key(id_s) {
                                errors.push(ConversionError {
                                    message: format!("Duplicate target: {:?}", id_s),
                                    fatal: true,
                                    source: "statechart/ast/statechart/to_core".to_string(),
                                    position: Default::default(),
                                });
                            } else {
                                state_ids.insert(id_s.clone(), idx);
                            }
                        }
                    }
                    _ => (),
                },
            }
        }

        for (transition_id, state_targets) in targets {
            let mut transition = &mut transitions[transition_id];
            for state_target in state_targets {
                if let Some(idx) = state_ids.get(&state_target) {
                    transition.targets.push(*idx);
                } else {
                    errors.push(ConversionError {
                        message: format!("Missing target: {:?}", state_target),
                        fatal: true,
                        source: "statechart/ast/statechart/to_core".to_string(),
                        position: Default::default(),
                    });
                }
            }
            transition.exits = get_exit_set(&transition, &states);
            transition.exits.sort();
        }

        compute_conflicts(&mut transitions, &states);

        if errors.len() > 0 {
            Err(errors)
        } else {
            Ok(core::Core {
                states: states,
                transitions: transitions,
            })
        }
    }
}

fn compute_conflicts(transitions: &mut Vec<core::Transition>, states: &Vec<core::State>) {
    let cloned = transitions.clone();
    for mut transition in transitions {
        transition.conflicts = get_conflicts(&transition, &cloned, &states);
        transition.conflicts.sort();
    }
}

fn get_conflicts(
    transition: &core::Transition,
    transitions: &Vec<core::Transition>,
    states: &Vec<core::State>,
) -> Vec<core::TransitionId> {
    transitions
        .iter()
        .filter(|t2| t2.idx != transition.idx)
        .filter(|t2| has_conflict(transition, t2, states))
        .map(|t2| t2.idx)
        .collect()
}

fn has_conflict(t1: &core::Transition, t2: &core::Transition, states: &Vec<core::State>) -> bool {
    let s1 = get_transition_source(t1, states);
    let s2 = get_transition_source(t2, states);

    s1.idx == s2.idx || has_insersection(&t1.exits, &t2.exits) || s1.descendants.contains(&s2.idx)
        || s2.descendants.contains(&s1.idx) || false
}

fn has_insersection<V: PartialEq>(arr1: &Vec<V>, arr2: &Vec<V>) -> bool {
    arr1.iter().any(|v| arr2.contains(v))
}

fn get_exit_set<'a>(
    transition: &'a core::Transition,
    states: &'a Vec<core::State>,
) -> Vec<core::StateId> {
    let domain = get_transition_domain(transition, states);

    domain
        .descendants
        .iter()
        .map(|idx| *idx)
        .filter(|idx| match states[*idx].t {
            core::StateType::Atomic
            | core::StateType::Compound
            | core::StateType::Parallel
            | core::StateType::Final => true,
            _ => false,
        })
        .collect()
}

fn get_transition_domain<'a>(
    transition: &'a core::Transition,
    states: &'a Vec<core::State>,
) -> &'a core::State {
    let source = get_transition_source(transition, states);
    let targets = &transition.targets;

    if transition.t == core::TransitionType::Internal && source.t == core::StateType::Compound
        && are_descendants(&source.descendants, &targets)
    {
        return source;
    }

    find_lcca(states, &source, &targets)
}

fn find_lcca<'a>(
    states: &'a Vec<core::State>,
    source: &'a core::State,
    targets: &'a Vec<core::StateId>,
) -> &'a core::State {
    let mut self_and_targets = targets.clone();
    self_and_targets.push(source.idx);
    source
        .ancestors
        .iter()
        .map(|anc| &states[*anc])
        .filter(|state| match state.t {
            core::StateType::Atomic | core::StateType::Compound | core::StateType::Parallel => true,
            _ => false,
        })
        .find(|state| are_descendants(&state.descendants, &self_and_targets))
        .unwrap_or(source)
}

fn are_descendants(descendants: &Vec<core::StateId>, targets: &Vec<core::StateId>) -> bool {
    targets.iter().all(|target| descendants.contains(target))
}

fn get_transition_source<'a>(
    transition: &'a core::Transition,
    states: &'a Vec<core::State>,
) -> &'a core::State {
    let core::Transition { source, .. } = transition;
    let source = &states[*source];
    if source.t == core::StateType::Initial {
        &states[source.parent]
    } else {
        source
    }
}

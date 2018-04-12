pub type StateId = usize;
pub type TransitionId = usize;
pub type ExecutableId = usize;
pub type InvocationId = usize;
pub type ConditonId = usize;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Core {
    pub states: Vec<State>,
    pub transitions: Vec<Transition>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StateType {
    Atomic,
    Compound,
    Parallel,
    HistoryShallow,
    HistoryDeep,
    Initial,
    Final,
}

impl Default for StateType {
    fn default() -> StateType {
        StateType::Atomic
    }
}

// TODO add location information

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    #[serde(default)]
    pub id: Option<String>,

    #[serde(default)]
    pub idx: StateId,

    #[serde(rename = "type")]
    pub t: StateType,

    #[serde(default)]
    pub on_init: Vec<ExecutableId>,

    #[serde(default)]
    pub on_enter: Vec<ExecutableId>,

    #[serde(default)]
    pub on_exit: Vec<ExecutableId>,

    #[serde(default)]
    pub invocations: Vec<InvocationId>,

    #[serde(default)]
    pub parent: StateId,

    #[serde(default)]
    pub children: Vec<StateId>,

    #[serde(default)]
    pub ancestors: Vec<StateId>,

    #[serde(default)]
    pub descendants: Vec<StateId>,

    #[serde(default)]
    pub initial: Vec<StateId>,

    #[serde(default)]
    pub transitions: Vec<TransitionId>,

    #[serde(default)]
    pub has_history: bool,
}

impl State {
    pub fn new() -> State {
        State {
            id: None,
            idx: 0,
            t: StateType::Atomic,
            on_init: vec![],
            on_enter: vec![],
            on_exit: vec![],
            invocations: vec![],
            parent: 0,
            children: vec![],
            ancestors: vec![],
            descendants: vec![],
            initial: vec![],
            transitions: vec![],
            has_history: false,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TransitionType {
    External,
    Targetless,
    Internal,
    Spontaneous,
    History,
    Initial,
    OnEvent,
}

impl Default for TransitionType {
    fn default() -> TransitionType {
        TransitionType::External
    }
}

// TODO add location information

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Transition {
    #[serde(default)]
    pub idx: TransitionId,

    #[serde(rename = "type")]
    pub t: TransitionType,

    #[serde(default)]
    pub source: StateId,

    #[serde(default)]
    pub events: Vec<String>,

    #[serde(default)]
    pub condition: Option<ConditonId>,

    #[serde(default)]
    pub on_transition: Vec<ExecutableId>,

    #[serde(default)]
    pub targets: Vec<StateId>,

    #[serde(default)]
    pub conflicts: Vec<TransitionId>,

    #[serde(default)]
    pub exits: Vec<StateId>,
}

impl Transition {
    pub fn new() -> Transition {
        Transition {
            idx: 0,
            t: TransitionType::External,
            source: 0,
            events: vec![],
            condition: None,
            on_transition: vec![],
            targets: vec![],
            conflicts: vec![],
            exits: vec![],
        }
    }
}

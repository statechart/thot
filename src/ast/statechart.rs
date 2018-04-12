use ast::location::Location;

#[path = "statechart/to_core.rs"]
pub mod to_core;

pub type ExecutableId = usize;
pub type InvocationId = usize;
pub type ConditonId = usize;
pub type EventId = usize;

#[derive(Clone, Debug)]
pub enum IteratorEvent {
    Enter(Box<Node>),
    Exit(Box<Node>),
}

#[derive(Clone, Debug)]
pub struct ChildIterator {
    node: Option<Box<NodeIterator>>,
    children: Box<Vec<Node>>,
    i: usize,
}

impl Iterator for ChildIterator {
    type Item = IteratorEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut iter) = self.node {
            let next = iter.next();
            if let None = next {
                self.i += 1;
            } else {
                return next;
            }
        }

        if self.i >= self.children.len() {
            return None;
        }

        let child = Box::new(self.children[self.i].clone());

        let child_itr = Box::new(NodeIterator {
            node: child,
            children: None,
            is_finished: false,
        });

        self.node = Some(child_itr);

        self.next()
    }
}

#[derive(Clone, Debug)]
pub struct NodeIterator {
    node: Box<Node>,
    children: Option<Box<ChildIterator>>,
    is_finished: bool,
}

impl Iterator for NodeIterator {
    type Item = IteratorEvent;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_finished {
            return None;
        }

        if let Some(ref mut iter) = self.children {
            let next = iter.next();
            if let None = next {
                self.is_finished = true;
                return Some(IteratorEvent::Exit(self.node.clone()));
            }

            return next;
        }

        let children = match self.node.as_ref() {
            Node::Statechart(node) => Box::new(node.children.clone()),
            Node::State(node) => Box::new(node.children.clone()),
            Node::Parallel(node) => Box::new(node.children.clone()),
            Node::Initial(node) => Box::new(node.children.clone()),
            Node::Final(node) => Box::new(node.children.clone()),
            _ => Box::new(vec![]),
        };

        let child_iterator = Box::new(ChildIterator {
            node: None,
            children: children,
            i: 0,
        });

        self.children = Some(child_iterator);

        Some(IteratorEvent::Enter(self.node.clone()))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Node {
    Statechart(Statechart),
    State(State),
    Parallel(Parallel),
    Transition(Transition),
    OnEvent(OnEvent),
    Initial(Initial),
    Final(Final),
    OnInit(OnInit),
    OnEntry(OnEntry),
    OnExit(OnExit),
    History(History),
    Invoke(Invoke),
}

impl Node {
    pub fn iter(self) -> NodeIterator {
        NodeIterator {
            node: Box::from(self),
            children: None,
            is_finished: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Binding {
    Early,
    Late,
}

impl Default for Binding {
    fn default() -> Self {
        Binding::Late
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Statechart {
    #[serde(default)]
    pub initital: Vec<String>,

    #[serde(default)]
    pub binding: Binding,

    #[serde(default)]
    pub children: Vec<Node>,

    #[serde(default)]
    pub loc: Location,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    #[serde(default)]
    pub id: Option<String>,

    #[serde(default)]
    pub initial: Vec<String>,

    #[serde(default)]
    pub children: Vec<Node>,

    #[serde(default)]
    pub loc: Location,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Parallel {
    #[serde(default)]
    pub id: Option<String>,

    #[serde(default)]
    pub children: Vec<Node>,

    #[serde(default)]
    pub loc: Location,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum TransitionType {
    Internal,
    External,
}

impl Default for TransitionType {
    fn default() -> Self {
        TransitionType::External
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Transition {
    #[serde(default)]
    pub events: Vec<EventId>,

    #[serde(default)]
    pub targets: Vec<String>,

    #[serde(default)]
    pub t: TransitionType,

    #[serde(default)]
    pub condition: Option<ConditonId>,

    #[serde(default)]
    pub children: Vec<ExecutableId>,

    #[serde(default)]
    pub loc: Location,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OnEvent {
    #[serde(default)]
    pub events: Vec<EventId>,

    #[serde(default)]
    pub condition: Option<ConditonId>,

    #[serde(default)]
    pub children: Vec<ExecutableId>,

    #[serde(default)]
    pub loc: Location,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Initial {
    #[serde(default)]
    pub children: Vec<Node>,

    #[serde(default)]
    pub loc: Location,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Final {
    #[serde(default)]
    pub id: Option<String>,

    #[serde(default)]
    pub children: Vec<Node>,

    #[serde(default)]
    pub loc: Location,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OnInit {
    #[serde(default)]
    pub children: Vec<ExecutableId>,

    #[serde(default)]
    pub loc: Location,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OnEntry {
    #[serde(default)]
    pub children: Vec<ExecutableId>,

    #[serde(default)]
    pub loc: Location,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OnExit {
    #[serde(default)]
    pub children: Vec<ExecutableId>,

    #[serde(default)]
    pub loc: Location,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum HistoryType {
    Shallow,
    Deep,
}

impl Default for HistoryType {
    fn default() -> Self {
        HistoryType::Shallow
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct History {
    #[serde(default)]
    pub id: Option<String>,

    #[serde(default)]
    pub t: HistoryType,

    #[serde(default)]
    pub children: Vec<Node>,

    #[serde(default)]
    pub loc: Location,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Invoke {
    pub id: InvocationId,

    #[serde(default)]
    pub loc: Location,
}

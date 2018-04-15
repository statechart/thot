use ast::location::Location;
type ExecutableId = usize;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Microstep {
    pub configuration_size: usize,
    pub init: Function,
    pub next: Function,
    pub render: Function,
    // TODO add state id mapping
    pub loc: Location,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    AssignmentStatement(AssignmentStatement),
    ConfigurationDestructureDeclaration(ConfigurationDestructureDeclaration),
    ReturnStatement(ReturnStatement),
    ExecuteStatement(ExecuteStatement),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Expression {
    Identifier(Identifier),
    NullLiteral,
    StringLiteral(StringLiteral),
    BooleanLiteral(BooleanLiteral),
    IntegerLiteral(IntegerLiteral),
    LogicalExpression(LogicalExpression),
    ConfigurationCreateExpression(ConfigurationCreateExpression),
    ConditionExpression(ConditionExpression),
    EventExpression(EventExpression),
    MicrostepResult(MicrostepResult),
    RenderExpression, // TODO
}

impl Expression {
    pub fn to_simple(self: Expression) -> SimpleExpression {
        match self {
            Expression::Identifier(v) => SimpleExpression::Identifier(v),
            Expression::StringLiteral(v) => SimpleExpression::StringLiteral(v),
            Expression::BooleanLiteral(v) => SimpleExpression::BooleanLiteral(v),
            Expression::IntegerLiteral(v) => SimpleExpression::IntegerLiteral(v),
            Expression::LogicalExpression(v) => SimpleExpression::LogicalExpression(v),
            Expression::ConditionExpression(v) => SimpleExpression::ConditionExpression(v),
            Expression::EventExpression(v) => SimpleExpression::EventExpression(v),
            Expression::ConfigurationCreateExpression(v) => {
                SimpleExpression::ConfigurationCreateExpression(v)
            }
            _ => {
                panic!("Invalid conversion {:?}", self);
            }
        }
    }
}

impl Default for Expression {
    fn default() -> Expression {
        Expression::NullLiteral
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum SimpleExpression {
    Identifier(Identifier),
    NullLiteral,
    StringLiteral(StringLiteral),
    BooleanLiteral(BooleanLiteral),
    IntegerLiteral(IntegerLiteral),
    LogicalExpression(LogicalExpression),
    ConditionExpression(ConditionExpression),
    EventExpression(EventExpression),
    ConfigurationCreateExpression(ConfigurationCreateExpression),
}

impl Default for SimpleExpression {
    fn default() -> SimpleExpression {
        SimpleExpression::NullLiteral
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Identifier {
    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub loc: Location,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct StringLiteral {
    #[serde(default)]
    pub value: String,

    #[serde(default)]
    pub loc: Location,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct BooleanLiteral {
    #[serde(default)]
    pub value: bool,

    #[serde(default)]
    pub loc: Location,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct IntegerLiteral {
    #[serde(default)]
    pub value: usize,

    #[serde(default)]
    pub loc: Location,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LogicalExpression {
    pub operator: LogicalOperator,

    pub arguments: Vec<Expression>,

    #[serde(default)]
    pub loc: Location,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum LogicalOperator {
    And,
    Or,
    Not,
    Xor,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConditionExpression {
    #[serde(default)]
    pub id: usize,

    #[serde(default)]
    pub loc: Location,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EventExpression {
    #[serde(default)]
    pub id: usize,

    #[serde(default)]
    pub loc: Location,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MicrostepResult {
    #[serde(default)]
    pub configuration: SimpleExpression,

    #[serde(default)]
    pub initialized: SimpleExpression,

    #[serde(default)]
    pub history: SimpleExpression,

    #[serde(default)]
    pub is_stable: SimpleExpression,

    #[serde(default)]
    pub loc: Location,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConfigurationCreateExpression {
    #[serde(default)]
    pub arguments: Vec<Expression>,

    #[serde(default)]
    pub loc: Location,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Function {
    #[serde(default)]
    pub params: Vec<Expression>,

    #[serde(default)]
    pub body: Vec<Statement>,

    #[serde(default)]
    pub loc: Location,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct VariableDeclaration {
    pub id: VariableDeclarationId,

    #[serde(default)]
    pub init: Expression,

    #[serde(default)]
    pub loc: Location,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConfigurationDestructureDeclaration {
    pub left: Vec<Expression>,

    #[serde(default)]
    pub right: Expression,

    #[serde(default)]
    pub loc: Location,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum VariableDeclarationId {
    Identifier(Identifier),
}

impl Default for VariableDeclarationId {
    fn default() -> VariableDeclarationId {
        VariableDeclarationId::Identifier(Default::default())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AssignmentStatement {
    pub left: AssignmentStatementLeft,

    #[serde(default)]
    pub right: Expression,

    #[serde(default)]
    pub loc: Location,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum AssignmentStatementLeft {
    Identifier(Identifier),
}

impl Default for AssignmentStatementLeft {
    fn default() -> AssignmentStatementLeft {
        AssignmentStatementLeft::Identifier(Default::default())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ExecuteStatement {
    pub id: ExecutableId,

    #[serde(default)]
    pub guard: Option<Expression>,

    #[serde(default)]
    pub loc: Location,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ReturnStatement {
    #[serde(default)]
    pub argument: Expression,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guard: Option<Expression>,

    #[serde(default)]
    pub loc: Location,
}

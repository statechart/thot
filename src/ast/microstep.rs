use ast::location::Location;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Microstep {
    pub configuration_size: usize,
    pub init: Function,
    pub select_transitions: Function,
    pub render: Function,
    // TODO add state ids
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    AssignmentStatement(AssignmentStatement),
    ReturnStatement,
    InvokeStatement,
    UninvokeStatement,
    ExecuteStatement,
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
    ConfigurationIndexExpression(ConfigurationIndexExpression),
    ConfigurationCreateExpression,
    ConditionExpression,
    RenderExpression,
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
    ConditionExpression,
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
    Xor,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConfigurationIndexExpression {
    pub configuration: SimpleExpression,

    #[serde(default)]
    pub index: usize,

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
    pub left: Identifier,

    #[serde(default)]
    pub right: Expression,

    #[serde(default)]
    pub loc: Location,
}

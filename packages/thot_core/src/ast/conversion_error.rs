use ast::location::Location;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConversionError {
    pub message: String,
    pub fatal: bool,
    pub source: String,
    pub loc: Location,
}

pub type Errors = Vec<ConversionError>;

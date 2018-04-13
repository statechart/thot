#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Point {
    #[serde(default)]
    pub line: usize,

    #[serde(default)]
    pub column: usize,
}

impl Default for Point {
    fn default() -> Point {
        Point { line: 1, column: 0 }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Location {
    #[serde(default)]
    pub start: Point,

    #[serde(default)]
    pub end: Point,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<usize>,
}

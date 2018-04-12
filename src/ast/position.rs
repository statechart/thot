#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Point {
    pub line: u16,
    pub column: u16,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Location {
    #[serde(default)]
    pub start: Point,

    #[serde(default)]
    pub end: Point,
}

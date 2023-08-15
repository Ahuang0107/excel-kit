#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Fill {
    #[serde(rename = "patternFill")]
    pub pattern_fill: PatternFill,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct PatternFill {
    #[serde(rename = "@patternType")]
    pub pattern_type: String,
}

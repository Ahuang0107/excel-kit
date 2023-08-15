use crate::Value;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Font {
    #[serde(rename = "sz")]
    pub size: Value<usize>,
    pub color: Color,
    pub name:Value<String>,
    pub family: Value<usize>,
    pub scheme: Value<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Color {
    #[serde(rename = "@theme")]
    pub theme: usize,
}

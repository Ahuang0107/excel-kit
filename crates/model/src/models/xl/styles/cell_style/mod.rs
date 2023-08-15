#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CellStyle {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@xfId")]
    pub xf_id: usize,
    #[serde(rename = "@builtinId")]
    pub build_in_id: usize,
}

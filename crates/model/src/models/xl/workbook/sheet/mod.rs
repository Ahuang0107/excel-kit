#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Sheet {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@sheetId")]
    pub sheet_id: usize,
    #[serde(rename = "@r:id")]
    pub r_id: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct Xf {
    #[serde(rename = "@numFmtId")]
    pub num_fmt_id: usize,
    #[serde(rename = "@fontId")]
    pub font_id: usize,
    #[serde(rename = "@fillId")]
    pub fill_id: usize,
    #[serde(rename = "@borderId")]
    pub border_id: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xfId")]
    pub xf_id: Option<usize>,
}

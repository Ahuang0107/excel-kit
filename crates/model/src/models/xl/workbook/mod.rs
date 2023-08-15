mod sheet;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename = "workbook")]
pub struct Workbook {
    #[serde(rename = "@xmlns")]
    pub xmlns: String,
    #[serde(rename = "@xmlns:r")]
    pub xmlns_r: String,
    #[serde(rename = "@xmlns:mc")]
    pub xmlns_mc: String,
    #[serde(rename = "@mc:Ignorable")]
    pub mc_ignorable: String,
    #[serde(rename = "@xmlns:x15")]
    pub xmlns_x15: String,
    #[serde(rename = "@xmlns:xr")]
    pub xmlns_xr: String,
    #[serde(rename = "@xmlns:xr6")]
    pub xmlns_xr6: String,
    #[serde(rename = "@xmlns:xr10")]
    pub xmlns_xr10: String,
    #[serde(rename = "@xmlns:xr2")]
    pub xmlns_xr2: String,
    pub sheets: Sheets,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct Sheets {
    pub sheet: Vec<sheet::Sheet>,
}

impl crate::XmlWriter for Workbook {}

impl Default for Workbook {
    fn default() -> Self {
        Self {
            xmlns: "http://schemas.openxmlformats.org/spreadsheetml/2006/main".into(),
            xmlns_r: "http://schemas.openxmlformats.org/officeDocument/2006/relationships".into(),
            xmlns_mc: "http://schemas.openxmlformats.org/markup-compatibility/2006".into(),
            mc_ignorable: "x15 xr xr6 xr10 xr2".into(),
            xmlns_x15: "http://schemas.microsoft.com/office/spreadsheetml/2010/11/main".into(),
            xmlns_xr: "http://schemas.microsoft.com/office/spreadsheetml/2014/revision".into(),
            xmlns_xr6: "http://schemas.microsoft.com/office/spreadsheetml/2016/revision6".into(),
            xmlns_xr10: "http://schemas.microsoft.com/office/spreadsheetml/2016/revision10".into(),
            xmlns_xr2: "http://schemas.microsoft.com/office/spreadsheetml/2015/revision2".into(),
            sheets: Sheets::default(),
        }
    }
}

impl Workbook {
    pub fn new() -> Self {
        Self {
            sheets: Sheets {
                sheet: vec![sheet::Sheet {
                    name: "Sheet1".into(),
                    sheet_id: 1,
                    r_id: "rId1".into(),
                }],
            },
            ..Default::default()
        }
    }
}

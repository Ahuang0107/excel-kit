#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename = "Types")]
pub struct ContentTypes {
    #[serde(rename = "@xmlns")]
    pub xmlns: String,
    pub defaults: Vec<DefaultContent>,
    pub overrides: Vec<OverrideContent>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename = "Default")]
pub struct DefaultContent {
    #[serde(rename = "@Extension")]
    pub extension: String,
    #[serde(rename = "@ContentType")]
    pub content_type: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename = "Override")]
pub struct OverrideContent {
    #[serde(rename = "@PartName")]
    pub part_name: String,
    #[serde(rename = "@ContentType")]
    pub content_type: String,
}

impl Default for ContentTypes {
    fn default() -> Self {
        Self {
            xmlns: "http://schemas.openxmlformats.org/package/2006/content-types".into(),
            defaults: vec![
                DefaultContent {
                    extension: "rels".into(),
                    content_type: "application/vnd.openxmlformats-package.relationships+xml".into(),
                },
                DefaultContent {
                    extension: "xml".into(),
                    content_type: "application/xml".into(),
                },
            ],
            overrides: vec![
                OverrideContent {
                    part_name: "/xl/workbook.xml".into(),
                    content_type:
                    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"
                        .into(),
                },
                OverrideContent {
                    part_name: "/xl/worksheets/sheet1.xml".into(),
                    content_type:
                    "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"
                        .into(),
                },
                OverrideContent {
                    part_name: "/xl/theme/theme1.xml".into(),
                    content_type:
                    "application/vnd.openxmlformats-officedocument.theme+xml"
                        .into(),
                },
                OverrideContent {
                    part_name: "/xl/styles.xml".into(),
                    content_type:
                    "application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml"
                        .into(),
                },
                OverrideContent {
                    part_name: "/xl/sharedStrings.xml".into(),
                    content_type:
                    "application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml"
                        .into(),
                },
                OverrideContent {
                    part_name: "/docProps/core.xml".into(),
                    content_type:
                    "application/vnd.openxmlformats-package.core-properties+xml"
                        .into(),
                },
                OverrideContent {
                    part_name: "/docProps/app.xml".into(),
                    content_type:
                    "application/vnd.openxmlformats-officedocument.extended-properties+xml"
                        .into(),
                },
            ],
        }
    }
}

impl crate::XmlWriter for ContentTypes {}

pub struct DocProps {
    pub app: App,
    pub core: Core,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename = "Properties")]
pub struct App {
    #[serde(rename = "@xmlns")]
    pub xmlns: String,
    #[serde(rename = "@xmlns:vt")]
    pub xmlns_vt: String,
    pub properties: Vec<Properties>,
}

impl crate::XmlWriter for App {}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Properties {
    Application(String),
    DocSecurity(usize),
    ScaleCrop(bool),
    HeadingPairs {
        #[serde(rename = "$value")]
        filed: Vec<Vt>,
    },
    TitlesOfParts {
        #[serde(rename = "$value")]
        filed: Vec<Vt>,
    },
    Company,
    LinksUpToDate(bool),
    SharedDoc(bool),
    HyperlinksChanged(bool),
    AppVersion(f32),
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename = "vt:vector")]
pub struct VtVector {
    #[serde(rename = "@size")]
    pub size: usize,
    #[serde(rename = "@baseType")]
    pub base_type: String,
    #[serde(rename = "$value")]
    pub filed: Vec<Vt>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Vt {
    #[serde(rename = "vt:vector")]
    Vector {
        #[serde(rename = "@size")]
        size: usize,
        #[serde(rename = "@baseType")]
        base_type: String,
        #[serde(rename = "$value")]
        filed: Vec<Box<Vt>>,
    },
    #[serde(rename = "vt:variant")]
    Variant {
        #[serde(rename = "$value")]
        filed: Box<Vt>,
    },
    #[serde(rename = "vt:lpstr")]
    Lpstr(String),
    #[serde(rename = "vt:i4")]
    I4(usize),
}

impl Default for App {
    fn default() -> Self {
        Self {
            xmlns: "http://schemas.openxmlformats.org/officeDocument/2006/extended-properties"
                .into(),
            xmlns_vt: "http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes".into(),
            properties: vec![
                Properties::Application("Microsoft Excel".into()),
                Properties::DocSecurity(0),
                Properties::ScaleCrop(false),
                Properties::HeadingPairs {
                    filed: vec![Vt::Vector {
                        size: 2,
                        base_type: "variant".into(),
                        filed: vec![
                            Box::new(Vt::Variant {
                                filed: Box::new(Vt::Lpstr("Worksheets".into())),
                            }),
                            Box::new(Vt::Variant {
                                filed: Box::new(Vt::I4(1)),
                            }),
                        ],
                    }],
                },
                Properties::TitlesOfParts {
                    filed: vec![Vt::Vector {
                        size: 1,
                        base_type: "lpstr".into(),
                        filed: vec![Box::new(Vt::Lpstr("Sheet1".into()))],
                    }],
                },
                Properties::Company,
                Properties::LinksUpToDate(false),
                Properties::SharedDoc(false),
                Properties::HyperlinksChanged(false),
                Properties::AppVersion(16.0300),
            ],
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename = "cp:coreProperties")]
pub struct Core {
    #[serde(rename = "@xmlns:cp")]
    pub xmlns_cp: String,
    #[serde(rename = "@xmlns:dc")]
    pub xmlns_dc: String,
    #[serde(rename = "@xmlns:dcmitype")]
    pub xmlns_dcmitype: String,
    #[serde(rename = "@xmlns:dcterms")]
    pub xmlns_dcterms: String,
    #[serde(rename = "@xmlns:xsi")]
    pub xmlns_xsi: String,
    #[serde(rename = "$value")]
    pub filed: Vec<Dc>,
}

impl crate::XmlWriter for Core {}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Dc {
    #[serde(rename = "dc:creator")]
    Creator(String),
    #[serde(rename = "dcterms:created")]
    DctermsCreated {
        #[serde(rename = "@xsi:type")]
        xsi_type: String,
        #[serde(rename = "$text")]
        text: String,
    },
}

impl Default for Core {
    fn default() -> Self {
        Self {
            xmlns_cp: "http://schemas.openxmlformats.org/package/2006/metadata/core-properties"
                .into(),
            xmlns_dc: "http://purl.org/dc/elements/1.1/".into(),
            xmlns_dcterms: "http://purl.org/dc/terms/".into(),
            xmlns_dcmitype: "http://purl.org/dc/dcmitype/".into(),
            xmlns_xsi: "http://www.w3.org/2001/XMLSchema-instance".into(),
            filed: vec![
                Dc::Creator("elase huang".into()),
                Dc::DctermsCreated {
                    xsi_type: "dcterms:W3CDTF".into(),
                    text: "2015-06-05T18:17:20Z".into(),
                },
            ],
        }
    }
}

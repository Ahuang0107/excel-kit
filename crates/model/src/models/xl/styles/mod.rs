mod border;
mod cell_style;
mod fill;
mod font;
mod xf;
use crate::Value;
use border::*;
use cell_style::*;
use fill::*;
use font::*;
use xf::*;

#[serde(rename = "styleSheet")]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Styles {
    #[serde(rename = "@xmlns")]
    pub xmlns: String,
    #[serde(rename = "@xmlns:mc")]
    pub xmlns_mc: String,
    #[serde(rename = "@mc:Ignorable")]
    pub mc_ignorable: String,
    #[serde(rename = "@xmlns:x14ac")]
    pub xmlns_x14ac: String,
    #[serde(rename = "@xmlns:x16r2")]
    pub xmlns_x16r2: String,
    #[serde(rename = "@xmlns:xr")]
    pub xmlns_xr: String,
    pub fonts: Fonts,
    pub fills: Fills,
    pub borders: Borders,
    #[serde(rename = "cellStyleXfs")]
    pub cell_style_xfs: CellStyleXfs,
    #[serde(rename = "cellXfs")]
    pub cell_xfs: CellXfs,
    #[serde(rename = "cellStyles")]
    pub cell_styles: CellStyles,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Fonts {
    #[serde(rename = "@count")]
    pub count: usize,
    #[serde(rename = "@x14ac:knownFonts")]
    pub x14ac_known_fonts: usize,
    pub font: Vec<Font>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Fills {
    #[serde(rename = "@count")]
    pub count: usize,
    pub fill: Vec<Fill>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Borders {
    #[serde(rename = "@count")]
    pub count: usize,
    pub border: Vec<Border>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CellStyleXfs {
    #[serde(rename = "@count")]
    pub count: usize,
    pub xf: Vec<Xf>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CellXfs {
    #[serde(rename = "@count")]
    pub count: usize,
    pub xf: Vec<Xf>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct CellStyles {
    #[serde(rename = "@count")]
    pub count: usize,
    #[serde(rename = "cellStyle")]
    pub cell_style: Vec<CellStyle>,
}

impl Default for Styles {
    fn default() -> Self {
        Self {
            xmlns: "http://schemas.openxmlformats.org/spreadsheetml/2006/main".into(),
            xmlns_mc: "http://schemas.openxmlformats.org/markup-compatibility/2006".into(),
            mc_ignorable: "x14ac x16r2 xr".into(),
            xmlns_x14ac: "http://schemas.microsoft.com/office/spreadsheetml/2009/9/ac".into(),
            xmlns_x16r2: "http://schemas.microsoft.com/office/spreadsheetml/2015/02/main".into(),
            xmlns_xr: "http://schemas.microsoft.com/office/spreadsheetml/2014/revision".into(),
            fonts: Fonts {
                count: 1,
                x14ac_known_fonts: 1,
                font: vec![Font {
                    size: Value { val: 11 },
                    color: Color { theme: 1 },
                    name: Value {
                        val: "Calibri".into(),
                    },
                    family: Value { val: 2 },
                    scheme: Value {
                        val: "minor".into(),
                    },
                }],
            },
            fills: Fills {
                count: 2,
                fill: vec![
                    Fill {
                        pattern_fill: PatternFill {
                            pattern_type: "none".into(),
                        },
                    },
                    Fill {
                        pattern_fill: PatternFill {
                            pattern_type: "gray125".into(),
                        },
                    },
                ],
            },
            borders: Borders {
                count: 1,
                border: vec![Border {}],
            },
            cell_style_xfs: CellStyleXfs {
                count: 1,
                xf: vec![Xf::default()],
            },
            cell_xfs: CellXfs {
                count: 1,
                xf: vec![Xf {
                    xf_id: Some(0),
                    ..Default::default()
                }],
            },
            cell_styles: CellStyles {
                count: 1,
                cell_style: vec![CellStyle {
                    name: "Normal".into(),
                    xf_id: 0,
                    build_in_id: 0,
                }],
            },
        }
    }
}

impl crate::XmlWriter for Styles {}

mod models;

pub use models::*;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Value<T: serde::Serialize> {
    #[serde(rename = "@val")]
    pub val: T,
}

pub trait XmlWriter: serde::Serialize {
    fn output(&self) -> String {
        const METADATA: &'static str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#;
        METADATA.to_string() + &quick_xml::se::to_string(&self).unwrap()
    }
}

pub trait Output {
    fn output(&self) -> String;
    fn to_xml(&self) -> String {
        const METADATA: &'static str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#;
        METADATA.to_string() + "\n" + &self.output()
    }
}

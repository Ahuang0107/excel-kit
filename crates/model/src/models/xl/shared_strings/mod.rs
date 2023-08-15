pub struct SharedStrings {}

impl crate::Output for SharedStrings {
    fn output(&self) -> String {
        r#"<sst xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" count="0" uniqueCount="0"></sst>"#.into()
    }
}

pub struct Rels {}

impl crate::Output for Rels {
    fn output(&self) -> String {
        r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
    <Relationship Id="rId3"
                  Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties"
                  Target="docProps/app.xml"/>
    <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties"
                  Target="docProps/core.xml"/>
    <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument"
                  Target="xl/workbook.xml"/>
</Relationships>"#.into()
    }
}

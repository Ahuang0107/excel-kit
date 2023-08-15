pub struct Workbook {}

impl crate::Output for Workbook {
    fn output(&self) -> String {
        r#"<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"
          xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
          xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" mc:Ignorable="x15 xr xr6 xr10 xr2"
          xmlns:x15="http://schemas.microsoft.com/office/spreadsheetml/2010/11/main"
          xmlns:xr="http://schemas.microsoft.com/office/spreadsheetml/2014/revision"
          xmlns:xr6="http://schemas.microsoft.com/office/spreadsheetml/2016/revision6"
          xmlns:xr10="http://schemas.microsoft.com/office/spreadsheetml/2016/revision10"
          xmlns:xr2="http://schemas.microsoft.com/office/spreadsheetml/2015/revision2">
    <fileVersion appName="xl" lastEdited="6" lowestEdited="6" rupBuild="14420"/>
    <workbookPr defaultThemeVersion="164011"/>
    <bookViews>
        <workbookView xWindow="0" yWindow="0" windowWidth="22260" windowHeight="12645"/>
    </bookViews>
    <sheets>
        <sheet name="Sheet1" sheetId="1" r:id="rId1"/>
    </sheets>
    <calcPr calcId="162913"/>
    <extLst>
        <ext uri="{140A7094-0E35-4892-8432-C4D2E57EDEB5}"
             xmlns:x15="http://schemas.microsoft.com/office/spreadsheetml/2010/11/main">
            <x15:workbookPr chartTrackingRefBase="1"/>
        </ext>
    </extLst>
</workbook>"#.into()
    }
}

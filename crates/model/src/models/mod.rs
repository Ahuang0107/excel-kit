use std::io::Write;
use std::path::Path;

use crate::{Output, XmlWriter};

mod content_types;
mod doc_props;
mod rels;
mod xl;

pub struct Excel {
    pub rels: rels::Rels,
    pub doc_props: doc_props::DocProps,
    pub xl: xl::Xl,
    pub content_types: content_types::ContentTypes,
}

impl Excel {
    pub fn new() -> Self {
        Self {
            rels: rels::Rels {},
            doc_props: doc_props::DocProps {
                app: doc_props::App::default(),
                core: doc_props::Core::default(),
            },
            xl: xl::Xl {
                workbook: xl::Workbook::new(),
                styles: xl::Styles::default(),
                shared_strings: xl::SharedStrings {},
                worksheets: vec![xl::Worksheet {}],
                themes: vec![xl::Theme {}],
                rels: xl::WorkbookRels {},
            },
            content_types: content_types::ContentTypes::default(),
        }
    }
    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let writer = std::fs::File::create(path)?;
        let mut zip = zip::ZipWriter::new(writer);
        let options = zip::write::FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o755);
        zip.start_file("[Content_Types].xml", options)?;
        zip.write_all(self.content_types.output().as_bytes())?;

        zip.add_directory("_rels", options)?;
        zip.add_directory("docProps", options)?;
        zip.add_directory("xl", options)?;
        zip.add_directory("xl/_rels", options)?;
        zip.add_directory("xl/theme", options)?;
        zip.add_directory("xl/worksheets", options)?;

        zip.start_file("_rels/.rels", options)?;
        zip.write_all(self.rels.to_xml().as_bytes())?;

        zip.start_file("docProps/app.xml", options)?;
        zip.write_all(self.doc_props.app.output().as_bytes())?;
        zip.start_file("docProps/core.xml", options)?;
        zip.write_all(self.doc_props.core.output().as_bytes())?;

        zip.start_file("xl/workbook.xml", options)?;
        zip.write_all(self.xl.workbook.output().as_bytes())?;
        zip.start_file("xl/styles.xml", options)?;
        zip.write_all(self.xl.styles.output().as_bytes())?;
        zip.start_file("xl/sharedStrings.xml", options)?;
        zip.write_all(self.xl.shared_strings.to_xml().as_bytes())?;
        for worksheet in self.xl.worksheets.iter() {
            zip.start_file("xl/worksheets/sheet1.xml", options)?;
            zip.write_all(worksheet.to_xml().as_bytes())?;
        }
        for theme in self.xl.themes.iter() {
            zip.start_file("xl/theme/theme1.xml", options)?;
            zip.write_all(theme.to_xml().as_bytes())?;
        }
        zip.start_file("xl/_rels/workbook.xml.rels", options)?;
        zip.write_all(self.xl.rels.to_xml().as_bytes())?;

        zip.finish()?;
        Ok(())
    }
}

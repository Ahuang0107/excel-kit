pub use rels::*;
pub use shared_strings::*;
pub use styles::*;
pub use theme::*;
pub use workbook::*;
pub use worksheet::*;

mod rels;
mod shared_strings;
mod styles;
mod theme;
mod workbook;
mod worksheet;

pub struct Xl {
    pub workbook: Workbook,
    pub styles: Styles,
    pub shared_strings: SharedStrings,
    pub worksheets: Vec<Worksheet>,
    pub themes: Vec<Theme>,
    pub rels: WorkbookRels,
}

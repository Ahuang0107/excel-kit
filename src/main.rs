use model::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let excel = Excel::new();
    // println!("{}", excel.xl.workbook.output());
    excel.write("outputs/output.xlsx")?;
    Ok(())
}

use model::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let excel = Excel::new();
    excel.write("outputs/output.xlsx")?;
    Ok(())
}

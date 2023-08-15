use std::error::Error;

use model::*;

fn main() -> Result<(), Box<dyn Error>> {
    let excel = Excel::new();
    excel.write("outputs/output.xlsx")?;
    Ok(())
}

use ts_rs::TS;

#[derive(Serialize, Debug, TS)]
pub struct Me {
    pub name: String,
}

pub fn export(output_filename: &str) -> Result<(), std::io::Error> {
    std::fs::remove_file(output_filename)?;

    // put all structs here
    Me::dump(output_filename)?;

    Ok(())
}

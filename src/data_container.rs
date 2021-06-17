use std::fs;
use std::io::Write;
use std::path::Path;

use crate::traits::ToCSV;

pub struct DataContainer<'a> {
    data_path: &'a Path,
}

impl<'a> DataContainer<'a> {
    pub fn new(data_path: &'a Path) -> Self {
        Self { data_path }
    }
    pub fn save(&self, csv_data: &impl ToCSV) -> std::io::Result<()> {
        let mut f = match self.data_path.exists() {
            true => fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(self.data_path)?,
            false => fs::File::create(self.data_path)?,
        };
        f.write_all(csv_data.to_csv().as_bytes())?;
        f.flush()?;
        Ok(())
    }
}

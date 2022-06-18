use std::fs::File;
use zip::read::ZipArchive;

use crate::utils::get_input_output_path;

pub fn open_zip(path: &str, out_dir: Option<&str>) -> Result<(), std::io::Error> {
    let (input_path, output_path) = get_input_output_path(path, out_dir)?;

    let zip_file = File::open(input_path)?;

    let mut zipArchive = ZipArchive::new(zip_file)?;

    zipArchive.extract(output_path);

    Ok(())
}

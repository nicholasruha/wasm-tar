use crate::utils::get_input_output_path;
use flate2::read::GzDecoder;
use std::fs::File;
use tar::Archive;

pub fn open_tar(path: &str, out_dir: Option<&str>) -> Result<(), std::io::Error> {
    let (input_path, output_path) = get_input_output_path(path, out_dir)?;

    let tar_gz = File::open(input_path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);

    archive.unpack(output_path)?;
    Ok(())
}

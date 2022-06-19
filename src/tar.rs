use crate::utils::get_input_output_path;
use flate2::read::GzDecoder;
use std::fs::{self, File};
use tar::{Archive, Builder};

pub fn open_tar(path: &str, out_dir: Option<&str>) -> Result<(), std::io::Error> {
    let (input_path, output_path) = get_input_output_path(path, out_dir)?;

    let tar_gz = File::open(input_path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);

    archive.unpack(output_path)?;
    Ok(())
}

pub fn write_tar(buffer: &mut [u8], out_dir: &str) {
    let builder = Builder::new(buffer);
    let data = builder.into_inner().unwrap();
    fs::write(&out_dir, &data).expect("Unable to write file");
}

use std::fs::File;
use std::io::Write;
use zip::{read::ZipArchive, result::ZipResult};

use crate::utils::get_input_output_path;

pub fn open_zip(path: &str, out_dir: Option<&str>) -> Result<(), std::io::Error> {
    let (input_path, output_path) = get_input_output_path(path, out_dir)?;

    let zip_file = File::open(input_path)?;

    let mut zip_archive = ZipArchive::new(zip_file)?;

    zip_archive.extract(output_path)?;

    Ok(())
}

pub fn write_zip(data: &mut [u8], out_dir: &str) -> ZipResult<()> {
    let mut buf: &mut [u8] = &mut [0u8; 65536];

    let mut cursor = std::io::Cursor::new(buf);
    let mut zip_writer = zip::ZipWriter::new(cursor);

    let options =
        zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
    zip_writer.start_file("hello_world.txt", options)?;
    zip_writer.write(data)?;

    // Optionally finish the zip. (this is also done on drop)
    zip_writer.finish()?;

    Ok(())
}

mod tar;
mod tests;
mod utils;
mod zip;

use crate::tar::*;
use crate::zip::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = openTar)]
pub fn open_tar_directory(path: &str, out_dir: &str) {
    open_tar(path, Some(out_dir)).unwrap();
}

#[wasm_bindgen(js_name = openZip)]
pub fn open_zip_directory(path: &str, out_dir: &str) {
    open_zip(path, Some(out_dir)).unwrap();
}

#[wasm_bindgen(js_name = writeTar)]
pub fn write_tar_data(data: &mut [u8], out_dir: &str) {
    write_tar(data, out_dir);
}
#[wasm_bindgen(js_name = writeZip)]
pub fn write_zip_data(data: &mut [u8], out_dir: &str) {
    write_zip(data, out_dir).unwrap();
}

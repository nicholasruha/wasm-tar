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
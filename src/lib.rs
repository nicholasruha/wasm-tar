mod tar;
mod tests;
mod utils;
mod zip;

use crate::tar::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = openTar)]
pub fn get_state(path: &str, out_dir: &str) {
    open_tar(path, Some(out_dir));
}

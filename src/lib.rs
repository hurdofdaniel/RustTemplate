mod utils;

use lazy_static::lazy_static;
use vfs::{MemoryFS, VfsPath, VfsResult};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

lazy_static! {
    static ref FS: VfsPath = MemoryFS::new().into();
}

#[wasm_bindgen]
pub fn list_files() -> Vec<js_sys::JsString> {
    let mut files = FS
        .walk_dir()
        .unwrap()
        .collect::<VfsResult<Vec<_>>>()
        .unwrap();
    files.sort_by_key(|path| path.as_str().to_string());

    return files
        .to_vec()
        .iter()
        .map(|path| js_sys::JsString::from(path.as_str().to_string()))
        .collect();
}

#[wasm_bindgen]
pub fn write_file(name: &str, content: &[u8]) {
    let path = FS.join(name).unwrap();

    path.create_file().unwrap().write_all(content).unwrap();
}

#[wasm_bindgen]
pub fn read_file(name: &str) -> Vec<u8> {
    let path = FS.join(name).unwrap();

    let mut content: Vec<u8> = vec![];
    path.open_file().unwrap().read_to_end(&mut content).unwrap();
    return content;
}

#[wasm_bindgen]
pub fn delete_file(name: &str) {
    let path = FS.join(name).unwrap();

    path.remove_file().unwrap();
}

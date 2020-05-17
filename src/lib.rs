#[macro_use]
extern crate serde;

use fluidlite;
use fluidlite::FileApi;
use js_sys::*;
use serde::Serialize;
use serde_wasm_bindgen;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::*;
use std::io::SeekFrom;
use std::path::Path;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Default, Debug)]
struct FluidWebFileLoader {}

impl FileApi for FluidWebFileLoader {
    type File = Vec<u16>;

    fn open(&mut self, filename: &Path) -> Option<Self::File> {
        todo!()
    }

    fn read(file: &mut Self::File, buf: &mut [u8]) -> bool {
        todo!()
    }

    fn seek(file: &mut Self::File, pos: SeekFrom) -> bool {
        todo!()
    }

    fn tell(file: &mut Self::File) -> Option<u64> {
        todo!()
    }
}

#[derive(Default)]
struct FluidWebState {
    pub files: HashMap<String, Vec<u16>>,
}

impl FluidWebState {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[wasm_bindgen]
pub struct FluidWeb {
    state: FluidWebState,
}

#[wasm_bindgen]
impl FluidWeb {
    #[wasm_bindgen(constructor)]
    pub fn new() -> FluidWeb {
        console_error_panic_hook::set_once();

        let state = FluidWebState::new();

        FluidWeb { state }
    }

    pub fn add_sf2_file_from_array_buffer(
        &mut self,
        name: &str,
        buf: &ArrayBuffer,
    ) -> Result<(), JsValue> {
        let u16_buf = Uint16Array::new(buf).to_vec();
        self.state.files.insert(name.to_string(), u16_buf);

        Ok(())
    }

    pub fn list_sf2_files(&mut self) -> JsValue {
        let mut keys: Vec<String> = vec![];

        for key in self.state.files.keys() {
            keys.push(key.to_string());
        }

        JsValue::from_serde(&keys).unwrap()
    }
}

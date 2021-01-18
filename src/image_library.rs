use super::*;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

log_rule!();

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageData {
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub pixels: Vec<u8>,
}

impl ImageData {
    pub fn new(name: String, width: i32, height: i32, pixels: Vec<u8>) -> Self {
        ImageData {
            name,
            width,
            height,
            pixels,
        }
    }

    pub fn clone(&self) -> ImageData {
        ImageData::new(
            self.name.clone(),
            self.width,
            self.height,
            self.pixels.clone(),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageLibrary {
    content: HashMap<i32, ImageData>,
    pub export_items: Vec<(i32, String)>,
}

impl ImageLibrary {
    pub fn new() -> Self {
        ImageLibrary {
            content: HashMap::new(),
            export_items: vec![],
        }
    }

    pub fn add_image(&mut self, name: String, width: i32, height: i32, data: Vec<u8>) -> i32 {
        let new_image_data: ImageData = ImageData::new(name.clone(), width, height, data);
        let id = self.content.len() as i32;
        let img = self
            .content
            .entry(id)
            .or_insert(ImageData::new("".to_string(), 0, 0, vec![]));
        *img = new_image_data;
        return id;
    }

    pub fn get_content(&self) -> &HashMap<i32, ImageData> {
        &self.content
    }

    pub fn get_image_data(&self, image_id: i32) -> Option<&ImageData> {
        self.content.get(&image_id).clone()
    }

    pub fn reset(&mut self) {
        self.content.clear();
        self.export_items.clear();
    }

    pub fn add_export_names(&mut self, output_image_names: &HashMap<String, (usize, bool)>) {
        for (name, info) in output_image_names.iter() {
            if info.1 {
                self.export_items.push((info.0 as i32, name.clone()));
            }
        }
    }

    pub fn export(&mut self) -> JsValue {
        let mut result: HashMap<String, ImageData> = HashMap::new();
        for (id, name) in self.export_items.iter() {
            if let Some(data) = self.content.get_mut(id) {
                let mut new_data = data.clone();
                new_data.name = name.clone();
                result.insert(name.clone(), new_data);
            }
        }
        JsValue::from_serde(&result).unwrap()
    }
}

lazy_static! {
    #[wasm_bindgen]
    pub static ref IMAGE_LIBRARY: Mutex<ImageLibrary> = Mutex::new(ImageLibrary::new());
}

#[wasm_bindgen]
pub fn library_add_image(name: String, width: i32, height: i32, data: Vec<u8>) {
    IMAGE_LIBRARY
        .lock()
        .unwrap()
        .add_image(name, width, height, data);
}

#[wasm_bindgen]
pub fn library_export() -> JsValue {
    IMAGE_LIBRARY.lock().unwrap().export()
}

#[wasm_bindgen]
pub fn library_reset() {
    IMAGE_LIBRARY.lock().unwrap().reset();
}

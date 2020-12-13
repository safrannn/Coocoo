use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

cfg_if::cfg_if! {
    if #[cfg(test)] {
        pub fn log(s: &str) {
            println!("{:?}", s);
        }
    } else {
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace = console)]
            fn log(s: &str);
            #[wasm_bindgen(js_namespace = console, js_name = log)]
            fn log_u32(a: u32);
            #[wasm_bindgen(js_namespace = console, js_name = log)]
            fn log_many(a: &str, b: &str);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageData {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
}

impl ImageData {
    pub fn new(width: usize, height: usize, pixels: Vec<u8>) -> Self {
        ImageData {
            width,
            height,
            pixels,
        }
    }

    pub fn clone(&self) -> ImageData {
        ImageData::new(self.width, self.height, self.pixels.clone())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageLibrary {
    content: HashMap<String, ImageData>,
    id_to_name: HashMap<i32, String>,
    generates: HashSet<String>,
}

impl ImageLibrary {
    pub fn new() -> Self {
        ImageLibrary {
            content: HashMap::new(),
            id_to_name: HashMap::new(),
            generates: HashSet::new(),
        }
    }

    pub fn add_image(
        &mut self,
        name: String,
        width: usize,
        height: usize,
        data: Vec<u8>,
        generates: bool,
    ) {
        let new_image_data: ImageData = ImageData::new(width, height, data);
        let img = self
            .content
            .entry(name.clone())
            .or_insert(ImageData::new(0, 0, vec![]));
        *img = new_image_data;

        if generates {
            self.generates.insert(name.to_string());
        }
    }

    pub fn rename_image(&mut self, name_old: String, name_new: String) -> String {
        if let Some(data) = self.content.remove(&name_old) {
            if self.content.contains_key(&name_new) {
                return "please enter another name for this image".to_string();
            } else {
                self.content.insert(name_new, data);
            }
        }
        return "".to_string();
    }

    pub fn delete_image(&mut self, name: String) {
        self.content.remove(&name);
    }

    pub fn image_exist(&self, name: String) -> bool {
        self.content.contains_key(&name)
    }

    pub fn get_content(&self) -> &HashMap<String, ImageData> {
        &self.content
    }

    pub fn get_image_data(&self, image_id: i32) -> Option<&ImageData> {
        let mut image_data: Option<&ImageData> = None;
        if let Some(name) = self.find_name(image_id) {
            if let Some(d) = self.content.get(name) {
                image_data = Some(d);
            }
        } else {
            let name = image_id.to_string();
            if let Some(d) = self.content.get(&name) {
                image_data = Some(d);
            }
        }
        image_data
    }

    pub fn image_names(&mut self) -> String {
        let mut names: String = "".to_string();
        self.id_to_name.clear();
        let mut i = 0;
        for (name, _) in self.content.iter() {
            names += &(name.clone() + ";");
            self.id_to_name.insert(i, name.clone());
            i += 1;
        }
        names
    }

    pub fn find_name(&self, image_id: i32) -> Option<&String> {
        self.id_to_name.get(&image_id)
    }

    pub fn reset(&mut self) {
        for name in self.generates.iter() {
            self.content.remove(&name.clone());
        }
    }

    pub fn export(&self) -> JsValue {
        let mut result: HashMap<String, &ImageData> = HashMap::new();
        for name in self.generates.iter() {
            if let Some(image_data) = self.content.get(&name.clone()) {
                result.insert(name.clone(), image_data);
            }
        }
        JsValue::from_serde(&result).unwrap()
    }
}

lazy_static! {
    #[wasm_bindgen]
    static ref IMAGE_LIBRARY: Mutex<ImageLibrary> = Mutex::new(ImageLibrary::new());
}

#[wasm_bindgen]
pub fn add_image_bindgen(name: String, width: usize, height: usize, data: Vec<u8>) {
    IMAGE_LIBRARY
        .lock()
        .unwrap()
        .add_image(name, width, height, data, false);
}

#[wasm_bindgen]
pub fn rename_image_bindgen(name_old: String, name_new: String) -> String {
    IMAGE_LIBRARY
        .lock()
        .unwrap()
        .rename_image(name_old, name_new)
}

#[wasm_bindgen]
pub fn delete_image_bindgen(name: String) {
    IMAGE_LIBRARY.lock().unwrap().delete_image(name);
}

#[wasm_bindgen]
pub fn image_exist_bindgen(name: String) {
    IMAGE_LIBRARY.lock().unwrap().image_exist(name);
}

#[wasm_bindgen]
pub fn image_names_bindgen() -> JsValue {
    let names: String = IMAGE_LIBRARY.lock().unwrap().image_names();
    JsValue::from_serde(&names).unwrap()
}

#[wasm_bindgen]
pub fn export_bindgen() -> JsValue {
    IMAGE_LIBRARY.lock().unwrap().export()
}

#[wasm_bindgen]
pub fn reset_bindgen() {
    IMAGE_LIBRARY.lock().unwrap().reset();
}

//============image filter functions below this line============
#[wasm_bindgen]
pub fn darken(image_id: i32, value: i32) -> i32 {
    let mut result_image_data = IMAGE_LIBRARY
        .lock()
        .unwrap()
        .get_image_data(image_id)
        .unwrap()
        .clone();
    for i in 0..result_image_data.pixels.len() {
        if result_image_data.pixels[i] >= value as u8 {
            result_image_data.pixels[i] -= value as u8;
        } else {
            result_image_data.pixels[i] = 0;
        }
    }

    let result_image_id = IMAGE_LIBRARY.lock().unwrap().get_content().len() as i32 + 1;
    IMAGE_LIBRARY.lock().unwrap().add_image(
        result_image_id.to_string(),
        result_image_data.width,
        result_image_data.height,
        result_image_data.pixels,
        true,
    );

    result_image_id
}

#[wasm_bindgen]
pub fn blank_image(width: i32, height: i32) -> i32 {
    let pixel_total: usize = (width * height * 3) as usize;
    let mut result_image_data: Vec<u8> = vec![0; pixel_total];
    for i in 0..pixel_total {
        result_image_data[i] = 100;
    }

    let result_image_id = IMAGE_LIBRARY.lock().unwrap().get_content().len() as i32 + 1;
    IMAGE_LIBRARY.lock().unwrap().add_image(
        result_image_id.to_string(),
        width as usize,
        height as usize,
        result_image_data,
        true,
    );

    result_image_id
}

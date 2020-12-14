use super::image_library::IMAGE_LIBRARY;
use wasm_bindgen::prelude::*;

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

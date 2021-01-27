use super::image_library::IMAGE_LIBRARY;
use super::*;
use wasm_bindgen::prelude::*;

log_rule!();

#[wasm_bindgen]
pub fn darken(image_id: i32, value: i32) -> i32 {
    log(&format!(
        "paramter image_id:{:?}, value:{:?}",
        image_id, value
    ));
    let mut result_image_data = IMAGE_LIBRARY
        .lock()
        .unwrap()
        .get_image_data(image_id)
        .unwrap()
        .clone();
    for i in 0..result_image_data.pixels.len() {
        if (i + 1) / 4 == 0 {
            continue;
        } else if result_image_data.pixels[i] > value as u8 {
            result_image_data.pixels[i] -= value as u8;
        } else {
            result_image_data.pixels[i] = 0;
        }
    }
    IMAGE_LIBRARY.lock().unwrap().add_image(
        "".to_string(),
        result_image_data.width,
        result_image_data.height,
        result_image_data.pixels,
    )
}

#[wasm_bindgen]
pub fn blank_image(width: i32, height: i32) -> i32 {
    let pixel_total: usize = (width * height * 4) as usize;
    let mut result_image_data: Vec<u8> = vec![0; pixel_total];
    for i in 0..pixel_total {
        if (i + 1) % 4 == 0 {
            result_image_data[i] = 255;
        } else {
            result_image_data[i] = 255;
        }
    }

    IMAGE_LIBRARY
        .lock()
        .unwrap()
        .add_image("".to_string(), width, height, result_image_data)
}

#[wasm_bindgen]
pub fn grayscale(image_id: i32) -> i32 {
    log(&format!("paramter image_id:{:?}", image_id));
    let mut result_image_data = IMAGE_LIBRARY
        .lock()
        .unwrap()
        .get_image_data(image_id)
        .unwrap()
        .clone();
    for i in (0..result_image_data.pixels.len()).step_by(4) {
        let avg = ((result_image_data.pixels[i] as f32 * 0.2989) as i32
            + (result_image_data.pixels[i + 1] as f32 * 0.5870) as i32
            + (result_image_data.pixels[i + 2] as f32 * 0.1140) as i32) as u8;
        result_image_data.pixels[i] = avg;
        result_image_data.pixels[i + 1] = avg;
        result_image_data.pixels[i + 2] = avg;
    }

    IMAGE_LIBRARY.lock().unwrap().add_image(
        "".to_string(),
        result_image_data.width,
        result_image_data.height,
        result_image_data.pixels,
    )
}

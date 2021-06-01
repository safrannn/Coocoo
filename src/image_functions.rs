use super::image_library::IMAGE_LIBRARY;
use super::log_rule;
use wasm_bindgen::prelude::*;

use ::noise::*;
use photon_rs::*;
use resize::Pixel::RGBA8;
use resize::Type::{Lanczos3, Mitchell};
use rgb::FromSlice;
use rgb::RGBA8;

log_rule!();

#[wasm_bindgen]
pub fn resize_(image_id: i32, new_width: i32, new_height: i32) -> i32 {
    let image_data = IMAGE_LIBRARY
        .lock()
        .unwrap()
        .get_image_data(image_id)
        .unwrap()
        .clone();

    let w1 = image_data.width as usize;
    let h1 = image_data.height as usize;
    let w2 = new_width as usize;
    let h2 = new_height as usize;

    if w1 == w2 && h1 == h2 {
        return image_id;
    }

    let src = image_data.pixels;
    let mut dst = vec![0; w2 * h2 * 4];

    if w1 < w2 || h1 < h2 {
        // Mitchell for upscaling
        if resize::new(w1, h1, w2, h2, RGBA8, Mitchell).is_ok() {
            let mut resizer = resize::new(w1, h1, w2, h2, RGBA8, Mitchell).unwrap();
            resizer.resize(src.as_rgba(), dst.as_rgba_mut()).unwrap();

            IMAGE_LIBRARY
                .lock()
                .unwrap()
                .add_image("".to_string(), new_width, new_height, dst)
        } else {
            return -1;
        }
    } else {
        // Lanczos3 for downscaling
        if resize::new(w1, h1, w2, h2, RGBA8, Lanczos3).is_ok() {
            let mut resizer = resize::new(w1, h1, w2, h2, RGBA8, Lanczos3).unwrap();
            resizer.resize(src.as_rgba(), dst.as_rgba_mut()).unwrap();

            IMAGE_LIBRARY
                .lock()
                .unwrap()
                .add_image("".to_string(), new_width, new_height, dst)
        } else {
            return -1;
        }
    }
}

#[wasm_bindgen]
pub fn darken(image_id: i32, value: i32) -> i32 {
    let mut image_data = IMAGE_LIBRARY
        .lock()
        .unwrap()
        .get_image_data(image_id)
        .unwrap()
        .clone();
    for i in 0..image_data.pixels.len() {
        if (i + 1) / 4 == 0 {
            continue;
        } else if image_data.pixels[i] > value as u8 {
            image_data.pixels[i] -= value as u8;
        } else {
            image_data.pixels[i] = 0;
        }
    }
    IMAGE_LIBRARY.lock().unwrap().add_image(
        "".to_string(),
        image_data.width,
        image_data.height,
        image_data.pixels,
    )
}

#[wasm_bindgen]
pub fn lighten_(image_id: i32, value: i32) -> i32 {
    let mut image_data = IMAGE_LIBRARY
        .lock()
        .unwrap()
        .get_image_data(image_id)
        .unwrap()
        .clone();
    let value = value as u8;
    for i in 0..image_data.pixels.len() {
        if (i + 1) / 4 == 0 {
            continue;
        } else if (image_data.pixels[i] + value) < 255 {
            image_data.pixels[i] += value;
        } else {
            image_data.pixels[i] = 255;
        }
    }
    IMAGE_LIBRARY.lock().unwrap().add_image(
        "".to_string(),
        image_data.width,
        image_data.height,
        image_data.pixels,
    )
}
#[wasm_bindgen]
pub fn blank_image(r: i32, g: i32, b: i32, a: i32, width: i32, height: i32) -> i32 {
    let pixel_total: usize = (width * height * 4) as usize;
    let mut image_data: Vec<u8> = vec![0; pixel_total];
    for i in (0..pixel_total).step_by(4) {
        image_data[i] = r as u8;
        image_data[i + 1] = g as u8;
        image_data[i + 2] = b as u8;
        image_data[i + 3] = a as u8;
    }
    IMAGE_LIBRARY
        .lock()
        .unwrap()
        .add_image("".to_string(), width, height, image_data)
}

#[wasm_bindgen]
pub fn grayscale_(image_id: i32) -> i32 {
    let mut image_data = IMAGE_LIBRARY
        .lock()
        .unwrap()
        .get_image_data(image_id)
        .unwrap()
        .clone();
    for i in (0..image_data.pixels.len()).step_by(4) {
        let avg = ((image_data.pixels[i] as f32 * 0.2989) as i32
            + (image_data.pixels[i + 1] as f32 * 0.5870) as i32
            + (image_data.pixels[i + 2] as f32 * 0.1140) as i32) as u8;
        image_data.pixels[i] = avg;
        image_data.pixels[i + 1] = avg;
        image_data.pixels[i + 2] = avg;
    }

    IMAGE_LIBRARY.lock().unwrap().add_image(
        "".to_string(),
        image_data.width,
        image_data.height,
        image_data.pixels,
    )
}

#[wasm_bindgen]
pub fn invert_color(image_id: i32) -> i32 {
    let mut image_data = IMAGE_LIBRARY
        .lock()
        .unwrap()
        .get_image_data(image_id)
        .unwrap()
        .clone();
    for i in (0..image_data.pixels.len()).step_by(4) {
        image_data.pixels[i] = 255 - image_data.pixels[i];
        image_data.pixels[i + 1] = 255 - image_data.pixels[i + 1];
        image_data.pixels[i + 2] = 255 - image_data.pixels[i + 2];
    }

    IMAGE_LIBRARY.lock().unwrap().add_image(
        "".to_string(),
        image_data.width,
        image_data.height,
        image_data.pixels,
    )
}

#[wasm_bindgen]
pub fn flip_horizontal(image_id: i32) -> i32 {
    let mut image_data = IMAGE_LIBRARY
        .lock()
        .unwrap()
        .get_image_data(image_id)
        .unwrap()
        .clone();
    let width = image_data.width as usize;
    let height = image_data.width as usize;
    for i in 0..height {
        for j in 0..width / 2 {
            let temp = image_data.pixels[i + j];
            image_data.pixels[i + j] = image_data.pixels[i + width - 1 - j];
            image_data.pixels[i + width - 1 - j] = temp;
        }
    }

    IMAGE_LIBRARY.lock().unwrap().add_image(
        "".to_string(),
        image_data.width,
        image_data.height,
        image_data.pixels,
    )
}

#[wasm_bindgen]
pub fn flip_vertical(image_id: i32) -> i32 {
    let mut image_data = IMAGE_LIBRARY
        .lock()
        .unwrap()
        .get_image_data(image_id)
        .unwrap()
        .clone();
    let width = image_data.height as usize;
    let height = image_data.height as usize;
    for i in 0..width {
        for j in 0..height / 2 {
            let temp = image_data.pixels[i + j];
            image_data.pixels[i + j] = image_data.pixels[(height - 1 - j) * width + i];
            image_data.pixels[(height - 1 - j) * width + i] = temp;
        }
    }

    IMAGE_LIBRARY.lock().unwrap().add_image(
        "".to_string(),
        image_data.width,
        image_data.height,
        image_data.pixels,
    )
}

#[wasm_bindgen]
pub fn mask_channel_r(image_id: i32) -> i32 {
    let mut image_data = IMAGE_LIBRARY
        .lock()
        .unwrap()
        .get_image_data(image_id)
        .unwrap()
        .clone();
    for i in (0..image_data.pixels.len()).step_by(4) {
        image_data.pixels[i] = 0;
    }

    IMAGE_LIBRARY.lock().unwrap().add_image(
        "".to_string(),
        image_data.width,
        image_data.height,
        image_data.pixels,
    )
}

#[wasm_bindgen]
pub fn mask_channel_g(image_id: i32) -> i32 {
    let mut image_data = IMAGE_LIBRARY
        .lock()
        .unwrap()
        .get_image_data(image_id)
        .unwrap()
        .clone();
    for i in (0..image_data.pixels.len()).step_by(4) {
        image_data.pixels[i + 1] = 0;
    }

    IMAGE_LIBRARY.lock().unwrap().add_image(
        "".to_string(),
        image_data.width,
        image_data.height,
        image_data.pixels,
    )
}

#[wasm_bindgen]
pub fn mask_channel_b(image_id: i32) -> i32 {
    let mut image_data = IMAGE_LIBRARY
        .lock()
        .unwrap()
        .get_image_data(image_id)
        .unwrap()
        .clone();
    for i in (0..image_data.pixels.len()).step_by(4) {
        image_data.pixels[i + 2] = 0;
    }

    IMAGE_LIBRARY.lock().unwrap().add_image(
        "".to_string(),
        image_data.width,
        image_data.height,
        image_data.pixels,
    )
}

#[wasm_bindgen]
pub fn blur(image_id: i32) -> i32 {
    let mut image_data = IMAGE_LIBRARY
        .lock()
        .unwrap()
        .get_image_data(image_id)
        .unwrap()
        .clone();

    let mut photon_image = PhotonImage::new(
        image_data.pixels,
        image_data.width as u32,
        image_data.height as u32,
    );

    conv::box_blur(&mut photon_image);

    IMAGE_LIBRARY.lock().unwrap().add_image(
        "".to_string(),
        image_data.width,
        image_data.height,
        photon_image.get_raw_pixels(),
    )
}

#[wasm_bindgen]
pub fn blur_gaussian(image_id: i32, radius: i32) -> i32 {
    let mut image_data = IMAGE_LIBRARY
        .lock()
        .unwrap()
        .get_image_data(image_id)
        .unwrap()
        .clone();

    let mut photon_image = PhotonImage::new(
        image_data.pixels,
        image_data.width as u32,
        image_data.height as u32,
    );

    conv::gaussian_blur(&mut photon_image, radius);

    IMAGE_LIBRARY.lock().unwrap().add_image(
        "".to_string(),
        image_data.width,
        image_data.height,
        photon_image.get_raw_pixels(),
    )
}

#[wasm_bindgen]
pub fn noise_perlin(image_id: i32) -> i32 {
    let mut image_data = IMAGE_LIBRARY
        .lock()
        .unwrap()
        .get_image_data(image_id)
        .unwrap()
        .clone();
    let perlin = Perlin::new();

    for i in (0..image_data.pixels.len()).step_by(4) {
        let val = perlin.get([
            image_data.pixels[i] as f64 / 255.0,
            image_data.pixels[i + 1] as f64 / 255.0,
            image_data.pixels[i + 2] as f64 / 255.0,
        ]) * 255.0;

        image_data.pixels[i] = val as u8;
        image_data.pixels[i + 1] = val as u8;
        image_data.pixels[i + 2] = val as u8;
    }

    IMAGE_LIBRARY.lock().unwrap().add_image(
        "".to_string(),
        image_data.width,
        image_data.height,
        image_data.pixels,
    )
}

#[wasm_bindgen]
pub fn sharpen_(image_id: i32) -> i32 {
    let mut image_data = IMAGE_LIBRARY
        .lock()
        .unwrap()
        .get_image_data(image_id)
        .unwrap()
        .clone();

    let mut photon_image = PhotonImage::new(
        image_data.pixels,
        image_data.width as u32,
        image_data.height as u32,
    );

    conv::sharpen(&mut photon_image);

    IMAGE_LIBRARY.lock().unwrap().add_image(
        "".to_string(),
        image_data.width,
        image_data.height,
        photon_image.get_raw_pixels(),
    )
}

#[wasm_bindgen]
pub fn overlay(image_id_1: i32, image_id_2: i32) -> i32 {
    let mut image_data_1 = IMAGE_LIBRARY
        .lock()
        .unwrap()
        .get_image_data(image_id_1)
        .unwrap()
        .clone();
    let mut photon_image_1 = PhotonImage::new(
        image_data_1.pixels,
        image_data_1.width as u32,
        image_data_1.height as u32,
    );

    let mut image_data_2 = IMAGE_LIBRARY
        .lock()
        .unwrap()
        .get_image_data(image_id_2)
        .unwrap()
        .clone();
    let mut photon_image_2 = PhotonImage::new(
        image_data_2.pixels,
        image_data_2.width as u32,
        image_data_2.height as u32,
    );

    multiple::blend(&mut photon_image_1, &photon_image_2, "overlay");

    IMAGE_LIBRARY.lock().unwrap().add_image(
        "".to_string(),
        photon_image_1.get_width() as i32,
        photon_image_1.get_height() as i32,
        photon_image_1.get_raw_pixels(),
    )
}
#[wasm_bindgen]
pub fn multiply(image_id_1: i32, image_id_2: i32) -> i32 {
    let mut image_data_1 = IMAGE_LIBRARY
        .lock()
        .unwrap()
        .get_image_data(image_id_1)
        .unwrap()
        .clone();
    let mut photon_image_1 = PhotonImage::new(
        image_data_1.pixels,
        image_data_1.width as u32,
        image_data_1.height as u32,
    );

    let mut image_data_2 = IMAGE_LIBRARY
        .lock()
        .unwrap()
        .get_image_data(image_id_2)
        .unwrap()
        .clone();
    let mut photon_image_2 = PhotonImage::new(
        image_data_2.pixels,
        image_data_2.width as u32,
        image_data_2.height as u32,
    );

    multiple::blend(&mut photon_image_1, &photon_image_2, "multiply");

    IMAGE_LIBRARY.lock().unwrap().add_image(
        "".to_string(),
        photon_image_1.get_width() as i32,
        photon_image_1.get_height() as i32,
        photon_image_1.get_raw_pixels(),
    )
}

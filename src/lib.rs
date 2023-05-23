use std::fmt::format;
use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;
use base64::{ encode, decode };
use image::load_from_memory;
use image::ImageOutputFormat::Png;

use wasm_bindgen::convert::IntoWasmAbi;

// The bindgen crate exports a macro for handling an action of exporting a function to javascript
#[wasm_bindgen]
// if we are borrowing a string type must be str if we are have ownership of a string type must be String
pub fn grayscale(encoded_file: &str) -> String {
    log(&"Grayscale called".into());

    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"Image decoded".into());

    let mut img = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image loaded".into());

    img = img.grayscale();
    log(&"Grayscale effect applied".into());

    let mut buffer = vec![];
    // &mut to borrow a mut variable
    img.write_to(&mut buffer, Png).unwrap();
    log(&"New image written".into());

    let encoded_img = encode(&buffer); // returned will be a base64 string
    // now add what we replaced to send data url
    let data_url = format!(
        "data:image/png;base64,{}",
        encoded_img
    );

    data_url
}

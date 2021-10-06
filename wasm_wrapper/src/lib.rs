use wasm_bindgen::prelude::*;

use convert_office_images::convert;

use std::io::Cursor;

#[wasm_bindgen]
pub fn convert_images(input: Vec<u8>) -> Result<Vec<u8>, JsValue> {
    let mut input = Cursor::new(input);
    let mut output = Cursor::new(Vec::new());

    convert(&mut input, &mut output).map_err(|_| JsValue::from_str("failed to convert images"))?;

    Ok(output.into_inner())
}

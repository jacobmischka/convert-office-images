use lazy_static::lazy_static;
use serde_json;
use wasm_bindgen::prelude::*;

use convert_office_images::{
    convert, get_images_data,
    img::{convert_image, AcceptedFormat, Image},
};

use std::{cell::RefCell, io::Cursor, str::FromStr, sync::Mutex};

lazy_static! {
    static ref DOCUMENT: Mutex<RefCell<Vec<u8>>> = Default::default();
    static ref IMAGES: Mutex<RefCell<Vec<Vec<u8>>>> = Default::default();
}

#[wasm_bindgen]
pub fn add_document(data: Vec<u8>) -> Result<String, JsValue> {
    let images_data: Vec<Vec<u8>> = get_images_data(Cursor::new(&data)).map_err(Error)?;

    let images: Vec<Image> = images_data
        .iter()
        .map(|img_data| Image::with_format(img_data.as_slice(), AcceptedFormat::Jpeg))
        .collect();

    let doc_cell = DOCUMENT
        .lock()
        .map_err(|_| JsValue::from_str("failed locking document"))?;
    doc_cell.replace(data);

    let images_cell = IMAGES
        .lock()
        .map_err(|_| JsValue::from_str("failed locking images"))?;
    images_cell.replace(images_data);

    serde_json::to_string(&images).map_err(|_| JsValue::from_str("failed serializing images"))
}

#[wasm_bindgen]
pub fn get_image(index: usize, format: Option<String>) -> Result<String, JsValue> {
    let image_cell = IMAGES
        .lock()
        .map_err(|_| JsValue::from_str("failed getting images"))?;

    let image_ref = image_cell.borrow();

    let data: &Vec<u8> = image_ref
        .get(index)
        .as_ref()
        .ok_or(JsValue::from_str("no image for specified index"))?;

    let img: Image = if let Some(format) = format {
        let format =
            AcceptedFormat::from_str(&format).map_err(|_| JsValue::from_str("invalid format"))?;
        let data = convert_image(data, format.output_format())
            .map_err(|_| JsValue::from_str("failed converting image"))?;
        Image::with_format(data.as_slice(), AcceptedFormat::Jpeg)
    } else {
        Image::with_format(data.as_slice(), AcceptedFormat::Jpeg)
    };

    serde_json::to_string(&img).map_err(|_| JsValue::from_str("failed serializing image"))
}

#[wasm_bindgen]
pub fn convert_images(input: Vec<u8>) -> Result<Vec<u8>, JsValue> {
    let mut input = Cursor::new(input);
    let mut output = Cursor::new(Vec::new());

    convert(&mut input, &mut output).map_err(|_| JsValue::from_str("failed to convert images"))?;

    Ok(output.into_inner())
}

struct Error(convert_office_images::Error);

impl From<convert_office_images::Error> for Error {
    fn from(err: convert_office_images::Error) -> Error {
        Error(err)
    }
}

impl From<Error> for JsValue {
    fn from(err: Error) -> JsValue {
        use convert_office_images::Error::*;

        match err.0 {
            ZipError(_) => JsValue::from_str("failed operating zip file"),
            XmlError(_) => JsValue::from_str("failed saving document metadata"),
            IoError(_) => JsValue::from_str("failed performing file operation"),
            ImageError(_) => JsValue::from_str("failed converting image"),
        }
    }
}

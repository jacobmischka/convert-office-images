use base64;
use image::{io::Reader as ImageReader, ImageOutputFormat};
use serde::Serialize;

use std::{fmt, io::Cursor, path::PathBuf, str::FromStr};

use super::Error;

#[derive(Serialize)]
pub struct Image {
    base64: String,
    size: usize,
    format: String,
}

impl Image {
    pub fn with_format(data: &[u8], format: AcceptedFormat) -> Self {
        Image {
            size: data.len(),
            format: format.to_string(),
            base64: format!("data:{},{}", format.content_type(), base64::encode(data)),
        }
    }
}

pub fn convert_image(data: &[u8], format: ImageOutputFormat) -> Result<Vec<u8>, Error> {
    let img = ImageReader::new(Cursor::new(data))
        .with_guessed_format()?
        .decode()?;

    let mut buf = Vec::new();
    img.write_to(&mut buf, format)?;

    Ok(buf)
}

pub fn new_img_name(old_path: &str, format: AcceptedFormat) -> String {
    let mut path = PathBuf::from(old_path);
    path.set_extension(format.extension());
    path.as_path().display().to_string()
}

#[derive(Clone, Copy)]
pub enum AcceptedFormat {
    Jpeg,
    Png,
}

impl fmt::Display for AcceptedFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match *self {
                AcceptedFormat::Jpeg => "JPEG",
                AcceptedFormat::Png => "PNG",
            }
        )
    }
}

impl AcceptedFormat {
    pub fn extension(self) -> &'static str {
        match self {
            AcceptedFormat::Jpeg => "jpg",
            AcceptedFormat::Png => "png,",
        }
    }

    pub fn output_format(self) -> ImageOutputFormat {
        match self {
            AcceptedFormat::Jpeg => ImageOutputFormat::Jpeg(90),
            AcceptedFormat::Png => ImageOutputFormat::Png,
        }
    }

    pub fn content_type(self) -> &'static str {
        match self {
            AcceptedFormat::Jpeg => "image/jpeg",
            AcceptedFormat::Png => "image/png",
        }
    }

    pub fn content_type_decl(self) -> String {
        format!(
            r#"<Default Extension="{}" ContentType="{}" />"#,
            self.extension(),
            self.content_type()
        )
    }
}

impl FromStr for AcceptedFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "jpg" => Ok(AcceptedFormat::Jpeg),
            "png" => Ok(AcceptedFormat::Png),
            _ => Err(()),
        }
    }
}

pub fn get_format(format: &str) -> Option<ImageOutputFormat> {
    match format {
        _ => None,
    }
}

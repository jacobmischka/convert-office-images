use image::error::ImageError;
use quick_xml::{
    events::{BytesStart, Event as XmlEvent},
    Error as XmlError, Reader as XmlReader, Writer as XmlWriter,
};
use zip::{
    read::{ZipArchive, ZipFile},
    result::ZipError,
    write::{FileOptions, ZipWriter},
};

use std::{
    borrow::Cow,
    fmt,
    io::{Cursor, Error as IoError, Read, Seek, Write},
    str,
};

pub mod img;

use img::{convert_image, new_img_name, AcceptedFormat};

const JPG_TYPE: &str = r#"<Default Extension="jpg" ContentType="image/jpeg" />"#;

const CONTENT_TYPES_NAME: &str = "[Content_Types].xml";
const RELS_EXT: &str = ".xml.rels";

pub fn convert<R, W>(reader: &mut R, writer: &mut W) -> Result<(), Error>
where
    R: Read + Seek,
    W: Write + Seek,
{
    let mut reader = ZipArchive::new(reader)?;
    let mut writer = ZipWriter::new(writer);

    let mut rels: Vec<(String, FileOptions, Vec<u8>)> = Vec::new();
    let mut replaced_images: Vec<(String, String)> = Vec::new();

    for i in 0..reader.len() {
        let mut file = reader.by_index(i)?;
        if file.name() == CONTENT_TYPES_NAME {
            // TODO: Unix mode?
            writer.start_file(
                CONTENT_TYPES_NAME,
                FileOptions::default()
                    .compression_method(file.compression())
                    .last_modified_time(file.last_modified()),
            )?;
            let mut xml_writer = XmlWriter::new(Cursor::new(Vec::new()));

            {
                let mut has_jpg = false;
                let mut xml_reader = XmlReader::from_reader(Cursor::new(read_zip(&mut file)?));
                let mut buf = Vec::new();
                loop {
                    match xml_reader.read_event(&mut buf) {
                        Ok(XmlEvent::Start(e)) => {
                            if !has_jpg && e.name() == b"Default" {
                                for attr in e.attributes() {
                                    let attr = attr?;
                                    if attr.key == b"Extension" && attr.value.as_ref() == b"jpg" {
                                        has_jpg = true;
                                    }
                                }
                            }

                            xml_writer.write_event(XmlEvent::Start(e))?;
                        }
                        Ok(XmlEvent::End(e)) => {
                            if !has_jpg && e.name() == b"Types" {
                                xml_writer.write(JPG_TYPE.as_bytes())?;
                            }
                            xml_writer.write_event(XmlEvent::End(e))?;
                        }
                        Ok(XmlEvent::Eof) => break,
                        Ok(e) => xml_writer.write_event(e)?,
                        Err(e) => return Err(e.into()),
                    }
                    buf.clear();
                }
            }

            writer.write_all(&xml_writer.into_inner().into_inner())?;
        } else if file.name().ends_with(RELS_EXT) {
            let mut buf = Vec::new();
            file.read_to_end(&mut buf)?;

            rels.push((
                file.name().to_owned(),
                FileOptions::default()
                    .last_modified_time(file.last_modified())
                    .compression_method(file.compression()),
                buf,
            ));
        } else if file.name().contains("/media/") {
            let format = AcceptedFormat::Jpeg;
            match convert_image(&read_zip(&mut file)?, format.output_format()) {
                Ok(new_img) => {
                    let new_name = new_img_name(file.name(), format);

                    writer.start_file(
                        &new_name,
                        FileOptions::default().compression_method(file.compression()),
                    )?;
                    writer.write_all(&new_img)?;

                    replaced_images.push((
                        strip_document_dir(file.name()),
                        strip_document_dir(&new_name),
                    ));
                }
                Err(e) => {
                    eprintln!("Failed converting image: {:?}", e);
                    writer.raw_copy_file(file)?;
                }
            }
        } else {
            writer.raw_copy_file(file)?;
        }
    }

    for (name, file_options, buf) in rels {
        writer.start_file(name, file_options)?;
        writer.write_all(&replace_rels(buf, &replaced_images)?)?;
    }

    writer.flush()?;

    Ok(())
}

pub fn get_images_data<R>(reader: R) -> Result<Vec<Vec<u8>>, Error>
where
    R: Read + Seek,
{
    let mut reader = ZipArchive::new(reader)?;
    let mut images = Vec::new();

    for i in 0..reader.len() {
        let mut file = reader.by_index(i)?;
        if file.name().contains("/media/") {
            let mut img_buf = Vec::new();
            file.read_to_end(&mut img_buf)?;
            images.push(img_buf);
        }
    }

    Ok(images)
}

fn strip_document_dir(path: &str) -> String {
    path.replace("word/", "").replace("ppt/", "")
}

fn read_zip(file: &mut ZipFile) -> Result<Vec<u8>, Error> {
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(buf)
}

fn replace_rels(rels: Vec<u8>, replacements: &[(String, String)]) -> Result<Vec<u8>, Error> {
    let mut xml_reader = XmlReader::from_reader(Cursor::new(rels));
    let mut xml_writer = XmlWriter::new(Cursor::new(Vec::new()));

    let mut buf = Vec::new();
    loop {
        match xml_reader.read_event(&mut buf) {
            Ok(XmlEvent::Empty(e)) => {
                if e.name() == b"Relationship" {
                    let mut new_elem =
                        BytesStart::owned(b"Relationship".to_vec(), "Relationship".len());

                    new_elem.extend_attributes(e.attributes().filter_map(|attr| {
                        if let Ok(mut attr) = attr {
                            if attr.key == b"Target" {
                                if let Ok(value) = str::from_utf8(&attr.value.clone()) {
                                    for (from, to) in replacements {
                                        if value.contains(from) {
                                            attr.value =
                                                Cow::Owned(value.replace(from, to).into_bytes());
                                        }
                                    }
                                }
                            }

                            Some(attr)
                        } else {
                            None
                        }
                    }));

                    xml_writer.write_event(XmlEvent::Empty(new_elem))?;
                } else {
                    xml_writer.write_event(XmlEvent::Empty(e))?;
                }
            }
            Ok(XmlEvent::Eof) => break,
            Ok(e) => xml_writer.write_event(e)?,
            Err(e) => return Err(e.into()),
        }
        buf.clear();
    }

    Ok(xml_writer.into_inner().into_inner())
}

#[derive(Debug)]
pub enum Error {
    ZipError(ZipError),
    XmlError(XmlError),
    IoError(IoError),
    ImageError(ImageError),
}

impl From<ZipError> for Error {
    fn from(e: ZipError) -> Self {
        Error::ZipError(e)
    }
}

impl From<XmlError> for Error {
    fn from(e: XmlError) -> Self {
        Error::XmlError(e)
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Error::IoError(e)
    }
}

impl From<ImageError> for Error {
    fn from(e: ImageError) -> Self {
        Error::ImageError(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ZipError(e) => write!(f, "ZIP Error: {}", e),
            Error::XmlError(e) => write!(f, "XML Error: {}", e),
            Error::ImageError(e) => write!(f, "Image Error: {}", e),
            Error::IoError(e) => write!(f, "IO Error: {}", e),
        }
    }
}

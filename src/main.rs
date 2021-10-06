use convert_office_images::convert;

use std::io::{self, Cursor, Read, Write};

fn main() {
    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf).unwrap();

    let mut input = Cursor::new(buf);
    let mut output = Cursor::new(Vec::new());

    convert(&mut input, &mut output).unwrap();

    io::stdout()
        .write_all(&output.into_inner().as_slice())
        .unwrap();
}

use chrono::prelude::*;
use std::fs::{self, File};
use std::io::{self, Read};

enum FileType {
    FILE,
    POSIX_EXECUTABLE,
    DIRECTORY,
    SYMLINK,
}

struct ZipEntry {
    file_name: String,
    file_type: FileType,
    contents: Vec<u8>,
    last_modified_timestamp: DateTime<Utc>,
}

struct LocalFileHeader {
    signature: u32,                 //  0, 4 | SHALL be 0x04034b50
    appnote_compatibility_min: u16, //  4, 2 |
    general_purpose_bits: u16,      //  6, 2 |
    compression_method: u16,        //  8, 2 | SHALL be 0 or 8
    dos_timestamp: u16,             // 10, 4 |
    crc32: u16,                     // 14, 4 |
    compressed_size_32: u32,        // 18, 4 |
    uncompressed_size_32: u32,      // 22, 4 |
    file_name_raw_length: u16,      // 26, 2 | See fileNameRaw
    extra_fields_length: u16,       // 28, 2 | See extraFields
    file_name_raw: Vec<u8>,         // 30, fileNameRawLength
    extra_fields: Vec<u8>,
}

enum DataDescriptor {}

fn main() -> io::Result<()> {
    match fs::read("test.txt") {
        Ok(b) => println!("{}", String::from_utf8(b).unwrap()),
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}

fn read_u16<R: Read>(reader: &mut R) -> io::Result<u16> {
    let mut buf = [0u8; 2];
    reader.read_exact(&mut buf)?;
    Ok(u16::from_le_bytes(buf))
}

fn read_u32<R: Read>(reader: &mut R) -> io::Result<u32> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

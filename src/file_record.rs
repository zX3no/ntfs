//! File Records are equivalent to inodes in Unix terminology.
//! The first FILE Record that describes a given file is called the Base FILE record and others are called Extension FILE Records.
//! A FILE Record is built up from a header, several variable length attributes and an end marker (0xFFFFFFFF).
//!
//! # Header
//!
//!| Offset | Size | OS | Description                                      |
//!|--------|------|----|--------------------------------------------------|
//!| 0x00   | 4    |    | Magic number 'FILE'                              |
//!| 0x04   | 2    |    | Offset to the Update Sequence                    |
//!| 0x06   | 2    |    | Size in words of Update Sequence (S)             |
//!| 0x08   | 8    |    | $LogFile Sequence Number (LSN)                   |
//!| 0x10   | 2    |    | Sequence number                                  |
//!| 0x12   | 2    |    | Hard link count                                  |
//!| 0x14   | 2    |    | Offset to the first Attribute                    |
//!| 0x16   | 2    |    | Flags                                            |
//!| 0x18   | 4    |    | Real size of the FILE record                     |
//!| 0x1C   | 4    |    | Allocated size of the FILE record                |
//!| 0x20   | 8    |    | File reference to the base FILE record           |
//!| 0x28   | 2    |    | Next Attribute Id                                |
//!| 0x2A   | 2    | XP | Align to a 4-byte boundary                       |
//!| 0x2C   | 4    | XP | Number of this MFT Record                        |
//!|        | 2    |    | Update Sequence Number (a)                       |
//!|        | 2S-2 |    | Update Sequence Array (a)                        |

use crate::*;
use std::{
    fs::File,
    io::{BufReader, Read},
    str::from_utf8_unchecked,
};

pub struct FileRecord {
    pub update_sequence_offset: u16,
    pub update_sequence_size: u16,
    pub log_file_sequence_number: u64,
    pub sequence_number: u16,
    pub hard_link_count: u16,
    pub first_attribute_offset: u16,
    pub flags: u16,
    pub real_size: u32,
    pub allocated_size: u32,
    pub file_reference_to_base: u64,
    pub next_attribute_id: u16,
    pub update_sequence_number: u16,
}

pub enum FileRecordFlags {
    RecordInUse = 0x01,
    RecordIsDirectory = 0x02,
    Unknown = 0x04,
    Unknown2 = 0x08,
}

pub enum FileAttributes {
    StandardInformation = 0x10,
    FileName = 0x30,
    SecurityDescriptor = 0x50,
    Data = 0x80,
}

pub enum Permissions {
    ReadOnly = 0x0001,
    Hidden = 0x0002,
    System = 0x0004,
    Archive = 0x0020,
    Device = 0x0040,
    Normal = 0x0080,
    Temporary = 0x0100,
    SparseFile = 0x0200,
    ReparsePoint = 0x0400,
    Compressed = 0x0800,
    Offline = 0x1000,
    NotContentIndexed = 0x2000,
    Encrypted = 0x4000,
}

pub fn file_record(reader: &mut BufReader<File>) -> FileRecord {
    let mut buf = [0u8; 1024];
    reader.read_exact(&mut buf).unwrap();

    let file = unsafe { from_utf8_unchecked(&buf[0..4]) };
    assert_eq!(file, "FILE");

    let update_sequence_offset = u16::from_le_bytes([buf[4], buf[5]]);

    //Size in words(u16) of Update Sequence
    //Does this mean it's the number of u16's?
    let update_sequence_size = u16::from_le_bytes([buf[6], buf[7]]);

    let log_file_sequence_number = u64::from_le_bytes([
        buf[8], buf[9], buf[10], buf[11], buf[12], buf[13], buf[14], buf[15],
    ]);

    let sequence_number = u16::from_le_bytes([buf[16], buf[17]]);
    let hard_link_count = u16::from_le_bytes([buf[18], buf[19]]);
    let first_attribute_offset = u16::from_le_bytes([buf[20], buf[21]]);

    //See `Flags`
    let flags = u16::from_le_bytes([buf[22], buf[23]]);

    //In bytes
    let real_size = u32::from_le_bytes([buf[24], buf[25], buf[26], buf[27]]);
    let allocated_size = u32::from_le_bytes([buf[28], buf[29], buf[30], buf[31]]);

    //This is zero for Base MFT Records.
    //When it is not zero it is a MFT Reference pointing to the Base MFT Record to which this Record belongs.
    //The Base Record contains the information about the Extension Record.
    //This information is stored in an ATTRIBUTE_LIST attribute.
    let file_reference_to_base = u64::from_le_bytes([
        buf[32], buf[33], buf[34], buf[35], buf[36], buf[37], buf[38], buf[39],
    ]);

    let next_attribute_id = u16::from_le_bytes([buf[40], buf[41]]);

    //Windows XP only

    //Align to 4 byte boundary
    // let _ = u16::from_le_bytes([buf[42], buf[43]]);

    //Number of this MFT Record
    // let mft_record_number = u32::from_le_bytes([buf[44], buf[45], buf[46], buf[47]]);

    let update_sequence_number = u16::from_le_bytes([
        buf[update_sequence_offset as usize],
        buf[update_sequence_offset as usize + 1],
    ]);

    //0x10	$STANDARD_INFORMATION
    //0x30	$FILE_NAME	filename
    //0x50	$SECURITY_DESCRIPTOR
    //0x80	$DATA	[Unnamed]
    // let first_attribute = buf[first_attribute_offset as usize];
    // dbg!(first_attribute);

    standard_attribute_header(&buf[first_attribute_offset as usize..]);

    println!();

    FileRecord {
        update_sequence_offset,
        update_sequence_size,
        log_file_sequence_number,
        sequence_number,
        hard_link_count,
        first_attribute_offset,
        flags,
        real_size,
        allocated_size,
        file_reference_to_base,
        next_attribute_id,
        update_sequence_number,
    }
}

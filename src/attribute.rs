use std::str::from_utf8;

/// Every attribute in every MFT record has a standard header.
/// The header stores information about the attribute's type, size, name (optional) and whether it is resident, or not.
/// <https://flatcap.github.io/linux-ntfs/ntfs/concepts/attribute_header.html>
use crate::*;

///In NTFS, everything on disk is a file. Even the metadata is stored as a set of files.
///The Master File Table (MFT) is an index of every file on the volume.
///For each file, the MFT keeps a set of records called attributes and each attribute stores a different type of information.
///
///<https://flatcap.github.io/linux-ntfs/ntfs/attributes/index.html>
//TODO: Clean this up.
pub const STANDARD_INFORMATION: u8 = 0x10;
pub const ATTRIBUTE_LIST: u8 = 0x20;
pub const FILE_NAME: u8 = 0x30;
pub const VOLUME_VERSION: u8 = 0x40; //Windows NT
pub const OBJECT_ID: u8 = 0x40; //Windows 2000
pub const SECURITY_DESCRIPTOR: u8 = 0x50;
pub const VOLUME_NAME: u8 = 0x60;
pub const VOLUME_INFORMATION: u8 = 0x70;
pub const DATA: u8 = 0x80;
pub const INDEX_ROOT: u8 = 0x90;
pub const INDEX_ALLOCATION: u8 = 0xA0;
pub const BITMAP: u8 = 0xB0;
pub const SYMBOLIC_LINK: u8 = 0xC0; //Windows NT
pub const REPARSE_POINT: u8 = 0xC0; //Windows 2000
pub const EA_INFORMATION: u8 = 0xD0;
pub const EA: u8 = 0xE0;
pub const PROPERTY_SET: u8 = 0xF0; //Windows NT
pub const LOGGED_UTILITY_STREAM: u16 = 0x100; //Windows 2000

#[derive(Debug)]
pub enum Attribute {
    Standard(Standard),
    List(List),
    FileName(FileName),
}

#[derive(Debug)]
pub enum AttributeFlags {
    Compressed = 0x0001,
    Encrypted = 0x4000,
    Sparse = 0x8000,
}

pub fn attribute_header(buf: &[u8]) {
    let attribute_type = u32(&buf[0..]);
    match attribute_type {
        //This is wrong, the attribute type is from the list of constants above.
        //How to check which header to use though?
        0x10 | 0x60 | 0x90 | 0xB0 => {
            resident_header(buf);
        }
        0x20 | 0x80 | 0xA0 => {
            non_resident_header(buf);
        }
        _ => unreachable!("{}", attribute_type),
    }
}

fn non_resident_header(_buf: &[u8]) {
    todo!()
}

pub struct Resident {
    pub attribute_type: u32,
    pub attribute_length_including_header: u32,
    pub non_resident_flag: u8,
    pub name_length: u8,
    pub offset_to_name: u16,
    pub flags: u16,
    pub attribute_id: u16,
    pub attribute_length: u32,
    pub attribute_offset: u16,
    pub indexed_flag: u8,
    pub attribute: Attribute,
}

//https://flatcap.github.io/linux-ntfs/ntfs/concepts/attribute_header.html
pub fn resident_header(buf: &[u8]) -> Resident {
    let attribute_type = u32(&buf[0..]);
    let attribute_length_including_header = u32(&buf[4..]);
    let non_resident_flag = buf[8];
    let name_length = buf[9];
    let offset_to_name = u16(&buf[10..]);
    let flags = u16(&buf[12..]);
    let attribute_id = u16(&buf[14..]);
    let attribute_length = u32(&buf[16..]);
    let attribute_offset = u16(&buf[20..]);
    let indexed_flag = buf[24];
    // let padding = buf[25];

    let attribute = if name_length != 0 {
        let name_end = 0x18 + (name_length as usize * 2);
        let name = &buf[0x18..name_end];
        dbg!(from_utf8(name).unwrap());

        assert_eq!(name_end, attribute_offset as usize);
        &buf[attribute_offset as usize..attribute_offset as usize + attribute_length as usize]
    } else {
        //TODO: Check with Godbolt, better to restrict the slice size? or leave it as &buf[attribute_offset..].
        &buf[attribute_offset as usize..attribute_offset as usize + attribute_length as usize]
    };

    let attribute = match attribute_type as u8 {
        STANDARD_INFORMATION => Attribute::Standard(standard_information_attribute(attribute)),
        ATTRIBUTE_LIST => Attribute::List(attribute_list(attribute)),
        FILE_NAME => Attribute::FileName(file_name(attribute)),
        _ => todo!("{}", attribute_type as u8),
    };

    Resident {
        attribute_type,
        attribute_length_including_header,
        non_resident_flag,
        name_length,
        offset_to_name,
        flags,
        attribute_id,
        attribute_length,
        attribute_offset,
        indexed_flag,
        attribute,
    }
}

#[derive(Debug)]
pub struct Standard {
    pub file_creation_time: u64,
    pub file_alteration_time: u64,
    pub mft_changed_time: u64,
    pub file_read_time: u64,
    pub dos_file_permissions: u32,
    pub maximum_number_of_versions: u32,
    pub version_number: u32,
    pub class_id: u32,
}

//https://flatcap.github.io/linux-ntfs/ntfs/attributes/standard_information.html
//minimum size of 48 bytes and a maximum of 72 bytes.
pub fn standard_information_attribute(buf: &[u8]) -> Standard {
    let file_creation_time = u64(&buf[0..]);
    let file_alteration_time = u64(&buf[8..]);
    let mft_changed_time = u64(&buf[16..]);
    let file_read_time = u64(&buf[24..]);
    let dos_file_permissions = u32(&buf[32..]);
    let maximum_number_of_versions = u32(&buf[36..]);
    let version_number = u32(&buf[40..]);
    let class_id = u32(&buf[44..]);

    //Unused 0x30..0x40

    Standard {
        file_creation_time,
        file_alteration_time,
        mft_changed_time,
        file_read_time,
        dos_file_permissions,
        maximum_number_of_versions,
        version_number,
        class_id,
    }
}

#[derive(Debug)]
pub struct List {
    pub attribute_type: u32,
    pub record_length: u16,
    pub name_length: u8,
    pub name_offset: u8,
    pub starting_vcn: u64,
    pub base_file_reference: u64,
    pub attribute_id: u16,
    pub name: Option<String>,
}

//https://flatcap.github.io/linux-ntfs/ntfs/attributes/attribute_list.html
pub fn attribute_list(buf: &[u8]) -> List {
    let attribute_type = u32(&buf[0..]);
    let record_length = u16(&buf[4..]);
    let name_length = buf[6];
    let name_offset = buf[7];
    //0 if the attribute is resident
    let starting_vcn = u64(&buf[8..]);
    let base_file_reference = u64(&buf[16..]);
    let attribute_id = u16(&buf[24..]);

    if name_length != 0 {
        let name_end = 26 + (name_length as usize * 2);
        let name = &buf[26..name_end];
        dbg!(from_utf8(name).unwrap());
    }

    List {
        attribute_type,
        record_length,
        name_length,
        name_offset,
        starting_vcn,
        base_file_reference,
        attribute_id,
        name: None,
    }
}

#[derive(Debug)]
pub struct FileName {
    pub parent_directory: u64,
    pub file_creation_time: u64,
    pub file_alteration_time: u64,
    pub mft_changed: u64,
    pub file_read_time: u64,
    pub allocated_size: u64,
    pub real_size: u64,
    pub flags: u32,
    pub ea_reparse: u32,
    pub file_name_length: u8,
    pub file_name_namespace: u8,
    pub file_name: Option<String>,
}

fn file_name(buf: &[u8]) -> FileName {
    let parent_directory = u64(&buf[0x00..]);
    let file_creation_time = u64(&buf[0x08..]);
    let file_alteration_time = u64(&buf[0x10..]);
    let mft_changed = u64(&buf[0x18..]);
    let file_read_time = u64(&buf[0x20..]);
    let allocated_size = u64(&buf[0x28..]);
    let real_size = u64(&buf[0x30..]);
    let flags = u32(&buf[0x38..]);
    let ea_reparse = u32(&buf[0x3C..]);
    let file_name_length = buf[0x40];
    let file_name_namespace = buf[0x41];

    if file_name_length != 0 {
        let file_name_end = 0x42 + (file_name_length as usize * 2);
        let file_name = &buf[0x42..file_name_end];
        dbg!(from_utf8(file_name).unwrap());
    }

    FileName {
        parent_directory,
        file_creation_time,
        file_alteration_time,
        mft_changed,
        file_read_time,
        allocated_size,
        real_size,
        flags,
        ea_reparse,
        file_name_length,
        file_name_namespace,
        file_name: None,
    }
}

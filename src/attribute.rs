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

pub struct Attribute {}

pub enum AttributeFlags {
    Compressed = 0x0001,
    Encrypted = 0x4000,
    Sparse = 0x8000,
}

//There are four types of attribute headers.
//Residant - Named
//Residant - Unanmed
//Non-Residant - Named
//Non-Residant - Unanmed
pub fn standard_attribute_header(buf: &[u8]) {
    let attribute_type = u32_from_slice(&buf[0..]);
    let attribute_length_including_header = u32_from_slice(&buf[4..]);
    let non_resident_flag = buf[8];
    let name_length = buf[9];
    let offset_to_name = u16_from_slice(&buf[10..]);
    let flags = u16_from_slice(&buf[12..]);
    let attribute_id = u16_from_slice(&buf[14..]);
    let attribute_length = u32_from_slice(&buf[16..]);
    let attribute_offset = u16_from_slice(&buf[20..]);
    let indexed_flag = buf[24];
    // let padding = buf[25];

    #[cfg(debug_assertions)]
    if name_length == 0 {
        assert_eq!(attribute_offset, 0x18);
        assert_eq!(offset_to_name, 0x18);
    }

    // let attriubte_offset = if name_length == 0 { 0x18 } else { todo!() };

    dbg!(
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
    );
    println!();

    match attribute_type as u8 {
        STANDARD_INFORMATION => standard_information_attribute(
            //Is it better to restrict the slice size, or leave it as &buf[attribute_offset..].
            //TODO: Check with godbolt.
            &buf[attribute_offset as usize..attribute_offset as usize + attribute_length as usize],
        ),
        _ => todo!("{}", attribute_type as u8),
    }
}

//https://flatcap.github.io/linux-ntfs/ntfs/concepts/attribute_header.html
//https://flatcap.github.io/linux-ntfs/ntfs/attributes/standard_information.html
//minimum size of 48 bytes and a maximum of 72 bytes.
pub fn standard_information_attribute(buf: &[u8]) {
    let file_creation_time = u64_from_slice(&buf[0..]);
    let file_alteration_time = u64_from_slice(&buf[8..]);
    let mft_changed_time = u64_from_slice(&buf[16..]);
    let file_read_time = u64_from_slice(&buf[24..]);

    // let file_creation_time = u64::from_le_bytes([
    //     buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7],
    // ]);

    // let file_alteration_time = u64::from_le_bytes([
    //     buf[8], buf[9], buf[10], buf[11], buf[12], buf[13], buf[14], buf[15],
    // ]);

    // let mft_changed_time = u64::from_le_bytes([
    //     buf[16], buf[17], buf[18], buf[19], buf[20], buf[21], buf[22], buf[23],
    // ]);

    // let file_read_time = u64::from_le_bytes([
    //     buf[24], buf[25], buf[26], buf[27], buf[28], buf[29], buf[30], buf[31],S
    // ]);

    let dos_file_permissions = u32::from_le_bytes([buf[32], buf[33], buf[34], buf[35]]);
    let maxium_number_of_versions = u32::from_le_bytes([buf[36], buf[37], buf[38], buf[39]]);
    let version_number = u32::from_le_bytes([buf[40], buf[41], buf[42], buf[43]]);
    let class_id = u32::from_le_bytes([buf[44], buf[45], buf[46], buf[47]]);

    //Unused 0x30..0x40

    dbg!(
        file_creation_time,
        file_alteration_time,
        mft_changed_time,
        file_read_time,
        dos_file_permissions,
        maxium_number_of_versions,
        version_number,
        class_id,
    );
}

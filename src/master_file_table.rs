//! All file, directory and metafile data—file name, creation date, size and access permissions are stored in the Master File Table.
//! The MFT is a collection of FILE records.
//! Each file in the MFT has a `fileID`, this is analogous to an inode in unix.
//! The description of each file is packed into [FILE records](struct@crate::file_record::FileRecord).
//TODO: Explain $ATTRIBUTE_LIST
//! If one FILE record is not large enough (this is unusual), then an $ATTRIBUTE_LIST attribute is needed.
//! The first 24 FILE records are reserved for the system files.
//!
//!| fileID | Filename  | OS  | Description                                            |
//!|--------|-----------|-----|--------------------------------------------------------|
//!| 0      | $MFT      |     | Master File Table - An index of every file             |
//!| 1      | $MFTMirr  |     | A backup copy of the first 4 records of the MFT        |
//!| 2      | $LogFile  |     | Transactional logging file                             |
//!| 3      | $Volume   |     | Serial number, creation time, dirty flag               |
//!| 4      | $AttrDef  |     | Attribute definitions                                  |
//!| 5      | . (dot)   |     | Root directory of the disk                             |
//!| 6      | $Bitmap   |     | Contains volume's cluster map (in-use vs. free)        |
//!| 7      | $Boot     |     | Boot record of the volume                              |
//!| 8      | $BadClus  |     | Lists bad clusters on the volume                       |
//!| 9      | $Quota    | NT  | Quota information                                      |
//!| 9      | $Secure   | 2K  | Security descriptors used by the volume                |
//!| 10     | $UpCase   |     | Table of uppercase characters used for collating       |
//!| 11     | $Extend   | 2K  | A directory: $ObjId, $Quota, $Reparse, $UsnJrnl        |
//!|        |           |     |                                                        |
//!| 12-15  | \<Unused> |     | Marked as in use but empty                             |
//!| 16-23  | \<Unused> |     | Marked as unused                                       |
//!|        |           |     |                                                        |
//!| Any    | $ObjId    | 2K  | Unique Ids given to every file                         |
//!| Any    | $Quota    | 2K  | Quota information                                      |
//!| Any    | $Reparse  | 2K  | Reparse point information                              |
//!| Any    | $UsnJrnl  | 2K  | Journalling of Encryption                              |
//!|        |           |     |                                                        |
//!| > 24   | A_File    |     | An ordinary file                                       |
//!| > 24   | A_Dir     |     | An ordinary directory                                  |
//!| ...    | ...       |     | ...                                                    |


/// Every attribute in every MFT record has a standard header.
/// The header stores information about the attribute's type, size, name (optional) and whether it is resident, or not.
/// <https://flatcap.github.io/linux-ntfs/ntfs/concepts/attribute_header.html>
//TODO: Attribute Headers
pub enum Flag {
    Compressed = 0x0001,
    Encrypted = 0x4000,
    Sparse = 0x8000,
}

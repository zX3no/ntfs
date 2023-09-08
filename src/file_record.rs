//! File Records are equivalent to inodes in Unix terminology.
//! The first FILE Record that describes a given file is called the Base FILE record and others are called Extension FILE Records.
//! A FILE Record is built up from a header, several variable length attributes and an end marker (0xFFFFFFFF).
//!
//! Header
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


pub struct FileRecord {}

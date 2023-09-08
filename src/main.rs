//! <https://flatcap.github.io/linux-ntfs/ntfs/index.html>
//!
//! <https://en.wikipedia.org/wiki/NTFS>
pub use std::{
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
};

pub use file_record::*;
pub use master_file_table::*;
pub use partition_boot_sector::*;

pub mod file_record;
pub mod master_file_table;
pub mod partition_boot_sector;

fn main() {
    let file = File::open("\\\\.\\C:").expect("Run as Admin");
    let mut reader = BufReader::new(file);

    let pbs = pbs(&mut reader);
    dbg!(&pbs);
}

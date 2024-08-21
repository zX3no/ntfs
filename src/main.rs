#![feature(read_buf, core_io_borrowed_buf)]
//! <https://flatcap.github.io/linux-ntfs/ntfs/index.html>
//!
//! <https://en.wikipedia.org/wiki/NTFS>
use ntfs::*;
use std::{
    fs::File,
    io::{BufReader, Seek, SeekFrom},
};

fn main() {
    let file = File::open("\\\\.\\C:").expect("Run as Admin");
    let mut reader = BufReader::new(file);
    let pbs = pbs(&mut reader);

    let mft_start_sector = pbs.mft_cluster_number * 8 * pbs.bytes_per_sector as u64;
    reader.seek(SeekFrom::Start(mft_start_sector)).unwrap();

    file_record(&mut reader);
}

use std::{fs::File, io::BufReader};

use partition_boot_sector::*;
mod partition_boot_sector;

fn main() {
    let file = File::open("\\\\.\\C:").expect("Run as Admin");
    let mut reader = BufReader::new(file);

    let pbs = pbs(&mut reader);
    dbg!(pbs);
}

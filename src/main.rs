use std::{
    fs::File,
    io::{BufReader, Read},
    str::from_utf8,
};

///512
#[rustfmt::skip]
const PARTITION_BOOT_SECTOR: usize = 3 + 8 + 2 + 1 + 2 + 3 + 2 + 1 + 2 + 2 + 2 + 4 +4 + 4 + 8 + 8 + 8 + 1 + 3 + 1 + 3 + 8 + 4 + 426 + 2;

fn main() {
    let file = File::open("\\\\.\\C:").unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = [0u8; PARTITION_BOOT_SECTOR];
    reader.read_exact(&mut buf).unwrap();

    //Causes execution to continue after the data structures in this boot sector.
    assert_eq!(buf[0], 0xeb);
    assert_eq!(buf[1], 0x52);
    assert_eq!(buf[2], 0x90);

    //This is the magic number that indicates this is an NTFS file system.
    let oem_id = from_utf8(&buf[3..11]).unwrap();
    assert_eq!(oem_id, "NTFS    ");

    //The number of bytes in a disk sector.
    let bytes_per_sector = u16::from_le_bytes([buf[11], buf[12]]);
    assert_eq!(bytes_per_sector, 0x0200);

    //The number of sectors in a cluster.
    //If the value is greater than 0x80, the amount of sectors is 2 to the power of the absolute value of considering this field to be negative.
    let sectors_per_cluster = buf[13];
    assert_eq!(sectors_per_cluster, 0x08);

    //Unused
    assert_eq!(buf[14], 0);
    assert_eq!(buf[15], 0);

    //Unused
    assert_eq!(buf[16], 0);
    assert_eq!(buf[17], 0);
    assert_eq!(buf[18], 0);

    //Unused
    assert_eq!(buf[19], 0);
    assert_eq!(buf[20], 0);

    //The type of drive. 0xF8 is used to denote a hard drive (in contrast to the several sizes of floppy).
    let media_descriptor = buf[21];
    assert_eq!(media_descriptor, 0xF8);

    //Unused
    assert_eq!(buf[22], 0);
    assert_eq!(buf[23], 0);

    //The number of disk sectors in a drive track.
    let sectors_per_track = u16::from_le_bytes([buf[24], buf[25]]);
    assert_eq!(sectors_per_track, 0x003F);

    //The number of heads on the drive.
    let number_of_heads = u16::from_le_bytes([buf[26], buf[27]]);
    assert_eq!(number_of_heads, 0x0ff);

    //The number of sectors preceding the partition.
    assert_eq!(28, 0x1C);
    let hidden_sectors = u32::from_le_bytes([buf[28], buf[29], buf[30], buf[31]]);
    println!("hidden_sectors: {}", hidden_sectors);

    //Unused
    assert_eq!(buf[32], 0);
    assert_eq!(buf[33], 0);
    assert_eq!(buf[34], 0);
    assert_eq!(buf[35], 0);

    let unused = u32::from_le_bytes([buf[36], buf[37], buf[38], buf[39]]);
    assert_eq!(unused, 0x00800080);

    //The partition size in sectors.
    assert_eq!(40, 0x28);
    let total_sectors = u64::from_le_bytes(buf[40..40 + 8].try_into().unwrap());
    println!("total_sectors: {}", total_sectors);

    //The cluster that contains the Master File Table
    assert_eq!(48, 0x30);
    let mft_cluster_number = u64::from_le_bytes(buf[48..48 + 8].try_into().unwrap());
    println!("mft cluster number: {:#x}", mft_cluster_number);

    //The cluster that contains a backup of the Master File Table
    assert_eq!(56, 0x38);
    let mft_mirr_cluster_number = u64::from_le_bytes(buf[56..56 + 8].try_into().unwrap());
    println!("mft mirr cluster number: {:#x}", mft_mirr_cluster_number);

    //A positive value denotes the number of clusters in a File Record Segment.
    //A negative value denotes the amount of bytes in a File Record Segment, in which case the size is 2 to the power of the absolute value.
    //(0xF6 = -10 → 210 = 1024).
    assert_eq!(64, 0x40);
    let n_per_file_record_segment = buf[64];
    assert_eq!(n_per_file_record_segment, 0xf6);

    //Unused
    assert_eq!(buf[65], 0);
    assert_eq!(buf[66], 0);
    assert_eq!(buf[67], 0);

    //A positive value denotes the number of clusters in an Index Buffer.
    //A negative value denotes the amount of bytes and it uses the same algorithm for negative numbers as the "Bytes or Clusters Per File Record Segment."
    assert_eq!(68, 0x44);
    let n_per_index_buffer = buf[68];
    assert_eq!(n_per_index_buffer, 0x01);

    //Unused
    assert_eq!(buf[69], 0);
    assert_eq!(buf[70], 0);
    assert_eq!(buf[71], 0);

    //Unused
    assert_eq!(72, 0x48);
    let volume_serial_number = u64::from_le_bytes(buf[72..72 + 8].try_into().unwrap());
    println!("volume serial number: {}", volume_serial_number);

    //Unused
    assert_eq!(0x50, 80);
    assert_eq!(buf[80], 0);
    assert_eq!(buf[81], 0);
    assert_eq!(buf[82], 0);
    assert_eq!(buf[83], 0);

    //The code that loads the rest of the operating system.
    //This is pointed to by the first 3 bytes of this sector.
    assert_eq!(84, 0x54);
    let _bootstrap_code = &buf[84..84 + 426];

    //This flag indicates that this is a valid boot sector.
    assert_eq!(510, 0x01FE);
    let end_of_sector = u16::from_le_bytes([buf[510], buf[511]]);
    assert_eq!(end_of_sector, 0xAA55);

    for byte in buf {
        print!("{byte:#x} ")
    }
}

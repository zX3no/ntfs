pub use attribute::*;
pub use file_record::*;
#[allow(unused_imports)]
pub use master_file_table::*;
pub use partition_boot_sector::*;

pub mod attribute;
pub mod file_record;
pub mod master_file_table;
pub mod partition_boot_sector;

#[inline]
pub fn u64(buf: &[u8]) -> u64 {
    #[cfg(debug_assertions)]
    return u64::from_le_bytes(buf[..8].try_into().unwrap());

    #[cfg(not(debug_assertions))]
    return unsafe { u64::from_le_bytes(buf[..8].try_into().unwrap_unchecked()) };
}

#[inline]
pub fn u32(buf: &[u8]) -> u32 {
    #[cfg(debug_assertions)]
    return u32::from_le_bytes(buf[..4].try_into().unwrap());

    #[cfg(not(debug_assertions))]
    return unsafe { u32::from_le_bytes(buf[..4].try_into().unwrap_unchecked()) };
}

#[inline]
pub fn u16(buf: &[u8]) -> u16 {
    #[cfg(debug_assertions)]
    return u16::from_le_bytes(buf[..2].try_into().unwrap());

    #[cfg(not(debug_assertions))]
    return unsafe { u16::from_le_bytes(buf[..2].try_into().unwrap_unchecked()) };
}

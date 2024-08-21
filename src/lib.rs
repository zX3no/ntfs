pub use file_record::*;
#[allow(unused_imports)]
pub use master_file_table::*;
pub use partition_boot_sector::*;
pub use attribute::*;

pub mod file_record;
pub mod master_file_table;
pub mod partition_boot_sector;
pub mod attribute;

#[inline]
pub fn u64_from_slice(buf: &[u8]) -> u64 {
    #[cfg(debug_assertions)]
    return u64::from_le_bytes(buf[..8].try_into().unwrap());

    #[cfg(not(debug_assertions))]
    return unsafe { u64::from_le_bytes(buf[..8].try_into().unwrap_unchecked()) };
}

#[inline]
pub fn u32_from_slice(buf: &[u8]) -> u32 {
    #[cfg(debug_assertions)]
    return u32::from_le_bytes(buf[..4].try_into().unwrap());

    #[cfg(not(debug_assertions))]
    return unsafe { u32::from_le_bytes(buf[..4].try_into().unwrap_unchecked()) };
}

#[inline]
pub fn u16_from_slice(buf: &[u8]) -> u16 {
    #[cfg(debug_assertions)]
    return u16::from_le_bytes(buf[..2].try_into().unwrap());

    #[cfg(not(debug_assertions))]
    return unsafe { u16::from_le_bytes(buf[..2].try_into().unwrap_unchecked()) };
}

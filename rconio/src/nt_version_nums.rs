//! https://docs.microsoft.com/zh-cn/windows/win32/sysinfo/operating-system-version

#[cfg(not(target_os = "windows"))]
compile_error!("This crate is for querying Windows, but the target isn't Windows.");

#[link(name = "ntdll")]
extern "system" {
    pub fn RtlGetNtVersionNumbers(major: *mut u32, minor: *mut u32, build: *mut u32);
}

#[derive(Debug)]
pub struct NTVersion {
    pub major: u32,
    pub minor: u32,
    pub build: u32,
}

pub fn get_version_numbers() -> NTVersion {
    let (mut major, mut minor, mut build) = (0u32, 0u32, 0u32);
    unsafe {
        self::RtlGetNtVersionNumbers(&mut major as _, &mut minor as _, &mut build as _);
    }
    NTVersion { major, minor, build }
}

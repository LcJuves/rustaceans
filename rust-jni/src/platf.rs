#![allow(dead_code)]

pub struct Platform;

impl Platform {
    pub const IS_UNIX: bool = cfg!(any(unix, target_os = "hermit"));

    pub const IS_WINDOWS: bool = cfg!(windows);

    pub const IS_LINUX: bool = cfg!(any(target_os = "linux", target_os = "l4re"));

    pub const IS_WASI: bool = cfg!(target_os = "wasi");

    pub const IS_ANDROID: bool = cfg!(target_os = "android");

    pub const IS_DRAGONFLY: bool = cfg!(target_os = "dragonfly");

    pub const IS_EMSCRIPTEN: bool = cfg!(target_os = "emscripten");

    pub const IS_FORTANIX_SGX: bool = cfg!(all(target_vendor = "fortanix", target_env = "sgx"));

    pub const IS_FREEBSD: bool = cfg!(target_os = "freebsd");

    pub const IS_FUCHSIA: bool = cfg!(target_os = "fuchsia");

    pub const IS_HAIKU: bool = cfg!(target_os = "haiku");

    pub const IS_ILLUMOS: bool = cfg!(target_os = "illumos");

    pub const IS_IOS: bool = cfg!(target_os = "ios");

    pub const IS_MACOS: bool = cfg!(target_os = "macos");

    pub const IS_NETBSD: bool = cfg!(target_os = "netbsd");

    pub const IS_OPENBSD: bool = cfg!(target_os = "openbsd");

    pub const IS_REDOX: bool = cfg!(target_os = "redox");

    pub const IS_SOLARIS: bool = cfg!(target_os = "solaris");

    pub const IS_VXWORKS: bool = cfg!(target_os = "vxworks");
}

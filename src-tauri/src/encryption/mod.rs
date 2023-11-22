#[cfg(target_os = "windows")]
pub mod platform_encryption {
    pub use super::encrypt_windows::*;
    pub use super::encrypt_shared::*;
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub mod platform_encryption {
    pub use super::encrypt_macos_ios::*;
    pub use super::encrypt_shared::*;
}

#[cfg(target_os = "android")]
pub mod platform_encryption {
    pub use super::encrypt_android::*;
    pub use super::encrypt_shared::*;
}

#[cfg(target_os = "linux")]
pub mod platform_encryption {
    pub use super::encrypt_linux::*;
    pub use super::encrypt_shared::*;
}

#[cfg(target_os = "windows")]
mod encrypt_windows;

#[cfg(any(target_os = "macos", target_os = "ios"))]
mod encrypt_macos_ios;

#[cfg(target_os = "android")]
mod encrypt_android;

#[cfg(target_os = "linux")]
mod encrypt_linux;

mod encrypt_shared;
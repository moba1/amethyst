use std::path;

#[cfg(target_os = "linux")]
pub fn storage() -> std::path::PathBuf {
    path::PathBuf::from("/var/lib/amethyst")
}

pub fn blob_storage() -> std::path::PathBuf {
    storage().join("blob")
}

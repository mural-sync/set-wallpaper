mod error;
use std::path::Path;

pub use error::Error;

#[cfg(target_os = "linux")]
mod linux;

pub fn set_wallpaper<P: AsRef<Path>>(wallpaper_path: P) -> Result<(), Error> {
    #[cfg(target_os = "linux")]
    linux::set_wallpaper(wallpaper_path)
}

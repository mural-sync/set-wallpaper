use std::path::Path;

use crate::Error;

pub fn set_wallpaper<P: AsRef<Path>>(wallpaper_path: P) -> Result<(), Error> {
    if std::process::Command::new("swwww")
        .arg("--version")
        .output()
        .is_ok()
    {
        swww_set_wallpaper(wallpaper_path)
    } else {
        Err(Error::WMNotSupported)
    }
}

fn swww_set_wallpaper<P: AsRef<Path>>(wallpaper_path: P) -> Result<(), Error> {
    let output = std::process::Command::new("swww")
        .arg("img")
        .arg("--transition-type")
        .arg("fade")
        .arg("--transition-bezier")
        .arg("0,0,1,1")
        .arg(wallpaper_path.as_ref())
        .output()
        .expect("function should only be called if swww is installed.");

    if !output.status.success() {
        Err(Error::SWWW {
            exit_code: output.status.code(),
            error_message: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    } else {
        Ok(())
    }
}

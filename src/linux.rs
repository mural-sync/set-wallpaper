use std::path::Path;

use crate::Error;

pub fn set_wallpaper<P: AsRef<Path>>(wallpaper_path: P) -> Result<(), Error> {
    let xdg_current_desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();

    if xdg_current_desktop == "GNOME" {
        gnome_set_wallpaper(wallpaper_path)
    } else if std::process::Command::new("swww")
        .arg("--version")
        .output()
        .is_ok()
    {
        swww_set_wallpaper(wallpaper_path)
    } else {
        Err(Error::WMNotSupported)
    }
}

fn gnome_set_wallpaper<P: AsRef<Path>>(wallpaper_path: P) -> Result<(), Error> {
    let output = std::process::Command::new("gsettings")
        .arg("set")
        .arg("org.gnome.desktop.background")
        .arg("picture-uri")
        .arg(format!("file://{}", wallpaper_path.as_ref().display()))
        .output()
        .expect("gsettings should always be installed on gnome.");

    if !output.status.success() {
        return Err(Error::Gnome {
            exit_code: output.status.code(),
            error_message: String::from_utf8_lossy(&output.stderr).to_string(),
        });
    }

    let output = std::process::Command::new("gsettings")
        .arg("set")
        .arg("org.gnome.desktop.background")
        .arg("picture-uri-dark")
        .arg(format!("file://{}", wallpaper_path.as_ref().display()))
        .output()
        .expect("gsettings should always be installed on gnome.");

    if !output.status.success() {
        return Err(Error::Gnome {
            exit_code: output.status.code(),
            error_message: String::from_utf8_lossy(&output.stderr).to_string(),
        });
    }

    Ok(())
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
        return Err(Error::SWWW {
            exit_code: output.status.code(),
            error_message: String::from_utf8_lossy(&output.stderr).to_string(),
        });
    }

    Ok(())
}

use std::{collections::HashSet, path::Path};

use crate::Error;

pub fn set_wallpaper<P: AsRef<Path>>(wallpaper_path: P) -> Result<(), Error> {
    let xdg_current_desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();

    match xdg_current_desktop.as_str() {
        "GNOME" => gnome_set_wallpaper(wallpaper_path),
        "XFCE" => xfce_set_wallpaper(wallpaper_path),
        _ if std::process::Command::new("swww")
            .arg("--version")
            .output()
            .is_ok() =>
        {
            swww_set_wallpaper(wallpaper_path)
        }
        _ => Err(Error::WMNotSupported),
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

fn xfce_set_wallpaper<P: AsRef<Path>>(wallpaper_path: P) -> Result<(), Error> {
    let monitors = String::from_utf8_lossy(
        &std::process::Command::new("xfconf-query")
            .args(["-c", "xfce4-desktop", "-p", "/backdrop/screen0", "-l"])
            .output()
            .map_err(|_| Error::XfConfNotInstalled)?
            .stdout,
    )
    .split_whitespace()
    .filter(|line| line.contains("workspace0"))
    .map(|line| {
        line.split('/')
            .nth(3)
            .expect("third element should always be monitor number")
            .to_string()
    })
    .collect::<HashSet<String>>();

    for monitor in &monitors {
        let output = std::process::Command::new("xfconf-query")
            .args([
                "-c",
                "xfce4-desktop",
                "-p",
                &format!("/backdrop/screen0/{}/workspace0/last-image", monitor),
                "-s",
            ])
            .arg(wallpaper_path.as_ref())
            .output()
            .expect("function should only be called if xfconf is installed");

        if !output.status.success() {
            return Err(Error::Xfce {
                exit_code: output.status.code(),
                error_message: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }
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

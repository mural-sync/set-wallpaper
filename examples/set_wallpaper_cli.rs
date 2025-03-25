use std::process::ExitCode;

fn main() -> ExitCode {
    let wallpaper_path = match std::env::args().nth(1) {
        Some(wallpaper_path) => wallpaper_path,
        None => {
            eprintln!("usage: set_wallpaper_cli <wallpaper_path>");
            return ExitCode::FAILURE;
        }
    };

    if let Err(e) = set_wallpaper::set_wallpaper(wallpaper_path) {
        eprintln!("error: {}", e);
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

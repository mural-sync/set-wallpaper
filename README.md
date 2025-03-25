# set-wallpaper

Set the desktop wallpaper using Rust.

This crate was primarily developed for `mural-sync`, but it is completely
standalone and can be used in any Rust project. However, the focus of the crate
will always be on supporting `mural-sync`'s needs. Since `mural-sync` currently
only supports Linux, `set-wallpaper` also supports Linux exclusively.

## Supported Systems

A mentioned in the introduction, `set-wallpaper` only supports Linux.
Furthermore, it only suppors Linux systems that either use GNOME or have
[swww](https://github.com/LGFae/swww) setup correctly. Support for other window
managers is planned, however, these are the only two environments I actively
use. Pull requests to add support for other environments (and of course any
other pull requests too) are welcome.

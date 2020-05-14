use crate::{parser, Error};
use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub const DESKTOP: &'static [u8] = b"XDG_DESKTOP_DIR";
pub const DOCUMENTS: &'static [u8] = b"XDG_DOCUMENTS_DIR";
pub const DOWNLOADS: &'static [u8] = b"XDG_DOWNLOAD_DIR";
pub const MUSIC: &'static [u8] = b"XDG_MUSIC_DIR";
pub const PICTURES: &'static [u8] = b"XDG_PICTURES_DIR";
pub const PUBLIC: &'static [u8] = b"XDG_PUBLICSHARE_DIR";
pub const TEMPLATES: &'static [u8] = b"XDG_TEMPLATES_DIR";
pub const VIDEOS: &'static [u8] = b"XDG_VIDEOS_DIR";

pub fn parse_file(mut callback: impl FnMut(&[u8], Option<PathBuf>) -> bool) -> Result<(), Error> {
    let home = home::home_dir().ok_or_else(|| crate::Error::NoHome)?;

    let dirs_file_path = std::env::var_os("XDG_CONFIG_HOME")
        .and_then(|e| {
            let mut path = PathBuf::from(e);
            if path.is_absolute() {
                path.push("user-dirs.dirs");
                Some(path)
            } else {
                None
            }
        })
        .unwrap_or_else(|| home.join(".config/user-dirs.dirs"));

    let dirs_file = std::fs::File::open(dirs_file_path)?;
    let mut dirs_file = BufReader::new(dirs_file);

    let mut line = Vec::new();
    while dirs_file.read_until(b'\n', &mut line)? != 0 {
        if let Some((key, val)) = parser::LineParser::new(&mut line).parse() {
            let val = std::str::from_utf8(val)?;

            let val = if val == "$HOME/" {
                None
            } else if val.starts_with("$HOME/") {
                Some(home.join(&val[6..]))
            } else {
                Some(val.into())
            };

            if !callback(key, val) {
                break;
            };
        }
        line.clear();
    }

    Ok(())
}

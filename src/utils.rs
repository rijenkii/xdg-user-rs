use crate::{parser, Error};
use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
};

// almost shamelessly stolen from dirs-sys
pub fn home_dir() -> Result<PathBuf, Error> {
    return std::env::var_os("HOME")
        .and_then(|h| if h.is_empty() { None } else { Some(h) })
        .or_else(|| unsafe { fallback() })
        .map(PathBuf::from)
        .ok_or_else(|| Error::NoHome);

    #[cfg(any(
        target_os = "android",
        target_os = "ios",
        target_os = "emscripten",
        target_os = "redox"
    ))]
    unsafe fn fallback() -> Option<std::ffi::OsString> {
        None
    }
    #[cfg(not(any(
        target_os = "android",
        target_os = "ios",
        target_os = "emscripten",
        target_os = "redox"
    )))]
    unsafe fn fallback() -> Option<std::ffi::OsString> {
        let amt = match libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) {
            n if n < 0 => 512 as usize,
            n => n as usize,
        };
        let mut buf = Vec::with_capacity(amt);
        let mut passwd = std::mem::zeroed();
        let mut result = std::ptr::null_mut();
        match libc::getpwuid_r(
            libc::getuid(),
            &mut passwd,
            buf.as_mut_ptr(),
            buf.capacity(),
            &mut result,
        ) {
            0 if !result.is_null() => {
                let ptr = passwd.pw_dir as *const _;
                let bytes = std::ffi::CStr::from_ptr(ptr).to_bytes();
                if bytes.is_empty() {
                    None
                } else {
                    Some(std::os::unix::ffi::OsStringExt::from_vec(bytes.to_vec()))
                }
            }
            _ => None,
        }
    }
}

pub const DESKTOP: &'static [u8] = b"XDG_DESKTOP_DIR";
pub const DOCUMENTS: &'static [u8] = b"XDG_DOCUMENTS_DIR";
pub const DOWNLOADS: &'static [u8] = b"XDG_DOWNLOAD_DIR";
pub const MUSIC: &'static [u8] = b"XDG_MUSIC_DIR";
pub const PICTURES: &'static [u8] = b"XDG_PICTURES_DIR";
pub const PUBLIC: &'static [u8] = b"XDG_PUBLICSHARE_DIR";
pub const TEMPLATES: &'static [u8] = b"XDG_TEMPLATES_DIR";
pub const VIDEOS: &'static [u8] = b"XDG_VIDEOS_DIR";

pub fn parse_file(mut callback: impl FnMut(&[u8], Option<PathBuf>) -> bool) -> Result<(), Error> {
    let home = home_dir()?;

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

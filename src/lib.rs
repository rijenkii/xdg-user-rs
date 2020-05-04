//! This simple crate allows you to get paths to "well known" user directories,
//! using [`xdg-user-dirs`][1]s `user-dirs.dirs` file.
//!
//! # Example
//!
//! ```rust
//! let dirs = xdg_user::UserDirs::new()?;
//! println!("Documents folder: {:?}", dirs.documents());
//! println!("Downloads folder: {:?}", dirs.downloads());
//! ```
//!
//! [1]: https://www.freedesktop.org/wiki/Software/xdg-user-dirs/

use std::path::{Path, PathBuf};

// almost shamelessly stolen from dirs-sys
fn home_dir() -> Result<PathBuf, Error> {
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

lazy_static::lazy_static! {
    static ref RE: regex::bytes::Regex =
        regex::bytes::Regex::new(
            r#"^[ \t]*([^ \t\\]+?)[ \t]*=[ \t]*"(?:\$HOME/|\$HOME/(.+?)|(/.+?))""#
        )
        .unwrap();
}

/// This crates main and only error type
#[derive(Debug)]
pub enum Error {
    /// Something went wrong while accessing the config file
    Io(std::io::Error),
    /// Unable to find the home directory
    NoHome,
    /// Error while parsing the config file
    Parse,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Io(e) => e.fmt(f),
            Self::NoHome => write!(f, "unable to find the home directory"),
            Self::Parse => write!(f, "error while parsing"),
        }
    }
}

impl std::error::Error for Error {}

/// This crates main and only struct, allows you to access the paths to all the
/// user directorss
#[derive(Debug)]
pub struct UserDirs {
    desktop: Option<PathBuf>,
    documents: Option<PathBuf>,
    downloads: Option<PathBuf>,
    music: Option<PathBuf>,
    pictures: Option<PathBuf>,
    public: Option<PathBuf>,
    templates: Option<PathBuf>,
    videos: Option<PathBuf>,
}

impl UserDirs {
    /// Attempts to read and parse the `${XDG_COFNIG_HOME}/user-dirs.dirs` file
    ///
    /// # Errors
    ///
    /// * Home folder was not found
    /// * Failed to read the `user-dirs.dirs` file
    /// * Failed to parse the `user-dirs.dirs` file
    pub fn new() -> Result<Self, Error> {
        use std::io::{BufRead, BufReader};

        let mut this = Self {
            desktop: None,
            documents: None,
            downloads: None,
            music: None,
            pictures: None,
            public: None,
            templates: None,
            videos: None,
        };

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
        let dirs_file = BufReader::new(dirs_file);

        for line in dirs_file.split(b'\n') {
            let mut line = line?;
            if RE.is_match(&line) {
                line.retain(|x| x != &b'\\');
                let caps = RE.captures(&line).unwrap();

                let val = if let Some(m) = caps.get(2) {
                    Some(home.join(std::str::from_utf8(m.as_bytes()).unwrap()))
                } else if let Some(m) = caps.get(3) {
                    Some(std::str::from_utf8(m.as_bytes()).unwrap().into())
                } else {
                    None
                };

                match caps.get(1).unwrap().as_bytes() {
                    b"XDG_DESKTOP_DIR" => this.desktop = val,
                    b"XDG_DOCUMENTS_DIR" => this.documents = val,
                    b"XDG_DOWNLOAD_DIR" => this.downloads = val,
                    b"XDG_MUSIC_DIR" => this.music = val,
                    b"XDG_PICTURES_DIR" => this.pictures = val,
                    b"XDG_PUBLICSHARE_DIR" => this.public = val,
                    b"XDG_TEMPLATES_DIR" => this.templates = val,
                    b"XDG_VIDEOS_DIR" => this.videos = val,
                    _ => {}
                }
            }
        }

        Ok(this)
    }

    /// Returns an absolute path to users desktop directory (`XDG_DESKTOP_DIR`),
    /// if found
    pub fn desktop(&self) -> Option<&Path> {
        self.desktop.as_ref().map(|p| p.as_path())
    }

    /// Returns an absolute path to users desktop directory (`XDG_DESKTOP_DIR`),
    /// if found
    pub fn documents(&self) -> Option<&Path> {
        self.documents.as_ref().map(|p| p.as_path())
    }

    /// Returns an absolute path to users downloads directory
    /// (`XDG_DOWNLOAD_DIR`), if found
    pub fn downloads(&self) -> Option<&Path> {
        self.downloads.as_ref().map(|p| p.as_path())
    }

    /// Returns an absolute path to users music directory (`XDG_MUSIC_DIR`),
    /// if found
    pub fn music(&self) -> Option<&Path> {
        self.music.as_ref().map(|p| p.as_path())
    }

    /// Returns an absolute path to users pictures directory
    /// (`XDG_PICTURES_DIR`), if found
    pub fn pictures(&self) -> Option<&Path> {
        self.pictures.as_ref().map(|p| p.as_path())
    }

    /// Returns an absolute path to users public share directory
    /// (`XDG_PUBLICSHARE_DIR`), if found
    pub fn public(&self) -> Option<&Path> {
        self.public.as_ref().map(|p| p.as_path())
    }

    /// Returns an absolute path to users templates directory
    /// (`XDG_TEMPLATES_DIR`), if found
    pub fn templates(&self) -> Option<&Path> {
        self.templates.as_ref().map(|p| p.as_path())
    }

    /// Returns an absolute path to users videos directory (`XDG_VIDEOS_DIR`),
    /// if found
    pub fn videos(&self) -> Option<&Path> {
        self.videos.as_ref().map(|p| p.as_path())
    }
}

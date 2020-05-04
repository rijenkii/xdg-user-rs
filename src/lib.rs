#![cfg(any(unix, target_os = "redox"))]

//! This simple crate allows you to get paths to well known user directories,
//! using [`xdg-user-dirs`][1]s `user-dirs.dirs` file.
//!
//! There are two ways of using this crate - with functions in the root of the
//! crate, or with the [`UserDirs`] struct. [`UserDirs`] will read and parse the
//! config file only once - when you call the [`UserDirs::new`] function.
//! Functions in the root will read and parse the config file EVERY TIME you
//! call them - so use them ONLY if you need to get one or two folders one or
//! two times.
//!
//! # Example
//!
//! ```rust
//! println!("Pictures folder: {:?}", xdg_user::pictures()?);
//! println!("Music folder:    {:?}", xdg_user::music()?);
//!
//! let dirs = xdg_user::UserDirs::new()?;
//! println!("Documents folder: {:?}", dirs.documents());
//! println!("Downloads folder: {:?}", dirs.downloads());
//! ```
//!
//! [1]: https://www.freedesktop.org/wiki/Software/xdg-user-dirs/

use std::path::{Path, PathBuf};

mod parser;
mod utils;

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

impl From<std::str::Utf8Error> for Error {
    fn from(_: std::str::Utf8Error) -> Self {
        Self::Parse
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
/// user directories
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

impl std::fmt::Debug for UserDirs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UserDirs").finish()
    }
}

impl UserDirs {
    /// Attempts to read and parse the `${XDG_COFNIG_HOME}/user-dirs.dirs` file
    pub fn new() -> Result<Self, Error> {
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

        utils::parse_file(|key, val| {
            match key {
                utils::DESKTOP => this.desktop = val,
                utils::DOCUMENTS => this.documents = val,
                utils::DOWNLOADS => this.downloads = val,
                utils::MUSIC => this.music = val,
                utils::PICTURES => this.pictures = val,
                utils::PUBLIC => this.public = val,
                utils::TEMPLATES => this.templates = val,
                utils::VIDEOS => this.videos = val,
                _ => {}
            }
            true
        })?;

        Ok(this)
    }

    /// Returns an absolute path to users desktop directory (`XDG_DESKTOP_DIR`),
    /// if found
    pub fn desktop(&self) -> Option<&Path> {
        self.desktop.as_deref()
    }

    /// Returns an absolute path to users documents directory
    /// (`XDG_DOCUMENTS_DIR`), if found
    pub fn documents(&self) -> Option<&Path> {
        self.documents.as_deref()
    }

    /// Returns an absolute path to users downloads directory
    /// (`XDG_DOWNLOAD_DIR`), if found
    pub fn downloads(&self) -> Option<&Path> {
        self.downloads.as_deref()
    }

    /// Returns an absolute path to users music directory (`XDG_MUSIC_DIR`),
    /// if found
    pub fn music(&self) -> Option<&Path> {
        self.music.as_deref()
    }

    /// Returns an absolute path to users pictures directory
    /// (`XDG_PICTURES_DIR`), if found
    pub fn pictures(&self) -> Option<&Path> {
        self.pictures.as_deref()
    }

    /// Returns an absolute path to users public share directory
    /// (`XDG_PUBLICSHARE_DIR`), if found
    pub fn public(&self) -> Option<&Path> {
        self.public.as_deref()
    }

    /// Returns an absolute path to users templates directory
    /// (`XDG_TEMPLATES_DIR`), if found
    pub fn templates(&self) -> Option<&Path> {
        self.templates.as_deref()
    }

    /// Returns an absolute path to users videos directory (`XDG_VIDEOS_DIR`),
    /// if found
    pub fn videos(&self) -> Option<&Path> {
        self.videos.as_deref()
    }
}

fn read_single(env: &[u8]) -> Result<Option<PathBuf>, Error> {
    let mut ret = None;
    utils::parse_file(|key, val| {
        if key == env {
            ret = val;
            false
        } else {
            true
        }
    })?;

    Ok(ret)
}

/// Returns an absolute path to users desktop directory (`XDG_DESKTOP_DIR`),  if
/// found
///
/// # Warning
///
/// This function will parse the `user-dirs.dirs` file every time it's called,
/// so if you need paths to multiple different directories - consider using
/// [`UserDirs`] instead
pub fn desktop() -> Result<Option<PathBuf>, Error> {
    read_single(utils::DESKTOP)
}

/// Returns an absolute path to users documents directory (`XDG_DOCUMENTS_DIR`),
/// if found
///
/// # Warning
///
/// This function will parse the `user-dirs.dirs` file every time it's called,
/// so if you need paths to multiple different directories - consider using
/// [`UserDirs`] instead
pub fn documents() -> Result<Option<PathBuf>, Error> {
    read_single(utils::DOCUMENTS)
}

/// Returns an absolute path to users downloads directory (`XDG_DOWNLOAD_DIR`),
/// if found
///
/// # Warning
///
/// This function will parse the `user-dirs.dirs` file every time it's called,
/// so if you need paths to multiple different directories - consider using
/// [`UserDirs`] instead
pub fn downloads() -> Result<Option<PathBuf>, Error> {
    read_single(utils::DOWNLOADS)
}

/// Returns an absolute path to users music directory (`XDG_MUSIC_DIR`),  if
/// found
///
/// # Warning
///
/// This function will parse the `user-dirs.dirs` file every time it's called,
/// so if you need paths to multiple different directories - consider using
/// [`UserDirs`] instead
pub fn music() -> Result<Option<PathBuf>, Error> {
    read_single(utils::MUSIC)
}

/// Returns an absolute path to users pictures directory (`XDG_PICTURES_DIR`),
/// if found
///
/// # Warning
///
/// This function will parse the `user-dirs.dirs` file every time it's called,
/// so if you need paths to multiple different directories - consider using
/// [`UserDirs`] instead
pub fn pictures() -> Result<Option<PathBuf>, Error> {
    read_single(utils::PICTURES)
}

/// Returns an absolute path to users public share directory
/// (`XDG_PUBLICSHARE_DIR`), if found
///
/// # Warning
///
/// This function will parse the `user-dirs.dirs` file every time it's called,
/// so if you need paths to multiple different directories - consider using
/// [`UserDirs`] instead
pub fn public() -> Result<Option<PathBuf>, Error> {
    read_single(utils::PUBLIC)
}

/// Returns an absolute path to users templates directory (`XDG_TEMPLATES_DIR`),
/// if found
///
/// # Warning
///
/// This function will parse the `user-dirs.dirs` file every time it's called,
/// so if you need paths to multiple different directories - consider using
/// [`UserDirs`] instead
pub fn templates() -> Result<Option<PathBuf>, Error> {
    read_single(utils::TEMPLATES)
}

/// Returns an absolute path to users videos directory (`XDG_VIDEOS_DIR`), if
/// found
///
/// # Warning
///
/// This function will parse the `user-dirs.dirs` file every time it's called,
/// so if you need paths to multiple different directories - consider using
/// [`UserDirs`] instead
pub fn videos() -> Result<Option<PathBuf>, Error> {
    read_single(utils::VIDEOS)
}

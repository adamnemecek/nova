// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::{self, Read as _};

/// A virtual file system context.
///
/// Each context can have any number of real file system paths mounted to
/// virtual file system paths. Virtual file system operations are mapped to the
/// real file system according to which mount point matches the virtual path.
///
/// Multiple file system paths can be mounted to the same virtual file system
/// path. When reading a file, the virtual file system will search all matching
/// mount points in reverse of the order in which they were added, reading from
/// the first file that exists on disk. When writing a file, the virtual file
/// system will write to the last matching mount point.
///
/// For example, both an application data directory and a user data directory
/// could be mounted to the same virtual path. If the user directory is mounted
/// second, then the files in that directory will override the files in the
/// application directory. Additionally, new files will be written to the user
/// directory, not the application directory.
///
/// This structure is cloneable, and all clones refer to the same virtual
/// file system context.
#[derive(Default, Clone)]
pub struct Context {
  mounts: Arc<RwLock<Vec<Mount>>>,
}

impl Context {
  /// Creates a new, empty virtual file system context.
  pub fn new() -> Self {
    Self { mounts: Arc::new(RwLock::new(Vec::new())) }
  }

  /// Mounts a file system path to a virtual file system path.
  pub fn mount(&self, path: impl Into<PathBuf>, fs_path: impl Into<FsPathBuf>) {
    let path = path.into();

    assert!(path.is_absolute(), "virtual file system mount path must be absolute");

    let mut fs_path = fs_path.into();

    if !fs_path.is_absolute() {
      // Try to make the `fs_path` absolute, relative to the current working
      // directory.
      if let Ok(cwd) = env::current_dir() {
        fs_path = cwd.join(fs_path);
      }
    }

    self.mounts.write().push(Mount { path, fs_path });
  }

  /// Reads the contents of a file in the virtual file system to a `String`.
  ///
  /// This function searches for the file in matching mount points in reverse
  /// of the order they were added.
  pub fn read_to_string(&self, path: impl AsRef<Path>) -> io::Result<String> {
    let mut string = String::new();
    let mut file = self.open(path)?;

    file.read_to_string(&mut string)?;

    Ok(string)
  }

  /// Opens a file in the virtual file system.
  ///
  /// This function searches for the file in matching mount points in reverse
  /// of the order they were added.
  pub fn open(&self, path: impl AsRef<Path>) -> io::Result<File> {
    let path = path.as_ref();

    self
      .relative_to_each_mount(path, |fs_path| {
        match File::open(&fs_path) {
          Ok(file) => return Some(Ok(file)),

          Err(err) => {
            if err.kind() != io::ErrorKind::NotFound {
              return Some(Err(err));
            }
          }
        }

        None
      })
      .unwrap_or_else(|| Err(io::ErrorKind::NotFound.into()))
  }

  /// Creates a file in the virtual file system.
  ///
  /// This function creates the file in the first mount point that matches, in
  /// reverse of the order in which they were added. If one or more parent
  /// directories of the file do not exist, they will also be created.
  pub fn create(&self, path: impl AsRef<Path>) -> io::Result<File> {
    let path = path.as_ref();

    self
      .relative_to_each_mount(path, |fs_path| {
        match create_all(&fs_path) {
          Ok(file) => return Some(Ok(file)),

          Err(err) => {
            if err.kind() != io::ErrorKind::NotFound {
              return Some(Err(err));
            }
          }
        }

        None
      })
      .unwrap_or_else(|| Err(io::ErrorKind::NotFound.into()))
  }

  /// Calls `func` once for each mount point that matches `path`, providing an
  /// `OsString` containing the corresponding real file system path.
  ///
  /// If `func` returns a `Some(T)`, this function will return that result
  /// immediately without calling `func` again.
  fn relative_to_each_mount<T>(
    &self,
    path: &Path,
    mut func: impl FnMut(&mut OsString) -> Option<T>,
  ) -> Option<T> {
    let mut fs_path = OsString::new();

    for mount in self.mounts.read().iter().rev() {
      let relative = match path.strip_prefix(&mount.path) {
        Some(path) => path,
        None => continue,
      };

      fs_path.push(&mount.fs_path);
      fs_path.push("/");
      fs_path.push(&relative);

      if let Some(result) = func(&mut fs_path) {
        return Some(result);
      }

      fs_path.clear();
    }

    None
  }
}

/// A mount point in `Context`.
struct Mount {
  /// The virtual file system path.
  path: PathBuf,
  /// The real file system path.
  fs_path: FsPathBuf,
}

/// Creates a new `File` at the given path, creating all parent directories that
/// do not already exist.
fn create_all(path: impl AsRef<FsPath>) -> io::Result<File> {
  let path = path.as_ref();

  // Try to create the file first, because usually all the directories *will*
  // already exist.
  match File::create(path) {
    Ok(file) => Ok(file),

    Err(err) => match err.kind() {
      // If any of the parent directories were not found, create them all
      // recursively.
      io::ErrorKind::NotFound => match path.parent() {
        Some(parent) => {
          std::fs::create_dir_all(parent)?;

          File::create(path)
        }

        None => Err(err),
      },

      // Return any other kind of error immediately.
      _ => Err(err),
    },
  }
}

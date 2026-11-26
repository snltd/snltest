use camino::{Utf8Path, Utf8PathBuf};
use camino_tempfile_ext::prelude::*;
use flate2::read::GzDecoder;
use std::io::Read;
use std::{env, fs};

/// Assert on the number of files in a directory.
pub trait ContainsFiles {
    /// Returns `true` if `self` is a directory containing exactly `count` entries, and `false`
    /// otherwise (including when it isn't a directory).
    ///
    /// # Panics
    ///
    /// Panics if the directory cannot be read.
    ///
    /// # Examples
    ///
    /// ```
    /// use camino::Utf8Path;
    /// use snltest::ContainsFiles;
    ///
    /// assert!(Utf8Path::new("src").contains_files(1));
    /// ```
    fn contains_files(&self, count: usize) -> bool;
}

impl ContainsFiles for Utf8Path {
    fn contains_files(&self, count: usize) -> bool {
        match self.is_dir() {
            true => {
                let files_in_dir = self.read_dir().expect("cannot read dir").count();
                files_in_dir == count
            }
            false => false,
        }
    }
}

impl ContainsFiles for Utf8TempDir {
    fn contains_files(&self, count: usize) -> bool {
        self.path().contains_files(count)
    }
}

/// Returns a path to a temp directory, paired in a tuple with a Utf8TempDir object which
/// keeps the directory alive whilst it is in scope. You will likely wish to disregard the first
/// element of the tuple. The file_name() of the returned directory is `dir_name`.
///
/// The directory will have under it all files given in `files`. If a file ends with `/`, a
/// directory is made. Otherwise you get a plain file, the content of which is its relative path,
/// as a String.
///
/// # Examples
///
/// ```
/// use snltest::fixture_dir;
/// let (x, path) = fixture_dir("my-test", vec!["file-a", "dir-b/file-b", "dir-c/"]);
/// assert!(path.exists());
/// assert!(path.join("file-a").exists());
/// assert!(path.join("dir-c").is_dir());
/// assert_eq!("dir-b/file-b".to_string(), std::fs::read_to_string(path.join("dir-b").join("file-b")).unwrap());
/// ```
pub fn fixture_dir(dir_name: &str, files: Vec<&str>) -> (Utf8TempDir, Utf8PathBuf) {
    let temp = Utf8TempDir::new().expect("failed to create temp dir");
    let dir = temp.child(dir_name);
    dir.create_dir_all().expect("failed to create subdirectory");

    for file in files {
        let file_path = dir.child(file);
        if file.ends_with("/") {
            file_path
                .create_dir_all()
                .expect("failed to make directory");
        } else {
            file_path.write_str(file).expect("failed to create file");
        }
    }

    (temp, dir.canonicalize_utf8().unwrap())
}

/// Returns the number of entries at the top level of dir.
///
/// # Examples
///
/// ```
/// use camino::Utf8PathBuf;
/// use snltest::files_in_dir;
/// let n = files_in_dir(&Utf8PathBuf::from("/"));
/// assert!(n > 4 && n < 20);
/// ```
pub fn files_in_dir(dir: &Utf8Path) -> usize {
    dir.read_dir().unwrap().collect::<Vec<_>>().len()
}

#[doc(hidden)]
pub fn fixture_impl(crate_root: &str, path: impl AsRef<Utf8Path>) -> Utf8PathBuf {
    Utf8PathBuf::from(crate_root)
        .join("tests")
        .join("resources")
        .join(path)
}

/// Returns a Utf8PathBuf relative to the calling crate's `tests/resources` directory.
///
/// The path is built from the *caller's* `CARGO_MANIFEST_DIR`, so this works from any crate
/// which depends on this one.
///
/// # Examples
///
/// ```ignore
/// let p = fixture!("sample.flac");
/// ```
#[macro_export]
macro_rules! fixture {
    ($path:expr) => {
        $crate::fixture_impl(env!("CARGO_MANIFEST_DIR"), $path)
    };
}

#[doc(hidden)]
pub fn load_fixture_impl(crate_root: &str, path: impl AsRef<Utf8Path>) -> String {
    let path = fixture_impl(crate_root, path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read fixture '{}': {e}", path))
}

/// Returns the contents of the given fixture file, as a String. Fixtures are assumed to be in
/// the calling crate's `{CARGO_MANIFEST_DIR}/test/resources` directory.
///
/// # Examples
///
/// ```ignore
/// let info = load_fixture!("sample.txt");
/// assert_eq!("contents of file".to_string(), info);
/// ```
#[macro_export]
macro_rules! load_fixture {
    ($path:expr) => {
        $crate::load_fixture_impl(env!("CARGO_MANIFEST_DIR"), $path)
    };
}

#[doc(hidden)]
pub fn load_fixture_gz_impl(crate_root: &str, path: impl AsRef<Utf8Path>) -> String {
    let path = fixture_impl(crate_root, path);
    let file =
        fs::File::open(&path).unwrap_or_else(|e| panic!("failed to open fixture '{}': {e}", path));

    let mut contents = String::new();
    GzDecoder::new(file)
        .read_to_string(&mut contents)
        .unwrap_or_else(|e| panic!("failed to decompress fixture '{}': {e}", path));
    contents
}

/// Returns the contents of the given gzip-compressed fixture file, as a String. Fixtures are
/// assumed to be in the calling crate's `{CARGO_MANIFEST_DIR}/test/resources` directory.
///
/// # Examples
///
/// ```ignore
/// let info = load_fixture!("sample.txt.gz");
/// assert_eq!("contents of file".to_string(), info);
/// ```
#[macro_export]
macro_rules! load_fixture_gz {
    ($path:expr) => {
        $crate::load_fixture_gz_impl(env!("CARGO_MANIFEST_DIR"), $path)
    };
}

/// Returns the current working directory as a Utf8PathBuf.
pub fn cwd() -> Utf8PathBuf {
    Utf8PathBuf::from_path_buf(env::current_dir().unwrap()).unwrap()
}

/// Returns the root of the repo in which code is running, based on the location of Cargo.lock.
pub fn repo_root() -> Utf8PathBuf {
    let crate_root =
        Utf8PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("cannot get CARGO_MANIFEST_DIR"));

    crate_root
        .ancestors()
        .find(|dir| dir.join("Cargo.lock").exists())
        .expect("cannot find repo_root")
        .to_path_buf()
}

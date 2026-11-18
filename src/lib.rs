use camino::{Utf8Path, Utf8PathBuf};
use camino_tempfile_ext::prelude::*;
use flate2::read::GzDecoder;
use std::io::Read;
use std::{env, fs};

pub trait ContainsFiles {
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

pub fn fixture_dir(dir_name: &str, files: Vec<&str>) -> (Utf8TempDir, Utf8PathBuf) {
    let temp = Utf8TempDir::new().expect("failed to create temp dir");
    let dir = temp.child(dir_name);
    dir.create_dir_all().expect("failed to create subdirectory");

    for file in files {
        let file_path = dir.child(file);
        file_path.write_str(file).expect("failed to create file");
    }

    (temp, dir.canonicalize_utf8().unwrap())
}

/// Returns the number of entries in dir
pub fn files_in_dir(dir: &Utf8Path) -> usize {
    dir.read_dir().unwrap().collect::<Vec<_>>().len()
}

pub fn fixture_impl(manifest_dir: &str, path: impl AsRef<Utf8Path>) -> Utf8PathBuf {
    Utf8PathBuf::from(manifest_dir)
        .join("tests")
        .join("resources")
        .join(path)
}

#[macro_export]
macro_rules! fixture {
    ($path:expr) => {
        $crate::fixture_impl(env!("CARGO_MANIFEST_DIR"), $path)
    };
}

pub fn load_fixture_impl(manifest_dir: &str, path: impl AsRef<Utf8Path>) -> String {
    let path = fixture_impl(manifest_dir, path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read fixture '{}': {e}", path))
}

#[macro_export]
macro_rules! load_fixture {
    ($path:expr) => {
        $crate::load_fixture_impl(env!("CARGO_MANIFEST_DIR"), $path)
    };
}

pub fn load_fixture_gz_impl(manifest_dir: &str, path: impl AsRef<Utf8Path>) -> String {
    let path = fixture_impl(manifest_dir, path);
    let file =
        fs::File::open(&path).unwrap_or_else(|e| panic!("failed to open fixture '{}': {e}", path));

    let mut contents = String::new();
    GzDecoder::new(file)
        .read_to_string(&mut contents)
        .unwrap_or_else(|e| panic!("failed to decompress fixture '{}': {e}", path));
    contents
}

#[macro_export]
macro_rules! load_fixture_gz {
    ($path:expr) => {
        $crate::load_fixture_gz_impl(env!("CARGO_MANIFEST_DIR"), $path)
    };
}

/// Return a handle to a test dir containing files with the given names,
/// along with its canonical path. (MacOS puts /tmp behind a /private symlink,
/// so tests that compare paths need both.)
pub fn tmpdir_with_files(fnames: Vec<&str>) -> (Utf8TempDir, Utf8PathBuf) {
    let src_dir = Utf8TempDir::new().unwrap();
    for fname in fnames {
        let f = src_dir.child(fname);
        f.write_str(fname).unwrap();
    }

    let canonical = Utf8PathBuf::from_path_buf(src_dir.path().canonicalize().unwrap())
        .expect("canonicalized path should still be UTF-8");

    (src_dir, canonical)
}

pub fn cwd() -> Utf8PathBuf {
    Utf8PathBuf::from_path_buf(env::current_dir().unwrap()).unwrap()
}

pub fn repo_root() -> Utf8PathBuf {
    let manifest_dir =
        Utf8PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("cannot get CARGO_MANIFEST_DIR"));

    manifest_dir
        .ancestors()
        .find(|dir| dir.join("Cargo.lock").exists())
        .expect("cannot find repo_root")
        .to_path_buf()
}

use camino::{Utf8Path, Utf8PathBuf};
use std::fs;

pub fn fixture_impl(manifest_dir: &str, path: impl AsRef<Utf8Path>) -> Utf8PathBuf {
    Utf8PathBuf::from(manifest_dir)
        .join("tests")
        .join("resources")
        .join(path)
}

pub fn load_fixture_impl(manifest_dir: &str, path: impl AsRef<Utf8Path>) -> String {
    let path = fixture_impl(manifest_dir, path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read fixture '{}': {e}", path))
}

#[macro_export]
macro_rules! fixture {
    ($path:expr) => {
        $crate::fixture_impl(env!("CARGO_MANIFEST_DIR"), $path)
    };
}

#[macro_export]
macro_rules! load_fixture {
    ($path:expr) => {
        $crate::load_fixture_impl(env!("CARGO_MANIFEST_DIR"), $path)
    };
}

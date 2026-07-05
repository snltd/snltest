use camino::{Utf8Path, Utf8PathBuf};
use std::{fs, sync::LazyLock};

static FIXTURES: LazyLock<Utf8PathBuf> = LazyLock::new(|| {
    Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("resources")
});

pub fn fixture(path: impl AsRef<Utf8Path>) -> Utf8PathBuf {
    FIXTURES.join(path)
}

pub fn load_fixture(path: impl AsRef<Utf8Path>) -> String {
    let path = fixture(path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read fixture '{}': {e}", path))
}

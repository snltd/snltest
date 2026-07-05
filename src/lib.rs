pub fn fixture_impl(manifest_dir: &str, path: impl AsRef<camino::Utf8Path>) -> camino::Utf8PathBuf {
    camino::Utf8PathBuf::from(manifest_dir)
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

#[macro_export]
macro_rules! load_fixture {
    ($path:expr) => {
        let path = $crate::fixture!(path);
        std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("failed to read fixture '{}': {e}", path))
    };
}

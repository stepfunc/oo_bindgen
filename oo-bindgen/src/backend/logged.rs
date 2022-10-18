/// Normal filesystem copy + logging
pub fn copy<P: AsRef<std::path::Path>, Q: AsRef<std::path::Path>>(
    from: P,
    to: Q,
) -> std::io::Result<u64> {
    tracing::info!(
        "Copy: {} to {}",
        from.as_ref().display(),
        to.as_ref().display()
    );
    std::fs::copy(from, to)
}

pub fn create_dir_all<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<()> {
    tracing::info!("Create dir: {}", path.as_ref().display());
    std::fs::create_dir_all(path)
}

pub fn remove_dir_all<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<()> {
    tracing::info!("Remove dir: {}", path.as_ref().display());
    std::fs::remove_dir_all(path)
}

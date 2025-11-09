use std::{fs, io, path::Path};

use fs_extra::dir::{copy as copy_dir, CopyOptions};

/// Checks whether a directory is empty.
///
/// Consider the following scenario:
///
/// Suppose we need to build mruby. Although the mruby directory exists,
/// it may contain no files at all.
///
/// In such cases, the build will inevitably fail.
/// Knowing in advance whether the mruby directory is empty is crucial to
/// ensuring the correctness of the build process.
fn dir_is_not_empty<P: AsRef<Path>>(path: P) -> bool {
    fs::read_dir(path)
        .ok()
        .and_then(|mut entries| entries.next())
        .is_some()
}

fn try_to_copy_dir(from: &Path, to: &Path) -> io::Result<u64> {
    let opts = CopyOptions::new()
        .overwrite(true)
        .copy_inside(true);

    copy_dir(from, to, &opts).map_err(|e| {
        let err_msg = format!(
            "Failed to copy directory.
            src dir: {from:?}, target dir: {to:?}
            Err: {e}"
        );
        io::Error::other(err_msg)
    })
}

/// Copies the "/path/to/mruby-src-dir" to "work_dir/mruby"
///
/// # Example
///
/// ```ignore
/// let work_dir: PathBuf = env::var("OUT_DIR")?.into();
/// let src_dir = Path::new("/tmp/mruby-3.4")
///
/// // src_dir => "{work_dir}/mruby"
/// copy_to_mruby_dir(src_dir, &work_dir)?;
/// ```
///
/// # Returns
///
/// This function returns either `io::ErrorKind::Other(msg)` or `Ok(u64)`.
///
/// When returning `Ok(n)`, `n` represents the amount of data transferred (in
/// bytes).
///
/// - If `n = 0`, it means an empty directory was copied.
/// - If `n >= 1`, it indicates that a non-empty directory was transferred
///   (copied).
///
/// > Note: This function uses the special return value `Ok(1)` to indicate that
/// > `target_dir` already exists and is not empty, but the function returned
/// > early without performing any copy operation.
pub(crate) fn copy_to_mruby_dir(src_dir: &Path, work_dir: &Path) -> io::Result<u64> {
    let target_dir = work_dir.join("mruby");

    if target_dir.exists() {
        if dir_is_not_empty(&target_dir) {
            println!("cargo:warning=Dir exists: {target_dir:?}");
            // Do not use Ok(0) here.
            // Later, we might need to use Ok(n) to determine the number of bytes copied.
            // In some cases, OK(0) (i.e., copying an empty directory) may be considered a
            // failure. Therefore, we use `Ok(1)`.
            return Ok(1);
        }
        // An existing but empty mruby directory is a clear sign of a failed or
        // incomplete build.
        // Perform cleanup at this stage to prevent issues with subsequent `dir::copy`
        // operations.
        println!("cargo:warning=Removing the existing directory: {target_dir:?}");
        fs::remove_dir_all(&target_dir)?
    }
    println!("cargo:warning=src dir: {src_dir:?}, target dir: {target_dir:?}");

    // There's no need to check whether `src_dir` is an empty directory here;
    // instead, let the final caller handle the Err(_) & Ok(0).
    try_to_copy_dir(src_dir, &target_dir)
}

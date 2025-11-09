use std::{fs, io, path::Path};

use fs_extra::{
    dir::{copy as copy_dir, CopyOptions},
    error::Result as FsResult,
};

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

/// Recursively copy a single directory. (behavior similar to `cp -rf` on Unix)
/// See also: [fs_extra::dir::copy]
///
/// # Returns
///
/// This function returns [`fs_extra::error::Result<u64>`].
///
/// When returning `Ok(n)`, `n` represents the amount of data transferred (in
/// bytes).
///
/// - If `n = 0`, it means an empty directory was copied.
/// - If `n >= 1`, it indicates that a non-empty directory was transferred
///   (copied).
fn try_to_copy_dir(from: &Path, to: &Path) -> FsResult<u64> {
    let opts = CopyOptions::new()
        .overwrite(true)
        .copy_inside(true);

    copy_dir(from, to, &opts)
}

/// Copies the "/path/to/mruby-src-dir" to "{work_dir}/mruby"
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
/// # Notes
///
/// > target_dir: "{work_dir}/mruby"
///
/// - If `target_dir` exists and contains files, returns `Ok(())`.
/// - If the total size of data copied from `src_dir` to `target_dir` is zero
///   bytes, returns `Err`.
pub(crate) fn copy_to_mruby_dir(src_dir: &Path, work_dir: &Path) -> FsResult<()> {
    let target_dir = work_dir.join("mruby");

    if target_dir.exists() {
        if dir_is_not_empty(&target_dir) {
            eprintln!("[INFO] Dir exists: {target_dir:?}");
            return Ok(());
        }

        // An existing but empty mruby directory is a clear sign of a failed or
        // incomplete build.
        // Perform cleanup at this stage to prevent issues with subsequent `dir::copy`
        // operations.
        println!("cargo:warning=Removing the existing directory: {target_dir:?}");
        fs::remove_dir_all(&target_dir)?
    }
    eprintln!("[INFO] src dir: {src_dir:?}, target dir: {target_dir:?}");

    let copied = try_to_copy_dir(src_dir, &target_dir)?;

    if copied == 0 {
        let err_msg = r#"No files were copied into the `mruby` directory.
    Please make sure that mruby src dir is not an empty directory."#;
        let fs_err = io::Error::other(err_msg).into();
        return Err(fs_err);
    }

    Ok(())
}

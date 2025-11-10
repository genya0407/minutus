use std::{borrow::Cow, fs, io, path::Path};

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
///
/// - `min_required_size` defines the minimum required size of copied data, in
///   bytes.
///   - If the actual copied size is too small (e.g., < 4 KiB), it is assumed
///     that `src_dir` does not contain a complete mruby source tree. To disable
///     this check, set `min_required_size` to 0.
pub(crate) fn copy_to_mruby_dir(
    src_dir: &Path,
    work_dir: &Path,
    min_required_size: u64,
) -> FsResult<()> {
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

    if copied < min_required_size {
        let no_files_err = r#"No files were copied into the `mruby` directory.
    Please make sure that mruby src dir is not an empty directory."#;

        let err_msg = match copied {
            0 => no_files_err.into(),
            n => {
                let msg = format!(
                    "The copied data size from {src_dir:?} appears to be too small.
        > Expected at least: {min_required_size} bytes;
        > Actual: {n} bytes.
        Please verify that the directory contains a valid mruby source tree.
        If you suspect this is a bug, feel free to report an issue."
                );
                Cow::from(msg)
            }
        };

        let fs_err = io::Error::other(err_msg).into();
        return Err(fs_err);
    }

    Ok(())
}

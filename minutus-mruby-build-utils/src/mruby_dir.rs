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
    let opts = CopyOptions::new().overwrite(true);

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
pub(crate) fn copy_to_mruby_dir(src_dir: &Path, work_dir: &Path) -> io::Result<u64> {
    let target_dir = work_dir.join("mruby");

    if target_dir.exists() {
        if dir_is_not_empty(&target_dir) {
            println!("cargo:warning=Dir exists: {target_dir:?}");
            return Ok(0);
        }
        // An existing but empty mruby directory is a clear sign of a failed or
        // incomplete build.
        // Perform cleanup at this stage to prevent issues with subsequent `fs::rename`
        // operations.
        fs::remove_dir_all(&target_dir)?
    }
    println!("cargo:warning=src dir: {src_dir:?}, target dir: {target_dir:?}");

    let status = try_to_copy_dir(src_dir, work_dir);

    let src_dir_name = Path::new(src_dir)
        .file_name()
        .ok_or_else(|| io::Error::other("Failed to get src_dir.file_name"))?;

    // In UNIX-like `sh` ,
    // running `cp -r /path/to/xx /out/mruby` behaves as follows:
    //
    // - 1. If the `mruby` directory does not exist, `xx` will be renamed to `mruby`
    //   implicitly.
    // - 2. If the `mruby` directory exists, the `xx` directory will be copied into
    //   `/out/mruby/xx`.
    //
    // The behavior of `fs_extra::dir::copy` is closer to the 2nd case:
    // it assumes the destination directory already exists.
    // Therefore, manual renaming is required.
    //
    // In other words:
    // `dir::copy("/tmp/xx", "/out"); rename("/out/xx", "/out/mruby");`
    fs::rename(work_dir.join(src_dir_name), target_dir) //
        .inspect_err(|e| println!("Failed to rename to mruby;\n Err: {e}"))?;
    status
}

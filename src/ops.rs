//! Implements the operations exposed through the CLI

use std::ffi::OsStr;
use std::path::Path;
use std::{fs, path::PathBuf};

use structopt::StructOpt;
use toml_edit as toml;

use crate::cargo;
use crate::data::{
    BINDINGS_BUILD_RS, BINDINGS_CARGO_TOML, BINDINGS_CRATE_PATH, BINDINGS_SRC_LIB_RS,
    CARGO_CONFIG_DIR, CARGO_CONFIG_TOML, CARGO_CONFIG_TOML_FILENAME, PACKAGE_METADATA_INIT,
    RUST_TOOLCHAIN_TOML, RUST_TOOLCHAIN_TOML_FILENAME, WINDOWS_RS_VERSION,
    WINDOWS_RS_VERSION_PLACEHOLDER,
};

#[derive(Debug, StructOpt)]
pub(crate) struct New {
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

impl New {
    pub(crate) fn perform(&self) -> anyhow::Result<()> {
        // TODO: Change this to use self.path instead, once everything has settled.
        let package_root = if cfg!(debug_assertions) {
            PathBuf::from(r#"C:\Users\Tim\source\_temp\uwp-rs"#)
        } else {
            PathBuf::from(&self.path)
        };
        cargo::new([&package_root].iter())?;

        // At this point the package directory should exist, so we could use
        // `canonicalize` if we ever need a fully qualified path name, e.g.:
        // let path = package_root.canonicalize()?;
        // println!("path: {:?}", &path);

        // Write toolchain file
        copy_file(
            &package_root,
            None,
            RUST_TOOLCHAIN_TOML_FILENAME,
            RUST_TOOLCHAIN_TOML,
        )?;

        // Append `package.metadata` to *Cargo.toml*
        let mut cargo_toml = PathBuf::from(&package_root);
        cargo_toml.push("Cargo.toml");
        let mut contents = fs::read(&cargo_toml)?;
        contents.extend_from_slice(PACKAGE_METADATA_INIT.as_bytes());
        fs::write(&cargo_toml, contents)?;

        // Write default cargo configuration
        copy_file(
            &package_root,
            Some(&[CARGO_CONFIG_DIR]),
            CARGO_CONFIG_TOML_FILENAME,
            CARGO_CONFIG_TOML,
        )?;

        // Create bindings crate
        let content =
            BINDINGS_CARGO_TOML.replace(WINDOWS_RS_VERSION_PLACEHOLDER, WINDOWS_RS_VERSION);
        copy_file(
            &package_root,
            Some(&[BINDINGS_CRATE_PATH]),
            "Cargo.toml",
            &content,
        )?;

        copy_file(
            &package_root,
            Some(&[BINDINGS_CRATE_PATH]),
            "build.rs",
            BINDINGS_BUILD_RS,
        )?;

        copy_file(
            &package_root,
            Some(&[BINDINGS_CRATE_PATH, "src"]),
            "lib.rs",
            BINDINGS_SRC_LIB_RS,
        )?;

        // Update Cargo.toml to include the bindings crate
        let mut manifest_file = PathBuf::from(&package_root);
        manifest_file.push("Cargo.toml");
        let manifest = fs::read_to_string(&manifest_file)?;
        let mut manifest: toml::Document = manifest.parse()?;

        // TODO: Insert table if it doesn't exist
        let dependencies = manifest["dependencies"].as_table_mut().unwrap();
        let mut bindings = toml::InlineTable::default();
        bindings.get_or_insert("path", "./bindings");
        bindings.fmt();
        dependencies["bindings"] = toml::value(toml::Value::InlineTable(bindings));
        dependencies["windows"] = toml::value(toml::Value::from(WINDOWS_RS_VERSION));

        fs::write(
            &manifest_file,
            manifest.to_string_in_original_order().as_bytes(),
        )?;

        Ok(())
    }
}

/// Copies file contents into a destination file. The destination is created starting from
/// `base_dir` by subsequently appending all `sub_dir` parts, if any.
///
/// If the destination directory (or a parent) doesn't exist it is created.
///
fn copy_file<'a>(
    base_dir: &Path,
    sub_dir: Option<&'a [&'a str]>,
    dest_name: impl AsRef<OsStr>,
    contents: impl AsRef<[u8]>,
) -> anyhow::Result<()> {
    let mut dest_dir: PathBuf = base_dir.into();

    // Construct destination directory
    if let Some(sub_dirs) = sub_dir {
        for sub_dir in sub_dirs {
            dest_dir.push(sub_dir);
        }
    }

    // Ensure the directory exists
    if !dest_dir.exists() {
        fs::create_dir_all(&dest_dir)?;
    }

    let mut dest_file = dest_dir;
    dest_file.push(dest_name.as_ref());

    // Write out contents
    fs::write(&dest_file, contents)?;

    Ok(())
}

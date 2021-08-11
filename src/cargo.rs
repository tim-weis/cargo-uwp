//! Provides wrappers to run individual cargo commands (e.g. `cargo new`).

use std::{
    ffi::OsStr,
    io,
    process::{Command, Stdio},
};

use anyhow::{anyhow, Context};

pub(crate) fn new(args: impl Iterator<Item = impl AsRef<OsStr>>) -> anyhow::Result<()> {
    let mut child = Command::new("cargo")
        .args(["new", "--color", "always"])
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("Executing cargo new")?;

    // Redirect child's stdout
    let mut stdout = child
        .stdout
        .as_mut()
        .ok_or_else(|| anyhow!("Failed to connect to child process stdout"))?;
    io::copy(&mut stdout, &mut io::stdout())?;

    // Redirect child's stderr
    let mut stderr = child
        .stderr
        .as_mut()
        .ok_or_else(|| anyhow!("Failed to conntect to child process stderr"))?;
    io::copy(&mut stderr, &mut io::stderr())?;

    // Wait for child process to terminate so that we can report potential errors
    let result = child.wait()?;
    if result.success() {
        Ok(())
    } else {
        Err(anyhow!("Failed to execute cargo new"))
    }
}

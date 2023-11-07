use std::{path::PathBuf, fs};

use hashbrown::HashSet;

use crate::{safe_unwrap, errors::RuntimeError};

macro_rules! chain_cond {
    ($x:expr, $($y:expr),* $(,)?) => {{
        if let Some(x) = $x {
            let x = x.to_string_lossy().to_string().to_lowercase();
            $(x.contains(&$y))||*
        } else { return; }
    }};
}

#[inline]
pub fn search(path: PathBuf, set: &mut HashSet<PathBuf>) {
    if !path.exists() { return; }

    // will skip if path contains:
    if chain_cond! {
        path.file_name(),
        "temp",
        "tmp",
        "cache",
        "history",
        "default",
        "discord",
        "microsoft",
        "brave",
        "workspace",
        "data",
        "storage",
        "language",
        "sandbox",
        "feedback",
        "session",
        "result",
        "static",
        "overrides",
        "probe",
    } { return; }

    if path.is_file() {
        if chain_cond!(&path.extension(), "toml", "json", "nix", "conf", "config") {
            set.insert(path);
        } return;
    }

    if path.is_dir() {
        let children = safe_unwrap!(fs::read_dir(&path) => RT011, path.to_string_lossy());
        for child in children {
            let child = safe_unwrap!(child => RT011, path.to_string_lossy());
            search(child.path(), set);
        }
    }
}
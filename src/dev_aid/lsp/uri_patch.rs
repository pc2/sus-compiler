/// Patches to bridge the 0.94 -> 0.97 lsp-types update, where they replaced their Uri library for one with far fewer dependencies.
/// Kindly stolen from https://github.com/Desdaemon/odoo-lsp/blob/9c03a1219f01a54bbdee76db4f012856cf72d90b/src/utils.rs#L482-L544
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

use lsp_types::Uri;

pub trait UriExt {
    fn to_file_path(&self) -> Option<Cow<Path>>;
}

impl UriExt for lsp_types::Uri {
    fn to_file_path(&self) -> Option<Cow<Path>> {
        let path = match self.path().as_estr().decode().into_string_lossy() {
            Cow::Borrowed(ref_) => Cow::Borrowed(Path::new(ref_)),
            Cow::Owned(owned) => Cow::Owned(PathBuf::from(owned)),
        };

        #[cfg(windows)]
        {
            let authority = self.authority().expect("url has no authority component");
            let host = authority.host().as_str();
            if host.is_empty() {
                // very high chance this is a `file:///` uri
                // in which case the path will include a leading slash we need to remove
                let host = path.to_string_lossy();
                let host = &host[1..];
                return Some(Cow::Owned(PathBuf::from(host)));
            }

            let host = format!("{host}:");
            Some(Cow::Owned(
                Path::new(&host)
                    .components()
                    .chain(path.components())
                    .collect(),
            ))
        }

        #[cfg(not(windows))]
        Some(path)
    }
}

pub fn uri_from_file_path(path: &Path) -> Option<Uri> {
    let fragment = if !path.is_absolute() {
        Cow::from(strict_canonicalize(path).ok()?)
    } else {
        Cow::from(path)
    };

    #[cfg(windows)]
    {
        // we want to parse a triple-slash path for Windows paths
        // it's a shorthand for `file://localhost/C:/Windows` with the `localhost` omitted
        let raw = format!("file:///{}", fragment.to_string_lossy().replace("\\", "/"));
        Uri::from_str(&raw).ok()
    }

    use std::str::FromStr;
    #[cfg(not(windows))]
    Uri::from_str(&format!("file://{}", fragment.to_string_lossy())).ok()
}

#[cfg(not(windows))]
pub use std::fs::canonicalize as strict_canonicalize;

/// On Windows, rewrites the wide path prefix `\\?\C:` to `C:`  
/// Source: https://stackoverflow.com/a/70970317
#[inline]
#[cfg(windows)]
pub fn strict_canonicalize<P: AsRef<Path>>(path: P) -> anyhow::Result<PathBuf> {
    use anyhow::Context;

    fn impl_(path: PathBuf) -> anyhow::Result<PathBuf> {
        let head = path.components().next().context("empty path")?;
        let disk_;
        let head = if let std::path::Component::Prefix(prefix) = head {
            if let std::path::Prefix::VerbatimDisk(disk) = prefix.kind() {
                disk_ = format!("{}:", disk as char);
                Path::new(&disk_)
                    .components()
                    .next()
                    .context("failed to parse disk component")?
            } else {
                head
            }
        } else {
            head
        };
        Ok(std::iter::once(head)
            .chain(path.components().skip(1))
            .collect())
    }
    let canon = std::fs::canonicalize(path)?;
    impl_(canon)
}

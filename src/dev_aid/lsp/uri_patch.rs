/// Patches to bridge the 0.94 -> 0.97 lsp-types update, where they replaced their Uri library for one with far fewer dependencies.
/// Kindly stolen from https://github.com/Desdaemon/odoo-lsp/blob/9c03a1219f01a54bbdee76db4f012856cf72d90b/src/utils.rs#L482-L544
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

use lsp_types::Uri;

pub trait UriExt {
    fn to_file_path(&self) -> Option<Cow<'_, Path>>;
}

impl UriExt for lsp_types::Uri {
    fn to_file_path(&self) -> Option<Cow<'_, Path>> {
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

/// Percent-encodes a file path for use as the path component of a `file://` URI.
///
/// Everything except RFC 3986 unreserved characters and `/` is encoded, so spaces, non-ASCII
/// characters, `#`, `?`, `%`, `[`, `]`, ... all become `%XX` escapes. This matches what editors
/// (e.g. VS Code) send us, and [UriExt::to_file_path] decodes it again.
fn percent_encode_path(path: &str) -> String {
    let mut result = String::with_capacity(path.len());
    for b in path.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.' | b'_' | b'~' | b'/') {
            result.push(b as char);
        } else {
            result.push_str(&format!("%{b:02X}"));
        }
    }
    result
}

pub fn uri_from_file_path(path: &Path) -> Option<Uri> {
    use std::str::FromStr;

    let fragment = if !path.is_absolute() {
        Cow::from(strict_canonicalize(path).ok()?)
    } else {
        Cow::from(path)
    };

    #[cfg(windows)]
    {
        // we want to parse a triple-slash path for Windows paths
        // it's a shorthand for `file://localhost/C:/Windows` with the `localhost` omitted
        let path = fragment.to_string_lossy().replace("\\", "/");
        Uri::from_str(&format!("file:///{}", percent_encode_path(&path))).ok()
    }

    #[cfg(not(windows))]
    Uri::from_str(&format!(
        "file://{}",
        percent_encode_path(&fragment.to_string_lossy())
    ))
    .ok()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn special_characters_round_trip() {
        for p in [
            "/home/stacu/proj/dspfp32_dw_arith.sus",
            "/home/stacu/my proj/a.sus",
            "/home/stacu/Übung/a.sus",
            "/home/stacu/proj#1/a.sus",
            "/home/stacu/proj?/a.sus",
            "/home/stacu/[x]/a.sus",
            "/home/stacu/100%/a.sus",
            "/home/stacu/a+b=c;d,e@f/a.sus",
        ] {
            let uri = uri_from_file_path(Path::new(p)).expect(p);
            assert_eq!(uri.to_file_path().unwrap().as_ref(), Path::new(p), "{p}");
        }
    }

    #[test]
    fn encoding_matches_editor_convention() {
        let uri = uri_from_file_path(Path::new("/home/stacu/my proj/Übung/a.sus")).unwrap();
        assert_eq!(
            uri.as_str(),
            "file:///home/stacu/my%20proj/%C3%9Cbung/a.sus"
        );
    }
}

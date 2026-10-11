//! The save contract shared by native dialogs and capability-scoped control.
//!
//! New destinations are created without clobbering an existing file. Saving
//! back to an opened file requires the exact bytes last opened or saved. An
//! explicit replacement may overwrite a different destination. Callers stage
//! bytes beside the target and check the contract again at commit time.

use std::io::{self, Read};
use std::path::Path;

use crate::atomic::{AtomicFs, RealFs, RenameRetry, atomic_write_with};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SaveIntent {
    CreateNew,
    ReplaceOpened([u8; 32]),
    ReplaceExplicit,
}

impl SaveIntent {
    pub fn for_document(source_path: Option<&str>, target: &str, opened_digest: Option<[u8; 32]>, replace: bool) -> Result<Self, String> {
        if source_path == Some(target) {
            return opened_digest.map(Self::ReplaceOpened).ok_or_else(|| "cannot overwrite this source: no opened-content baseline; save to a new path".into());
        }
        Ok(if replace { Self::ReplaceExplicit } else { Self::CreateNew })
    }
}

pub fn digest(bytes: &[u8]) -> [u8; 32] {
    *blake3::hash(bytes).as_bytes()
}

/// Hash a file through either an ambient or capability-scoped handle without
/// allocating in proportion to its size.
pub fn digest_reader(reader: &mut impl Read) -> io::Result<[u8; 32]> {
    let mut hasher = blake3::Hasher::new();
    let mut chunk = [0u8; 64 * 1024];
    loop {
        let n = reader.read(&mut chunk)?;
        if n == 0 {
            return Ok(*hasher.finalize().as_bytes());
        }
        hasher.update(&chunk[..n]);
    }
}

/// Check a target state observed through the caller's filesystem authority.
pub fn check_observed(intent: SaveIntent, observed: Option<[u8; 32]>, display: &str) -> io::Result<()> {
    match (intent, observed) {
        (SaveIntent::CreateNew, None) | (SaveIntent::ReplaceExplicit, _) => Ok(()),
        (SaveIntent::ReplaceOpened(expected), Some(actual)) if expected == actual => Ok(()),
        (SaveIntent::CreateNew, Some(_)) => {
            Err(io::Error::new(io::ErrorKind::AlreadyExists, format!("save conflict: `{display}` already exists; choose another path or confirm replacement")))
        }
        (SaveIntent::ReplaceOpened(_), None) => {
            Err(io::Error::new(io::ErrorKind::NotFound, format!("save conflict: `{display}` was removed; save to a new path")))
        }
        (SaveIntent::ReplaceOpened(_), Some(_)) => {
            Err(io::Error::new(io::ErrorKind::Other, format!("save conflict: `{display}` changed since it was opened; save to a new path")))
        }
    }
}

fn observed_at(path: &Path) -> io::Result<Option<[u8; 32]>> {
    match std::fs::File::open(path) {
        Ok(mut file) => {
            if !file.metadata()?.is_file() {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("save conflict: `{}` is not a regular file", path.display())));
            }
            digest_reader(&mut file).map(Some)
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error),
    }
}

pub fn check_path(path: &Path, intent: SaveIntent) -> io::Result<()> {
    check_observed(intent, observed_at(path)?, &path.display().to_string())
}

/// Staged native save. The CreateNew commit uses an atomic hard link so a
/// competing writer cannot insert a destination between check and commit.
pub fn atomic_write_guarded(path: &Path, bytes: &[u8], intent: SaveIntent) -> io::Result<()> {
    check_path(path, intent)?;
    atomic_write_with(&GuardedFs(intent), RenameRetry::platform(), path, bytes)
}

struct GuardedFs(SaveIntent);

impl AtomicFs for GuardedFs {
    fn write_new(&self, tmp: &Path, bytes: &[u8]) -> io::Result<()> {
        RealFs.write_new(tmp, bytes)
    }
    fn rename(&self, from: &Path, to: &Path) -> io::Result<()> {
        match self.0 {
            SaveIntent::CreateNew => {
                std::fs::hard_link(from, to)?;
                let _ = std::fs::remove_file(from);
                Ok(())
            }
            SaveIntent::ReplaceOpened(_) => {
                check_path(to, self.0)?;
                RealFs.rename(from, to)
            }
            SaveIntent::ReplaceExplicit => RealFs.rename(from, to),
        }
    }
    fn remove(&self, path: &Path) -> io::Result<()> {
        RealFs.remove(path)
    }
    fn sync_dir(&self, dir: &Path) -> io::Result<()> {
        RealFs.sync_dir(dir)
    }
    fn copy_permissions(&self, from: &Path, to: &Path) -> io::Result<()> {
        RealFs.copy_permissions(from, to)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn changed_source_and_occupied_save_as_leave_the_last_good_bytes() {
        let dir = std::env::temp_dir().join(format!("photostrike-save-guard-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let source = dir.join("source.pstrike");
        std::fs::write(&source, b"opened project").unwrap();
        let intent = SaveIntent::ReplaceOpened(digest(b"opened project"));
        std::fs::write(&source, b"external project").unwrap();
        assert!(atomic_write_guarded(&source, b"our edits", intent).unwrap_err().to_string().contains("save conflict"));
        assert_eq!(std::fs::read(&source).unwrap(), b"external project");

        let image = dir.join("existing.png");
        std::fs::write(&image, b"valid PNG fixture bytes").unwrap();
        assert!(atomic_write_guarded(&image, b"new export", SaveIntent::CreateNew).is_err());
        assert_eq!(std::fs::read(&image).unwrap(), b"valid PNG fixture bytes");
        atomic_write_guarded(&image, b"new export", SaveIntent::ReplaceExplicit).unwrap();
        assert_eq!(std::fs::read(&image).unwrap(), b"new export");
        let _ = std::fs::remove_file(source);
        let _ = std::fs::remove_file(image);
        let _ = std::fs::remove_dir(dir);
    }
}

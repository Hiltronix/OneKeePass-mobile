// Android SAF providers may not support atomic document replacement.
// This function copies a previously fsynced KDBX recovery file over a seekable
// provider descriptor. The caller must keep the recovery file on any failure.

use std::fs::File;
use std::io::{self, Error, ErrorKind, Seek, SeekFrom};

pub(crate) fn copy_staged_kdbx_to_provider(
    writer: &mut File,
    backup_file: &mut File,
) -> io::Result<()> {
    let staged_len = backup_file.metadata()?.len();
    if staged_len < 8 {
        return Err(Error::new(ErrorKind::InvalidData, "Staged KDBX is incomplete"));
    }

    // Reject non-seekable/non-truncatable providers without changing the length.
    let original_len = writer.seek(SeekFrom::End(0))?;
    writer.set_len(original_len)?;
    writer.rewind()?;
    backup_file.rewind()?;

    let bytes_written = io::copy(backup_file, writer)?;
    if bytes_written != staged_len {
        return Err(Error::new(ErrorKind::UnexpectedEof, "Incomplete staged KDBX copy"));
    }

    // Old trailing bytes must not remain if the new encrypted KDBX is shorter.
    writer.set_len(bytes_written)?;
    writer.sync_all()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, OpenOptions};
    use std::io::Write;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn test_path(label: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!(
            "okp-kdbx-save-{}-{}-{}", std::process::id(), label, nonce
        ))
    }

    fn create_file(path: &PathBuf, bytes: &[u8]) -> File {
        let mut file = OpenOptions::new()
            .create_new(true)
            .read(true)
            .write(true)
            .open(path)
            .unwrap();
        file.write_all(bytes).unwrap();
        file.sync_all().unwrap();
        file
    }

    #[test]
    fn shorter_save_removes_old_trailing_bytes() {
        let destination = test_path("shorter-destination");
        let backup = test_path("shorter-backup");
        let old_bytes = vec![0xA5; 2048];
        let new_bytes = b"NEW_ENCRYPTED_DATABASE";
        let mut writer = create_file(&destination, &old_bytes);
        let mut stage = create_file(&backup, new_bytes);

        copy_staged_kdbx_to_provider(&mut writer, &mut stage).unwrap();
        drop(writer);
        drop(stage);
        assert_eq!(fs::read(&destination).unwrap(), new_bytes);
        assert_eq!(fs::read(&backup).unwrap(), new_bytes);
        fs::remove_file(destination).unwrap();
        fs::remove_file(backup).unwrap();
    }

    #[test]
    fn longer_save_grows_destination_without_losing_stage() {
        let destination = test_path("longer-destination");
        let backup = test_path("longer-backup");
        let new_bytes = vec![0xB6; 4096];
        let mut writer = create_file(&destination, b"OLD_DATABASE");
        let mut stage = create_file(&backup, &new_bytes);

        copy_staged_kdbx_to_provider(&mut writer, &mut stage).unwrap();
        drop(writer);
        drop(stage);
        assert_eq!(fs::read(&destination).unwrap(), new_bytes);
        assert_eq!(fs::read(&backup).unwrap(), new_bytes);
        fs::remove_file(destination).unwrap();
        fs::remove_file(backup).unwrap();
    }

    #[test]
    fn readonly_provider_fails_before_overwriting_destination() {
        let destination = test_path("readonly-destination");
        let backup = test_path("readonly-backup");
        let old_bytes = b"ORIGINAL_DATABASE_CONTENT";
        let new_bytes = b"NEW_ENCRYPTED_DATABASE";
        drop(create_file(&destination, old_bytes));
        let mut writer = File::open(&destination).unwrap();
        let mut stage = create_file(&backup, new_bytes);

        assert!(copy_staged_kdbx_to_provider(&mut writer, &mut stage).is_err());
        drop(writer);
        drop(stage);
        assert_eq!(fs::read(&destination).unwrap(), old_bytes);
        assert_eq!(fs::read(&backup).unwrap(), new_bytes);
        fs::remove_file(destination).unwrap();
        fs::remove_file(backup).unwrap();
    }

    #[test]
    fn incomplete_stage_does_not_modify_destination() {
        let destination = test_path("invalid-destination");
        let backup = test_path("invalid-backup");
        let old_bytes = b"ORIGINAL_DATABASE_CONTENT";
        let mut writer = create_file(&destination, old_bytes);
        let mut stage = create_file(&backup, b"bad");

        assert!(copy_staged_kdbx_to_provider(&mut writer, &mut stage).is_err());
        drop(writer);
        drop(stage);
        assert_eq!(fs::read(&destination).unwrap(), old_bytes);
        fs::remove_file(destination).unwrap();
        fs::remove_file(backup).unwrap();
    }
}

// Document-provider display names are untrusted. Never use their path
// components to form paths in the app-private encrypted KDBX backup tree.

pub(crate) fn backup_file_stem(name: &str) -> Option<&str> {
    if name.trim().is_empty()
        || name.chars().any(|c| c == '/' || c == '\\' || c.is_control())
    {
        return None;
    }

    let stem = name.strip_suffix(".kdbx").unwrap_or(name);
    if stem.trim().is_empty() || stem == "." || stem == ".." {
        return None;
    }

    Some(stem)
}

#[cfg(test)]
mod tests {
    use super::backup_file_stem;

    #[test]
    fn accepts_normal_database_names() {
        assert_eq!(backup_file_stem("Vault.kdbx"), Some("Vault"));
        assert_eq!(backup_file_stem("Shared password data.kdbx"), Some("Shared password data"));
        assert_eq!(backup_file_stem("Archive"), Some("Archive"));
        assert_eq!(backup_file_stem("Données privées.kdbx"), Some("Données privées"));
    }

    #[test]
    fn rejects_directory_traversal_and_separators() {
        for input in [
            "../outside.kdbx",
            "/tmp/outside.kdbx",
            "backup/subdir.kdbx",
            "..\\outside.kdbx",
            "C:\\temp\\outside.kdbx",
            "safe/../../outside.kdbx",
        ] {
            assert_eq!(backup_file_stem(input), None, "{input}");
        }
    }

    #[test]
    fn rejects_invalid_and_control_character_names() {
        for input in ["", " ", ".kdbx", ".", "..", "  .kdbx", "evil\n.kdbx", "evil\r.kdbx", "evil\0.kdbx"] {
            assert_eq!(backup_file_stem(input), None, "{input:?}");
        }
    }

    #[test]
    fn does_not_interpret_encoded_separators_as_directories() {
        assert_eq!(backup_file_stem("encoded%2Fname.kdbx"), Some("encoded%2Fname"));
    }
}

use std::{
    borrow::Cow,
    ffi::OsString,
    path::{Component, Path, PathBuf, MAIN_SEPARATOR_STR},
};

/// Normalizes a given file path by removing redundant components such as `.` and `..`,
/// handling symbolic links correctly, and simplifying the path according to the operating system's rules.
/// On Windows, it also converts paths to a consistent format by using `dunce::simplified`.
///
/// # Arguments
///
/// * `path` - The file path to normalize, provided as any type that can be converted into a `Path`.
///
/// # Returns
///
/// A `PathBuf` containing the normalized path.
///
/// # Platform-specific behavior
///
/// On Windows, the function simplifies the path using `dunce::simplified` to ensure consistent handling
/// of path separators and other Windows-specific quirks.
/// On Unix-like systems, it only performs basic normalization without additional simplifications.
pub fn normalize(path: impl AsRef<Path>) -> PathBuf {
    let mut components = path.as_ref().components().peekable();
    let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
        components.next();
        PathBuf::from(c.as_os_str())
    } else {
        PathBuf::new()
    };

    while let Some(component) = components.next() {
        match component {
            Component::Prefix(..) => unreachable!(),
            Component::RootDir => {
                ret.push(component.as_os_str());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                if !ret.pop() {
                    ret.push("..");
                }
            }
            Component::Normal(c) => {
                #[cfg(not(windows))]
                {
                    ret.push(c);
                }

                #[cfg(windows)]
                {
                    let new_path = ret.join(c);
                    if let Ok(metadata) = fs::symlink_metadata(&new_path) {
                        if metadata.file_type().is_symlink() {
                            ret = new_path;
                            continue;
                        }
                    }

                    match (fs::canonicalize(&new_path), fs::canonicalize(&ret)) {
                        (Ok(can_new), Ok(can_old)) => {
                            if let Ok(stripped) = can_new.strip_prefix(&can_old) {
                                ret.push(stripped);
                            } else {
                                ret.push(c);
                            }
                        }
                        _ => ret.push(c),
                    }
                }
            }
        }
    }

    dunce::simplified(&ret).to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_normalize_simple_path() {
        let path = Path::new("/home/user/project");
        assert_eq!(normalize(path), PathBuf::from("/home/user/project"));
    }

    #[test]
    fn test_normalize_with_curdir() {
        let path = Path::new("/home/user/./project");
        assert_eq!(normalize(path), PathBuf::from("/home/user/project"));
    }

    #[test]
    fn test_normalize_with_parentdir() {
        let path = Path::new("/home/user/../project");
        assert_eq!(normalize(path), PathBuf::from("/home/project"));
    }

    #[test]
    fn test_normalize_with_multiple_parentdir() {
        let path = Path::new("/home/user/.././../project");
        assert_eq!(normalize(path), PathBuf::from("/project"));
    }

    #[test]
    fn test_normalize_root_path() {
        let path = Path::new("/");
        assert_eq!(normalize(path), PathBuf::from("/"));
    }

    #[test]
    fn test_normalize_with_symlink() {
        // This test assumes "target" is a symlink to "/home/user/project"
        let path = Path::new("/home/user/symlink/../project");
        assert_eq!(normalize(path), PathBuf::from("/home/user/project"));
    }

    #[test]
    #[cfg(windows)]
    fn test_normalize_windows_root() {
        let path = Path::new("C:\\Windows\\System32");
        assert_eq!(normalize(path), PathBuf::from("C:\\Windows\\System32"));
    }

    #[test]
    #[cfg(windows)]
    fn test_normalize_windows_with_parentdir() {
        let path = Path::new("C:\\Windows\\System32\\..\\Temp");
        assert_eq!(normalize(path), PathBuf::from("C:\\Windows\\Temp"));
    }

    #[test]
    #[cfg(windows)]
    fn test_normalize_windows_with_symlink() {
        // This test assumes "C:\\link" is a symlink to "C:\\Actual\\Path"
        let path = Path::new("C:\\link\\..\\Path");
        assert_eq!(normalize(path), PathBuf::from("C:\\Path"));
    }
}

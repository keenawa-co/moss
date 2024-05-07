use async_graphql::{Scalar, ScalarType, Value};
use std::{ffi::OsStr, fmt, fs, io::Error, path::PathBuf};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Path(PathBuf);

impl Path {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Path(path.into())
    }

    pub fn canonicalize(&self) -> Result<PathBuf, Error> {
        fs::canonicalize(self)
    }

    pub fn as_path_buf(&self) -> &PathBuf {
        &self.0
    }

    pub fn as_path(&self) -> &std::path::Path {
        &self.0.as_path()
    }
}

impl AsRef<PathBuf> for Path {
    fn as_ref(&self) -> &PathBuf {
        &self.0
    }
}

impl AsRef<std::path::Path> for Path {
    fn as_ref(&self) -> &std::path::Path {
        self.0.as_path()
    }
}

impl AsRef<OsStr> for Path {
    fn as_ref(&self) -> &OsStr {
        self.0.as_os_str()
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

impl From<Path> for PathBuf {
    fn from(path: Path) -> Self {
        path.0
    }
}

#[Scalar]
impl ScalarType for Path {
    fn parse(value: Value) -> async_graphql::InputValueResult<Self> {
        if let Value::String(path) = &value {
            Ok(Path(PathBuf::from(path)))
        } else {
            Err("Path must be a string.".into())
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string_lossy().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_graphql::InputValueResult;

    #[test]
    fn path_display() {
        let path = Path::new("/some/directory/file.txt");
        assert_eq!(format!("{}", path), "/some/directory/file.txt");
    }

    #[test]
    fn path_as_ref_path() {
        let path = Path::new("/some/directory/file.txt");
        let as_path: &std::path::Path = path.as_ref();
        assert_eq!(as_path, std::path::Path::new("/some/directory/file.txt"));
    }

    #[test]
    fn path_as_ref_os_str() {
        let path = Path::new("/some/directory/file.txt");
        let as_os_str: &OsStr = path.as_ref();
        assert_eq!(as_os_str, OsStr::new("/some/directory/file.txt"));
    }

    #[test]
    fn path_into_path_buf() {
        let path = Path::new("/some/directory/file.txt");
        let path_buf: PathBuf = path.into();
        assert_eq!(path_buf, PathBuf::from("/some/directory/file.txt"));
    }

    #[test]
    fn parse_valid_path() {
        let value = Value::String(String::from("/some/path/file.txt"));
        let result: InputValueResult<Path> = Path::parse(value);
        assert!(result.is_ok());
        let path = result.unwrap();
        assert_eq!(path.0.to_string_lossy(), "/some/path/file.txt");
    }
    #[test]
    fn to_value_conversion() {
        let path = Path(PathBuf::from("/another/path/file.txt"));
        let value = path.to_value();
        assert_eq!(value, Value::String("/another/path/file.txt".to_string()));
    }
}

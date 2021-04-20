pub trait IntoStreamPath {
    fn into_stream_path(self) -> Option<(String, usize)>;
}

use std::path::{Path, PathBuf};

impl IntoStreamPath for String {
    fn into_stream_path(self) -> Option<(String, usize)> {
        let len = std::fs::metadata(&self).ok()?.len();
        
        Some((self, len as usize))
    }
}

impl IntoStreamPath for &Path {
    fn into_stream_path(self) -> Option<(String, usize)> {
        let len = std::fs::metadata(&self).ok()?.len();
        
        Some((self.to_string_lossy().into_owned(), len as usize))
    }
}


impl IntoStreamPath for PathBuf {
    fn into_stream_path(self) -> Option<(String, usize)> {
        let len = std::fs::metadata(&self).ok()?.len();
        
        Some((self.to_string_lossy().into_owned(), len as usize))
    }
}

impl IntoStreamPath for Vec<u8> {
    fn into_stream_path(self) -> Option<(String, usize)> {
        let len = self.len();
        const PATH: &str = "sd:/temp.bin";

        std::fs::write(
            PATH,
            self
        ).ok()?;

        Some((String::from(PATH), len))
    }
}

impl<T: IntoStreamPath> IntoStreamPath for Option<T> {
    fn into_stream_path(self) -> Option<(String, usize)> {
        self?.into_stream_path()
    }
}

impl<T: IntoStreamPath, E> IntoStreamPath for Result<T, E> {
    fn into_stream_path(self) -> Option<(String, usize)> {
        self.ok()?.into_stream_path()
    }
}

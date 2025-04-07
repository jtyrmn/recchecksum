use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub enum FileType {
    File,
    Directory,
}

pub struct FileEntry {
    pub relative_path: PathBuf,
    pub file_type: FileType,
}

pub trait DirectoryTreeReader {
    fn iter(&self) -> impl Iterator<Item = FileEntry>;
}

pub struct DirectoryTreeReaderImpl {
    root_directory: PathBuf,
}

impl DirectoryTreeReaderImpl {
    pub fn new(root_directory: &str) -> Result<DirectoryTreeReaderImpl, std::io::Error> {
        let root_directory = Path::new(root_directory).canonicalize()?;

        Ok(DirectoryTreeReaderImpl { root_directory })
    }
}

impl DirectoryTreeReader for DirectoryTreeReaderImpl {
    fn iter(&self) -> impl Iterator<Item = FileEntry> {
        WalkDir::new(&self.root_directory)
            .sort_by_file_name()
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .map(|e| {
                let relative_path = match e.path().strip_prefix(&self.root_directory) {
                    Ok(p) => p,
                    Err(e) => panic!("{}", e),
                }
                .to_owned();
                let file_type = match relative_path.is_dir() {
                    true => FileType::Directory,
                    false => FileType::File,
                };

                FileEntry {
                    relative_path,
                    file_type,
                }
            })
    }
}

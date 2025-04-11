use crate::filter::filter::Filter;
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

pub struct DirectoryTreeReaderOpts {
    pub filters: Vec<Box<dyn Filter>>,
    pub max_depth: Option<usize>,
    pub follow_symlinks: bool,
}

impl DirectoryTreeReaderOpts {
    pub fn new() -> DirectoryTreeReaderOpts {
        DirectoryTreeReaderOpts {
            filters: Vec::new(),
            max_depth: None,
            follow_symlinks: false,
        }
    }
}

pub trait DirectoryTreeReader {
    fn iter(&self) -> impl Iterator<Item = FileEntry>;
}

pub struct DirectoryTreeReaderImpl {
    root_directory: PathBuf,
    opts: DirectoryTreeReaderOpts,
}

impl DirectoryTreeReaderImpl {
    pub fn new(
        root_directory: impl AsRef<Path>,
        opts: DirectoryTreeReaderOpts,
    ) -> Result<DirectoryTreeReaderImpl, std::io::Error> {
        let root_directory = root_directory.as_ref().canonicalize()?;

        Ok(DirectoryTreeReaderImpl {
            root_directory,
            opts,
        })
    }
}

impl DirectoryTreeReader for DirectoryTreeReaderImpl {
    fn iter(&self) -> impl Iterator<Item = FileEntry> {
        let iter = WalkDir::new(&self.root_directory)
            .sort_by_file_name()
            .follow_links(self.opts.follow_symlinks);
        let iter = if let Some(max_depth) = self.opts.max_depth {
            iter.max_depth(max_depth)
        } else {
            iter
        };

        iter.into_iter()
            .map(|e| match e {
                Ok(e) => e,
                Err(e) => panic!("{:?}", e),
            })
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

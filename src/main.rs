mod reader;
mod filter;

use reader::directory_tree_reader::DirectoryTreeReader;
use reader::directory_tree_reader::DirectoryTreeReaderImpl;
use reader::directory_tree_reader::FileType;
use crate::reader::directory_tree_reader::DirectoryTreeReaderOpts;

fn main() {
    let mut opts = DirectoryTreeReaderOpts::new();
    opts.max_depth = Some(3);
    opts.follow_symlinks = true;

    let dir_tree_reader: DirectoryTreeReaderImpl =
        DirectoryTreeReaderImpl::new(".".as_ref(), opts).unwrap_or_else(|e| panic!("{}", e));

    for file_entry in dir_tree_reader.iter().take(100) {
        println!(
            "{} {}",
            match file_entry.file_type {
                FileType::File => 0,
                FileType::Directory => 1,
            },
            file_entry.relative_path.display()
        );
    }
    println!("done");
}

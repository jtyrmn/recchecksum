mod reader;

use reader::directory_tree_reader::DirectoryTreeReader;
use reader::directory_tree_reader::DirectoryTreeReaderImpl;
use reader::directory_tree_reader::FileType;

fn main() {
    let dir_tree_reader: DirectoryTreeReaderImpl =
        DirectoryTreeReaderImpl::new(".").unwrap_or_else(|e| panic!("{}", e));

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

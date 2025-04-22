use std::fs::{read_dir, rename};

fn main() {
    for root_dir in read_dir("image-db").unwrap() {
        let root_dir = root_dir.unwrap();
        for file_entry in read_dir(root_dir.path()).unwrap() {
            let file_entry = file_entry.unwrap();

            let path_full = file_entry.path();
            let filename = path_full.iter().last().unwrap().to_str().unwrap();
            if filename.contains("@n06") {
                let new_filename = filename.replace("@n06", "@N06");
                let new_path = path_full.with_file_name(new_filename);
                println!("rename {filename} to {}", new_path.display());
                rename(path_full, new_path).unwrap();
            } else if filename.contains("@N06") {
                // already renamed
            } else {
                println!("unknown file {}", path_full.display());
            }
        }
    }
}

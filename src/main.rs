use std::collections::HashMap;
use std::env;
use std::fs::{self, DirEntry};
use std::path::Path;

fn default_configuration() -> HashMap<String, String> {
    let mut config = HashMap::new();

    config.insert("txt".to_string(), "Documents".to_string());
    config.insert("docx".to_string(), "Documents".to_string());
    config.insert("jpg".to_string(), "Images".to_string());
    config.insert("png".to_string(), "Images".to_string());
    config.insert("mp3".to_string(), "Audio".to_string());
    config.insert("mp4".to_string(), "Video".to_string());
    config.insert("epub".to_string(), "Books-EPUB".to_string());
    config.insert("pdf".to_string(), "Books-PDF".to_string());
    config.insert("zip".to_string(), "Zipped".to_string());

    config
}

fn organize_files<P: AsRef<Path>>(source_dir: P, config: &HashMap<String, String>) {
    if let Ok(entries) = fs::read_dir(source_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                categorize_file(entry, config);
            }
        }
    }
}

fn categorize_file(entry: DirEntry, config: &HashMap<String, String>) {
    let path = entry.path();
    if path.is_file() {
        if let Some(extension) = path.extension() {
            if let Some(category) = config.get(extension.to_str().unwrap()) {
                let target_dir = path.parent().unwrap().join(category);
                fs::create_dir_all(&target_dir).unwrap();
                fs::rename(path, target_dir.join(entry.file_name())).unwrap();
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <source_directory>", args[0]);
        return;
    }

    let source_dir = &args[1];
    let config = default_configuration();

    organize_files(source_dir, &config);
}

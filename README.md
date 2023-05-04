# File Organizer

File Organizer is a simple command-line application written in Rust that organizes files within a directory into category folders based on their file extensions.

## Features

- Organizes files in a directory based on their extensions.
- Supports various file types, including documents, images, audio, video, and books.
- Creates category folders automatically if they don't already exist.

## Usage

1. Install Rust on your machine by following the instructions on the [official website](https://www.rust-lang.org/tools/install).

2. Clone this repository to your local machine: ```git clone https://github.com/your_username/file_organizer.git```

3. Navigate to the project directory: ```cd file_organizer```

4. Compile the project with optimizations: ```cargo build --release```

5. Run the application, providing the path to the source directory you want to organize: ```.\target\release\file_organizer.exe <source_directory>```
   Replace `<source_directory>` with the path to the directory containing the files you want to organize.

## Supported File Types

The default configuration supports the following file types and organizes them into the respective category folders:

- Documents: txt, docx
- Images: jpg, png
- Audio: mp3
- Video: mp4
- Books-EPUB: epub
- Books-PDF: pdf
- Zipped: zip

You can modify the `default_configuration()` function in `main.rs` to add or modify the file types and categories.

## License

This project is licensed under the [MIT License](LICENSE).



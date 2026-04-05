use std::fs;
use std::path::PathBuf;
use std::io;

pub enum FileOrDir {
    File {
        name: String,
        path: PathBuf,
        content: String,
        parent: Box<FileOrDir>,
    },
    Folder {
        name: String,
        path: PathBuf,
        children: Vec<FileOrDir>,
        parent: Box<FileOrDir>
    },
}

pub fn open_folder(path: &str) -> io::Result<Vec<FileOrDir>> {
    let mut result = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();
        let meta = entry.metadata()?;

        if meta.is_dir() {
            result.push(FileOrDir::Folder {
                name,
                children: open_folder(entry.path().to_str().unwrap())?,
                parent: todo!(),
            });
        } else {
            result.push(FileOrDir::File {
                name,
                content: fs::read_to_string(entry.path())?,
                parent: todo!(),
            });
        }
    }

    Ok(result)
}

pub fn get_content(file: &FileOrDir) -> Option<&str> {
    match file {
        FileOrDir::File { content, .. } => Some(content),
        FileOrDir::Folder { .. } => None,
    }
}

pub fn save_content(content: &str, destination: &FileOrDir, file_name: &str) -> io::Result<()> {
    match destination {
        FileOrDir::File { path, .. } => {
            fs::write(path, content)?;
            Ok(())
        }
        FileOrDir::Folder { path, .. } => {
            let file_path = path.join(file_name);
            fs::write(file_path, content)?;
            Ok(())
        }
    }
}
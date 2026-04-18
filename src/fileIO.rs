use std::fs;
use std::path::PathBuf;
use std::io;

pub enum FileOrDir {
    File {
        name: String,
        path: PathBuf,
        content: String,
    },
    Folder {
        name: String,
        path: PathBuf,
        children: Vec<FileOrDir>,
    },
}

pub fn open_folder(path: &str) -> io::Result<Vec<FileOrDir>> {
    let mut result = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();
        let meta = entry.metadata()?;
        let path = entry.path();

        if meta.is_dir() {
            result.push(FileOrDir::Folder {
                name,
                children: open_folder(path.to_str().unwrap())?,
                path,
            });
        } else {
            result.push(FileOrDir::File {
                name,
                content: fs::read_to_string(&path)?,
                path,
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

pub fn find_file<'a>(folder: &'a FileOrDir, target: &str) -> Option<&'a FileOrDir> {
    match folder {
        FileOrDir::Folder { children, .. } => {
            children.iter().find(|child| match child {
                FileOrDir::File { name, .. } => name == target,
                FileOrDir::Folder { name, .. } => name == target,
            })
        }
        FileOrDir::File { .. } => None,
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
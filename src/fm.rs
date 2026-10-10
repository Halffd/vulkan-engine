use log::*;
use std::env;
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub struct FM {
    home: PathBuf,
    current_path: PathBuf,
}

impl FM {
    pub fn new() -> Self {
        let home = PathBuf::from(env::var("HOME").expect("HOME not set"));
        let current_path = home.join("Documents");

        Self { home, current_path }
    }

    // --- core path resolver ---
    pub fn resolve(&self, path: &str) -> PathBuf {
        let p = Path::new(path);
        if p.is_absolute() {
            p.to_path_buf()
        } else {
            self.current_path.join(p)
        }
    }

    // --- navigation ---
    pub fn cd(&mut self, dir: &str) {
        let new_path = self.resolve(dir);
        if new_path.is_dir() {
            self.current_path = new_path;
        } else {
            panic!("cd: not a directory");
        }
    }

    pub fn pwd(&self) {
        info!("{}", self.current_path.display());
    }

    // --- listing ---
    pub fn ls(&self) {
        self.ls_dir(".");
    }

    pub fn ls_dir(&self, dir: &str) {
        let path = self.resolve(dir);

        // Check if directory exists, if not create it or skip
        if !path.exists() {
            warn!("Directory does not exist: {}", path.display());
            return;
        }

        let entries = match fs::read_dir(&path) {
            Ok(entries) => entries,
            Err(e) => {
                error!("Failed to read directory {}: {}", path.display(), e);
                return;
            }
        };

        for entry in entries.flatten() {
            let path = entry.path();
            let name = path.file_name().unwrap().to_string_lossy();

            match entry.metadata() {
                Ok(meta) => {
                    let size = meta.len();
                    let modified = meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
                    let kind = if meta.is_dir() { "dir" } else { "file" };

                    info!("{} | {} | {} bytes | {:?}", kind, name, size, modified);
                }
                Err(_) => info!("? | {}", name),
            }
        }
    }

    // --- file ops ---
    pub fn touch(&self, file: &str) {
        let path = self.resolve(file);

        OpenOptions::new()
            .create(true)
            .write(true)
            .open(path)
            .expect("touch failed");
    }

    pub fn write(&self, file: &str, content: &str) {
        let path = self.resolve(file);
        fs::write(path, content).expect("write failed");
    }

    pub fn cat(&self, file: &str) {
        let path = self.resolve(file);
        let content = fs::read_to_string(path).expect("cat failed");
        info!("{}", content);
    }

    pub fn rm(&self, file: &str) {
        let path = self.resolve(file);
        fs::remove_file(path).expect("rm failed");
    }

    // --- dir ops ---
    pub fn mkdir(&self, dir: &str) {
        let path = self.resolve(dir);
        fs::create_dir_all(path).expect("mkdir failed");
    }

    pub fn rmdir(&self, dir: &str) {
        let path = self.resolve(dir);
        fs::remove_dir_all(path).expect("rmdir failed");
    }

    pub fn exists(&self, path: &str) -> bool {
        self.resolve(path).exists()
    }
}

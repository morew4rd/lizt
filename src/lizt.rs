use std::fs;
use std::path::Path;
use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct LzConf {
    path: String,
    ignore: Vec<String>,
}

impl LzConf {
    pub fn new(path: String) -> LzConf {
        LzConf {
            path,
            ignore: vec![
                ".git".to_string(),
                "node_modules".to_string(),
                ".zig-cache".to_string(),
                "deps".to_string(),
            ],
        }
    }
}

pub struct LzList {
    pub entries: Vec<String>
}

pub enum LzError {
    Io(io::Error),
    // Other(String)
}

impl From<io::Error> for LzError {
    fn from(err: io::Error) -> LzError {
        LzError::Io(err)
    }
}

fn read_gitignore(path: &Path) -> Result<Vec<String>, LzError> {
    let gitignore_path = path.join(".gitignore");
    if gitignore_path.exists() {
        let file = File::open(gitignore_path)?;
        let reader = BufReader::new(file);
        let mut ignores = Vec::new();
        for line in reader.lines() {
            ignores.push(line?);
        }
        Ok(ignores)
    } else {
        Ok(Vec::new())
    }
}

fn should_ignore(path: &Path, ignores: &[String]) -> bool {
    for ignore in ignores {
        if path.to_str().unwrap_or("").contains(ignore) {
            return true;
        }
    }
    false
}

pub fn lizt(lzconf: &LzConf) -> Result<LzList, LzError> {
    let p = Path::new(&lzconf.path);
    let mut entries = Vec::new();
    let mut stack = vec![p.to_path_buf()];
    let mut ignores = read_gitignore(p)?;
    ignores.extend(lzconf.ignore.iter().cloned());

    while let Some(current_path) = stack.pop() {
        if current_path.is_dir() && !should_ignore(&current_path, &ignores) {
            for entry in fs::read_dir(&current_path)? {
                if let Ok(direntry) = entry {
                    let path = direntry.path();
                    if !should_ignore(&path, &ignores) {
                        stack.push(path);
                    }
                }
            }
        } else {
            let full_path = current_path.to_str().unwrap_or("ERR");
            entries.push(full_path.to_string());
        }
    }

    Ok(LzList { entries })
}
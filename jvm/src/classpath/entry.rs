use std::fmt::Debug;
use std::io::Read;
use std::path::Path;
use walkdir::WalkDir;

#[cfg(target_os = "windows")]
const PATH_SEPARATOR: &str = "\\";
#[cfg(target_os = "linux")]
const PATH_SEPARATOR: &str = "/";
#[cfg(target_os = "macos")]
const PATH_SEPARATOR: &str = "/";

#[cfg(target_os = "windows")]
const PATH_LIST_SEPARATOR: &str = ";";
#[cfg(target_os = "linux")]
const PATH_LIST_SEPARATOR: &str = ":";
#[cfg(target_os = "macos")]
pub const PATH_LIST_SEPARATOR: &str = ":";
pub const CLASS_SEPARATOR: &str = ".";
pub const CLASS_EXTENSION: &str = "class";
pub const JAR_EXTENSION: &str = "jar";

pub trait Entry {
    fn string(&self) -> &String;
    fn read_class(&self, class_name: &str) -> anyhow::Result<Vec<u8>>;
}

pub fn new_entry(path: String) -> anyhow::Result<Box<dyn Entry>> {
    if path.contains(PATH_LIST_SEPARATOR) {
        Ok(Box::new(CompositeEntry::new(path)?))
    } else if path.to_lowercase().ends_with(".jar") {
        Ok(Box::new(ZipEntry::new(path)?))
    } else if path.to_lowercase().ends_with('*') {
        Ok(Box::new(CompositeEntry::from_wildcard(path)?))
    } else {
        Ok(Box::new(DirEntry::new(path)?))
    }
}

#[derive(Debug)]
pub struct DirEntry {
    path: String,
}

impl DirEntry {
    pub fn new(path: String) -> anyhow::Result<Self> {
        let new_path = Path::new(&path);
        if new_path.exists() && new_path.is_dir() {
            Ok(Self { path })
        } else {
            Err(anyhow::anyhow!("{} is not a directory", path))
        }
    }
}

impl Entry for DirEntry {
    fn string(&self) -> &String {
        &self.path
    }

    fn read_class(&self, class_name: &str) -> anyhow::Result<Vec<u8>> {
        let path = format!(
            "{}{}{}",
            self.path.trim_end_matches(PATH_SEPARATOR),
            PATH_SEPARATOR,
            class_name
        );
        let file = std::fs::File::open(path.as_str())?;
        let bytes: Vec<u8> = file.bytes().map(Result::unwrap).collect();
        Ok(bytes)
    }
}

#[derive(Debug)]
pub struct ZipEntry {
    path: String,
}

impl ZipEntry {
    pub fn new(path: String) -> anyhow::Result<Self> {
        let new_path = Path::new(&path);
        if new_path.exists()
            && new_path.is_file()
            && new_path.extension() == Some(std::ffi::OsStr::new(JAR_EXTENSION))
        {
            Ok(Self { path })
        } else {
            Err(anyhow::anyhow!("{} is not a valid jar file", path))
        }
    }
}

impl Entry for ZipEntry {
    fn string(&self) -> &String {
        &self.path
    }

    fn read_class(&self, class_name: &str) -> anyhow::Result<Vec<u8>> {
        let file = std::fs::File::open(self.path.as_str())?;
        let mut zip = zip::ZipArchive::new(file)?;
        let mut class = zip.by_name(class_name)?;
        let mut bytes = Vec::new();
        class.read_to_end(&mut bytes)?;
        Ok(bytes)
    }
}

pub struct CompositeEntry {
    entries: Vec<Box<dyn Entry>>,
    path: String,
}

impl CompositeEntry {
    pub fn new(path: String) -> anyhow::Result<Self> {
        let entries: Vec<Box<dyn Entry>> = path
            .split(PATH_LIST_SEPARATOR)
            .map(|path| new_entry(path.to_string()))
            .collect::<anyhow::Result<Vec<Box<dyn Entry>>>>()?;
        Ok(Self { entries, path })
    }

    pub fn from_wildcard(path: String) -> anyhow::Result<Self> {
        let entries = path
            .split(PATH_LIST_SEPARATOR)
            .flat_map(|path| {
                let mut entries: Vec<Box<dyn Entry>> = Vec::new();
                let path = path.trim_end_matches('*');
                let path = Path::new(path);
                if path.exists() && path.is_dir() {
                    let dirs = WalkDir::new(path);
                    for entry in dirs {
                        let entry = entry.unwrap();
                        if entry.file_type().is_file()
                            && entry.path().extension() != Some(std::ffi::OsStr::new(JAR_EXTENSION))
                        {
                            continue;
                        }
                        if let Ok(entry) = new_entry(entry.path().to_str().unwrap().to_string()) {
                            entries.push(entry);
                        }
                    }
                }
                entries
            })
            .collect::<Vec<Box<dyn Entry>>>();
        Ok(Self { entries, path })
    }
}

impl Entry for CompositeEntry {
    fn string(&self) -> &String {
        &self.path
    }

    fn read_class(&self, class_name: &str) -> anyhow::Result<Vec<u8>> {
        for entry in self.entries.iter() {
            match entry.read_class(class_name) {
                Ok(bytes) => {
                    if !bytes.is_empty() {
                        return Ok(bytes);
                    } else {
                        continue;
                    }
                }
                _ => continue,
            }
        }
        Err(anyhow::anyhow!("{} not found", class_name))
    }
}

#[cfg(test)]
mod tests {
    use crate::classpath::entry::{CompositeEntry, DirEntry, Entry, ZipEntry};
    use classfile::get_utf8;
    use std::io::Read;
    use walkdir::WalkDir;

    #[test]
    fn dir_entry() {
        let entry = DirEntry::new("../data/jvm8/".to_string()).unwrap();
        let bytes = entry.read_class("GaussTest.class").unwrap();
        assert_eq!(bytes[..4], [0xCA, 0xFE, 0xBA, 0xBE]);
    }

    #[test]
    fn zip_entry() {
        let entry = ZipEntry::new("../data/jvm8/rt.jar".to_string()).unwrap();
        let bytes = entry.read_class("java/lang/Object.class").unwrap();
        assert_eq!(bytes[..4], [0xCA, 0xFE, 0xBA, 0xBE]);
    }

    #[test]
    fn composite_entry() {
        let entry = CompositeEntry::new("../data/jvm8/rt.jar:../data/jvm8/".to_string()).unwrap();
        let bytes = entry.read_class("GaussTest.class").unwrap();
        assert_eq!(bytes[..4], [0xCA, 0xFE, 0xBA, 0xBE]);
    }

    #[test]
    fn read_jar() {
        let file = std::fs::File::open("../data/jvm8/rt.jar").unwrap();
        let mut zip = zip::ZipArchive::new(file).unwrap();
        for i in 0..zip.len() {
            let mut file = zip.by_index(i).unwrap();
            if file.name().ends_with(".class") {
                let mut bytes = Vec::new();
                file.read_to_end(&mut bytes).unwrap();
                let (_, class_file) = classfile::parse(bytes.as_slice()).unwrap();
                let raw = get_utf8(class_file.constant_pool, class_file.this_class as usize);
                println!("{}", String::from_utf8_lossy(raw));
            }
        }
    }

    #[test]
    fn walk_dir() {
        for entry in WalkDir::new("../data/jvm8/").max_depth(1) {
            let entry = entry.unwrap();
            for path in entry.path().iter() {
                println!("{}", path.to_str().unwrap());
            }
        }
    }
}

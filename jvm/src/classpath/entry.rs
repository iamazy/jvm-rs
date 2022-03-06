use std::io::Read;
use std::path::Path;

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
const PATH_LIST_SEPARATOR: &str = ":";

pub trait Entry {
    fn string(&self) -> &String;
    fn read_class(&self, class_name: &str) -> anyhow::Result<Vec<u8>>;
}

pub fn new_entry(path: String) -> anyhow::Result<Box<dyn Entry>> {
    if path.contains(PATH_LIST_SEPARATOR) {
        Ok(Box::new(CompositeEntry::new(path)?))
    } else if path.ends_with(".jar")
        || path.ends_with(".JAR")
        || path.ends_with(".zip")
        || path.ends_with(".ZIP")
    {
        Ok(Box::new(ZipEntry::new(path)?))
    } else {
        Err(anyhow::anyhow!("Invalid path: {}", path))
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
        let class_path = format!("{}{}{}", self.path, PATH_SEPARATOR, class_name);
        let file = std::fs::File::open(class_path.as_str())?;
        let bytes: Vec<u8> = file.bytes().map(|x| x.unwrap()).collect();
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
            && (new_path.extension() == Some(std::ffi::OsStr::new("zip"))
                || new_path.extension() == Some(std::ffi::OsStr::new("jar")))
        {
            Ok(Self { path })
        } else {
            Err(anyhow::anyhow!("{} is not a valid zip file", path))
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
}

impl Entry for CompositeEntry {
    fn string(&self) -> &String {
        &self.path
    }

    fn read_class(&self, class_name: &str) -> anyhow::Result<Vec<u8>> {
        for entry in self.entries.iter() {
            let bytes = entry.read_class(class_name)?;
            if bytes.len() > 0 {
                return Ok(bytes);
            }
        }
        Err(anyhow::anyhow!("{} not found", class_name))
    }
}

#[cfg(test)]
mod tests {
    use classfile::get_utf8;
    use std::io::Read;

    #[test]
    fn read_jar() {
        let file = std::fs::File::open("../data/jvm8/elasticsearch-sql-core-7.16.3.jar").unwrap();
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
}

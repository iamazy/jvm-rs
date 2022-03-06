use std::io::Read;
use std::path::Path;

const PATH_SEPARATOR: &str = "/";

pub trait Entry {
    fn string(&self) -> &str;
    fn read_class(&self, class_name: &str) -> anyhow::Result<Vec<u8>>;
}

#[derive(Debug)]
pub struct DirEntry {
    absolute_path: String,
}

impl DirEntry {
    pub fn new(absolute_path: String) -> anyhow::Result<Self> {
        let path = Path::new(&absolute_path);
        if path.exists() && path.is_dir() {
            Ok(Self { absolute_path })
        } else {
            Err(anyhow::anyhow!("{} is not a directory", absolute_path))
        }
    }
}

impl Entry for DirEntry {
    fn string(&self) -> &str {
        &self.absolute_path
    }

    fn read_class(&self, class_name: &str) -> anyhow::Result<Vec<u8>> {
        let class_path = format!("{}{}{}", self.absolute_path, PATH_SEPARATOR, class_name);
        let file = std::fs::File::open(class_path.as_str())?;
        let bytes: Vec<u8> = file.bytes().map(|x| x.unwrap()).collect();
        Ok(bytes)
    }
}

#[derive(Debug)]
pub struct ZipEntry {
    absolute_path: String,
}

impl ZipEntry {
    pub fn new(absolute_path: String) -> anyhow::Result<Self> {
        let path = Path::new(&absolute_path);
        if path.exists()
            && path.is_file()
            && (path.extension() == Some(std::ffi::OsStr::new("zip"))
                || path.extension() == Some(std::ffi::OsStr::new("jar")))
        {
            Ok(Self { absolute_path })
        } else {
            Err(anyhow::anyhow!("{} is not a valid zip file", absolute_path))
        }
    }
}

impl Entry for ZipEntry {
    fn string(&self) -> &str {
        &self.absolute_path
    }

    fn read_class(&self, class_name: &str) -> anyhow::Result<Vec<u8>> {
        let file = std::fs::File::open(self.absolute_path.as_str())?;
        let mut zip = zip::ZipArchive::new(file)?;
        let mut class = zip.by_name(class_name)?;
        let mut bytes = Vec::new();
        class.read_to_end(&mut bytes)?;
        Ok(bytes)
    }
}

type CompositeEntry = Vec<Box<dyn Entry>>;

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

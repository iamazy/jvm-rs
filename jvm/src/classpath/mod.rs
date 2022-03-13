use crate::classpath::entry::{new_entry, CompositeEntry, Entry, CLASS_EXTENSION};
use std::path::{Path, PathBuf};

mod entry;

pub struct ClassPath {
    pub bootstrap: Box<dyn Entry>,
    pub extension: Box<dyn Entry>,
    pub user: Box<dyn Entry>,
}

impl ClassPath {
    pub fn new(jre_opt: String, cp_opt: String) -> ClassPath {
        let user = parse_user_class_path(cp_opt);
        match parse_boot_ext_class_path(jre_opt) {
            Ok((bootstrap, extension)) => ClassPath {
                bootstrap,
                extension,
                user,
            },
            Err(e) => panic!("{}", e),
        }
    }
}

fn parse_user_class_path(mut cp_opt: String) -> Box<dyn Entry> {
    if cp_opt.is_empty() {
        cp_opt = ".".to_string();
    }
    match new_entry(cp_opt) {
        Ok(entry) => entry,
        Err(e) => {
            panic!("can not parse user class path, {}", e);
        }
    }
}

fn parse_boot_ext_class_path(jre_opt: String) -> anyhow::Result<(Box<dyn Entry>, Box<dyn Entry>)> {
    match get_jre_dir(&jre_opt) {
        Ok(path) => {
            let boot_path = path.join("lib").join("*");
            let bootstrap = Box::new(CompositeEntry::from_wildcard(
                boot_path.to_str().unwrap().to_string(),
            )?);
            let ext_path = path.join("lib").join("ext").join("*");
            let extension = Box::new(CompositeEntry::from_wildcard(
                ext_path.to_str().unwrap().to_string(),
            )?);
            Ok((bootstrap, extension))
        }
        Err(e) => {
            panic!("Failed to get jre dir: {}", e);
        }
    }
}

fn get_jre_dir(jre_opt: &str) -> anyhow::Result<PathBuf> {
    if !jre_opt.is_empty() {
        let path = PathBuf::from(jre_opt);
        if path.exists() {
            return Ok(path);
        }
        let path = PathBuf::from("./jre");
        if path.exists() {
            return Ok(path);
        }
    }

    return match std::env::var("JAVA_HOME") {
        Ok(path) => {
            let java_home = Path::new(&path);
            let path = java_home.join("jre");
            if path.exists() {
                Ok(path)
            } else {
                Err(anyhow::anyhow!(
                    "JAVA_HOME is set, but jre folder not found"
                ))
            }
        }
        Err(e) => Err(anyhow::anyhow!("{}", e)),
    };
}

impl Entry for ClassPath {
    fn string(&self) -> &String {
        self.user.string()
    }

    fn read_class(&self, class_name: &str) -> anyhow::Result<Vec<u8>> {
        let class_name = format!("{}.{}", class_name, CLASS_EXTENSION);
        if let Ok(bytes) = self.bootstrap.read_class(class_name.as_str()) {
            return Ok(bytes);
        }
        if let Ok(bytes) = self.extension.read_class(class_name.as_str()) {
            return Ok(bytes);
        }
        self.user.read_class(class_name.as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::classpath::entry::Entry;
    use crate::classpath::ClassPath;

    #[test]
    fn read_class() {
        let class_path = ClassPath::new("".to_string(), "".to_string());
        let bytes = class_path.read_class("java/lang/Object").unwrap();
        assert_eq!(bytes[..4], [0xCA, 0xFE, 0xBA, 0xBE]);
    }
}

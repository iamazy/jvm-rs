use crate::classpath::{ClassPath, Entry};
use crate::rtda::heap::class::Class;
use anyhow::anyhow;
use dashmap::DashMap;
use std::ptr::NonNull;

const OBJECT_CLASS_NAME: &str = "java/lang/Object";

pub struct ClassLoader {
    class_path: ClassPath,
    pub class_map: DashMap<String, NonNull<Class>>,
}

impl ClassLoader {
    pub fn new(class_path: ClassPath) -> Self {
        ClassLoader {
            class_path,
            class_map: DashMap::new(),
        }
    }

    pub fn load_class(&self, name: &str) -> anyhow::Result<&mut Class> {
        if self.class_map.contains_key(name) {
            let class = unsafe { self.class_map.get_mut(name).unwrap().as_mut() };
            return Ok(class);
        }
        let data = self.read_class(name)?;
        let class = self.define_class(data.as_slice())?;
        self.class_map
            .insert(name.to_string(), Box::leak(Box::new(class)).into());
        let class = unsafe { self.class_map.get_mut(name).unwrap().as_mut() };
        Ok(class)
    }

    fn read_class(&self, name: &str) -> anyhow::Result<Vec<u8>> {
        self.class_path.read_class(name)
    }

    pub fn define_class(&self, data: &[u8]) -> anyhow::Result<Class> {
        let mut class = parse_class(data)?;
        if class.name != OBJECT_CLASS_NAME {
            let super_class = self.load_class(class.super_class_name.as_ref().unwrap())?;
            class.super_class = Some(NonNull::from(super_class));
        }
        let interface_count = class.interface_names.len();
        if interface_count > 0 {
            for idx in 0..interface_count {
                let class_ref = self.load_class(&class.interface_names[idx])?;
                class.interfaces.push(NonNull::from(class_ref));
            }
        }
        class.loader = NonNull::from(self);
        Ok(class)
    }
}

fn parse_class(data: &[u8]) -> anyhow::Result<Class> {
    match classfile::parse(data) {
        Ok((_, ref class_file)) => Ok(Class::new(class_file)),
        Err(e) => Err(anyhow!("parse class error: {}", e)),
    }
}

fn link_class(class: &Class) {
    verify_class(class);
    prepare_class(class);
}

fn verify_class(_class: &Class) {}

fn prepare_class(_class: &Class) {}

#[cfg(test)]
mod tests {
    use crate::classpath::{ClassPath, Entry};
    use crate::rtda::heap::class::Class;
    use crate::rtda::heap::class_loader::ClassLoader;
    use std::ptr::NonNull;

    fn class_loader_init() -> ClassLoader {
        let class_path = ClassPath::new("".to_string(), "../data/jvm8".to_string());
        ClassLoader::new(class_path)
    }

    #[test]
    fn test_class_loader() {
        let class_loader = class_loader_init();
        let class = class_loader.load_class("User").unwrap();
        assert_eq!(class.name, "User");
        assert_eq!(class.super_class_name.as_ref().unwrap(), "java/lang/Object");
        assert_eq!(class.fields.len(), 3);
        for field in class.fields.iter().as_ref() {
            println!("field: {}", field.borrow().name);
        }

        for method in class.methods.iter().as_ref() {
            println!("method: {}", method.borrow().name);
        }

        let super_class = unsafe { class.super_class.unwrap().as_ref() };
        assert_eq!(super_class.name, "java/lang/Object");
        assert_eq!(super_class.super_class_name, None);
        assert_eq!(super_class.fields.len(), 0);
        for field in super_class.fields.iter().as_ref() {
            println!("field: {}", field.borrow().name);
        }

        for method in super_class.methods.iter().as_ref() {
            println!("method: {}", method.borrow().name);
        }
    }

    #[test]
    fn test_sub_class() {
        let class_loader = class_loader_init();
        let class = class_loader.load_class("User").unwrap();
        let object_class = class_loader.load_class("java/lang/Object").unwrap();

        assert!(class.is_sub_class_of(NonNull::from(object_class)));

        let class_path = ClassPath::new("".to_string(), "../data/jvm8".to_string());
        if let Ok(class_bytes) = class_path.read_class("java/lang/Object") {
            if let Ok((_, ref class_file)) = classfile::parse(class_bytes.as_slice()) {
                let class_inner = Class::new(class_file);
                assert_eq!(class.is_sub_class_of(NonNull::from(&class_inner)), false);
            }
        }
    }
}

use crate::classpath::{ClassPath, Entry};
use crate::rtda::heap::class::Class;
use anyhow::anyhow;
use dashmap::DashMap;
use std::ptr::NonNull;

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

    pub fn load_class(&mut self, name: &str) -> anyhow::Result<Class> {
        if self.class_map.contains_key(name) {
            return unsafe { Ok(*Box::from_raw(self.class_map.get(name).unwrap().as_ptr())) };
        }
        let data = self.read_class(name)?;
        let mut class = self.define_class(data.as_slice())?;
        self.class_map
            .insert(name.to_string(), NonNull::from(&mut class));
        Ok(class)
    }

    fn read_class(&self, name: &str) -> anyhow::Result<Vec<u8>> {
        self.class_path.read_class(name)
    }

    pub fn define_class(&mut self, data: &[u8]) -> anyhow::Result<Class> {
        let mut class = parse_class(data)?;
        if class.name != "java/lang/Object" {
            let super_class = self.load_class(class.super_class_name.as_ref().unwrap())?;
            class.super_class = Some(Box::leak(Box::new(super_class)).into());
        }
        let interface_count = class.interface_names.len();
        if interface_count > 0 {
            for idx in 0..interface_count {
                let class_boxed = self.load_class(&class.interface_names[idx])?;
                class
                    .interfaces
                    .push(Box::leak(Box::new(class_boxed)).into());
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
    use crate::classpath::ClassPath;
    use crate::rtda::heap::class_loader::ClassLoader;

    #[test]
    fn test_class_loader() {
        let class_path = ClassPath::new("".to_string(), "../data/jvm8".to_string());
        let mut class_loader = ClassLoader::new(class_path);
        let class = class_loader.load_class(&"User".to_string()).unwrap();
        assert_eq!(class.name, "User");
        assert_eq!(class.super_class_name.unwrap(), "java/lang/Object");
        assert_eq!(class.fields.len(), 3);
        for field in class.fields.iter().as_ref() {
            println!("field: {}", field.name);
        }

        for method in class.methods.iter().as_ref() {
            println!("method: {}", method.name);
        }

        let super_class = unsafe { class.super_class.unwrap().as_ref() };
        assert_eq!(super_class.name, "java/lang/Object");
        assert_eq!(super_class.super_class_name, None);
        assert_eq!(super_class.fields.len(), 0);
        for field in super_class.fields.iter().as_ref() {
            println!("field: {}", field.name);
        }

        for method in super_class.methods.iter().as_ref() {
            println!("method: {}", method.name);
        }
    }
}

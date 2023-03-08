use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Key {
    Id,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    NotSet,
    Integer(i32),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MyStruct {
    pub a: (Key, Value),
    pub o: Key,
}

fn make_attribute() -> MyStruct {
    MyStruct {
        a: (Key::Custom("abc".into()), Value::String("bar".into())),
        o: Key::Id,
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_in(data: *const u8, len: usize) {
    if !data.is_null() {
        let slice = std::slice::from_raw_parts(data, len);
        let data = bincode::deserialize::<MyStruct>(slice).unwrap();
        assert_eq!(data, make_attribute());
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_out(data: *mut *mut u8) -> usize {
    let p = make_attribute();

    let binary = bincode::serialize(&p).unwrap();
    let b = binary.into_boxed_slice();
    let len = b.len();
    let out: &mut *mut u8 = &mut *data;
    *out = Box::into_raw(b) as *mut u8;
    len
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use serde_generate::SourceInstaller;
    use serde_reflection::Tracer;
    use serde_reflection::TracerConfig;

    use super::*;

    #[test]
    fn it_works() {
        let mut tracer = Tracer::new(TracerConfig::default());

        // Trace the desired top-level type(s).
        tracer.trace_simple_type::<Key>().unwrap();
        tracer.trace_simple_type::<Value>().unwrap();
        tracer.trace_simple_type::<MyStruct>().unwrap();
        let r = tracer.registry().unwrap();

        let config = serde_generate::CodeGeneratorConfig::new("testing".to_string())
            .with_encodings(vec![serde_generate::Encoding::Bincode]);

        let path = env!("CARGO_MANIFEST_DIR");

        let installer =
            serde_generate::cpp::Installer::new(PathBuf::from(path.to_string() + "/include"));
        installer.install_module(&config, &r).unwrap();
        installer.install_serde_runtime().unwrap();
        installer.install_bincode_runtime().unwrap();

        let p = super::make_attribute();

        let b = bincode::serialize(&p).unwrap();
        let d = bincode::deserialize::<MyStruct>(&b).unwrap();
        assert_eq!(p, d);
    }
}

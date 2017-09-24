extern crate cpp_build;

fn main() {
    use std::env;
    use std::path::PathBuf;

    let bin_dir = PathBuf::from(env::var("DEP_CHRONOENGINE_BIN_DIR").unwrap());
    if bin_dir.exists() {
        let current_dir = env::current_dir().unwrap();
        for entry in ::std::fs::read_dir(&bin_dir).unwrap() {
            let entry = entry.unwrap();
            ::std::fs::copy(entry.path(), current_dir.join(entry.path().file_name().unwrap())).unwrap();
        }
    }
    let include_dir = PathBuf::from(env::var("DEP_CHRONOENGINE_INCLUDE_DIR").unwrap());

    let target = env::var("TARGET").unwrap();
    let msvc = target.contains("msvc");

    let mut config = cpp_build::Config::new();
    config.include(&include_dir);
    if !msvc {
        config.flag("-std=c++14");
    }
    config.build("src/lib.rs");
}
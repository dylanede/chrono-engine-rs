extern crate cpp_build;

fn main() {
    use std::env;
    use std::path::PathBuf;

    let bin_dir = PathBuf::from(env::var("DEP_CHRONOENGINE_BIN_DIR").unwrap());
    let current_dir = env::current_dir().unwrap();
    for entry in ::std::fs::read_dir(&bin_dir).unwrap() {
        let entry = entry.unwrap();
        ::std::fs::copy(entry.path(), current_dir.join(entry.path().file_name().unwrap())).unwrap();
    }
    let include_dir = PathBuf::from(env::var("DEP_CHRONOENGINE_INCLUDE_DIR").unwrap());

    cpp_build::Config::new().include(&include_dir).build("src/lib.rs");
}
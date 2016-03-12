extern crate gcc;

use std::env;
use std::path::Path;

static WRAPPER_SOURCES: &'static [&'static str] = &[
    "physics.cpp"
        ];

fn main() {
    // Find the directory that executables will be put in (if any), to copy DLLs to.
    // TODO: See if there's a better way of finding this path.
    let out_dir = env::var("OUT_DIR").unwrap();
    let exe_dir = Path::new(&out_dir)
        .parent().unwrap()
        .parent().unwrap()
        .parent().unwrap();
    // Copy all of lib - in practice this is just DLLs
    let lib_dir = env::var("DEP_CHRONOENGINE_LIB_DIR").unwrap();
    for entry in ::std::fs::read_dir(&lib_dir).unwrap() {
        let entry = entry.unwrap();
        ::std::fs::copy(entry.path(),
                        exe_dir.join(entry.path().file_name().unwrap())).unwrap();
    }
    // Build the wrapper
    let include_dir = env::var("DEP_CHRONOENGINE_INCLUDE_DIR").unwrap();
    let mut cfg = gcc::Config::new();
    cfg
        .cpp(true)
        .include(&include_dir)
        .include(Path::new(&include_dir).join("chrono"))
        .flag("-std=c++11"); // is this cross-platform?
    let wrapper_dir = Path::new("src").join("wrapper");
    for source in WRAPPER_SOURCES {
        cfg.file(wrapper_dir.join(&source));

    }
    cfg.compile("libwrapper.a");
}

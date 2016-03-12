extern crate cmake;

fn main() {

    let mut cfg = cmake::Config::new("chrono");
    cfg.define("BUILD_DEMOS", "OFF")
        .define("BUILD_TESTING", "OFF");
    if cfg!(feature = "cascade") {
        cfg.define("ENABLE_MODULE_CASCADE", "ON");
    }
    if cfg!(feature = "cosimulation") {
        cfg.define("ENABLE_MODULE_COSIMULATION", "ON");
    }
    if cfg!(feature = "fea") {
        cfg.define("ENABLE_MODULE_FEA", "ON");
    }
    if cfg!(feature = "irrlicht") {
        cfg.define("ENABLE_MODULE_IRRLICHT", "ON");
    }
    if cfg!(feature = "matlab") {
        cfg.define("ENABLE_MODULE_MATLAB", "ON");
    }
    if cfg!(feature = "mkl") {
        cfg.define("ENABLE_MODULE_MKL", "ON");
    }
    if cfg!(feature = "opengl") {
        cfg.define("ENABLE_MODULE_OPENGL", "ON");
    }
    if cfg!(feature = "parallel") {
        cfg.define("ENABLE_MODULE_PARALLEL", "ON");
    }
    if cfg!(feature = "postprocess") {
        cfg.define("ENABLE_MODULE_POSTPROCESS", "ON");
    }
    if cfg!(feature = "python") {
        cfg.define("ENABLE_MODULE_PYTHON", "ON");
    }
    if cfg!(feature = "vehicle") {
        cfg.define("ENABLE_MODULE_VEHICLE", "ON");
    }
    let chrono_dst = cfg.build();
    println!("cargo:rustc-link-search=native={}", chrono_dst.join("lib").display());
    println!("cargo:rustc-link-lib=dylib=ChronoEngine");
    println!("cargo:include-dir={}", chrono_dst.join("include").display());
    println!("cargo:lib-dir={}", chrono_dst.join("lib").display());
}

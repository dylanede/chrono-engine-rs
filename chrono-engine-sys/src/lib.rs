#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

#[link(name="ChronoEngine")]
#[cfg_attr(feature = "cascade", link(name="ChronoEngine_cascade"))]
#[cfg_attr(feature = "cosimulation", link(name="ChronoEngine_cosimulation"))]
#[cfg_attr(feature = "fea", link(name="ChronoEngine_fea"))]
#[cfg_attr(feature = "fsi", link(name="ChronoEngine_fsi"))]
#[cfg_attr(feature = "irrlicht", link(name="ChronoEngine_irrlicht"))]
#[cfg_attr(feature = "matlab", link(name="ChronoEngine_matlab"))]
#[cfg_attr(feature = "mkl", link(name="ChronoEngine_mkl"))]
#[cfg_attr(feature = "opengl", link(name="ChronoEngine_opengl"))]
#[cfg_attr(feature = "parallel", link(name="ChronoEngine_parallel"))]
#[cfg_attr(feature = "postprocess", link(name="ChronoEngine_postprocess"))]
#[cfg_attr(feature = "python", link(name="ChronoEngine_python"))]
#[cfg_attr(feature = "vehicle", link(name="ChronoEngine_vehicle"))]
extern {}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

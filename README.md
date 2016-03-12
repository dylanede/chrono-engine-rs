# chrono-engine-rs
[![Build Status](https://travis-ci.org/dylanede/chrono-engine-rs.svg?branch=master)](https://travis-ci.org/dylanede/chrono-engine-rs)

Rust bindings for Chrono Engine, an open source physics engine. See https://github.com/projectchrono/chrono

*This is a WIP - there is no released version yet.*

This crate by default handles building and linking Chrono Engine for you. All it asks is that you have installed **CMake 2.8 or later** and a C++11 compliant C++ compiler for your target platform. Recent versions of clang, gcc and Microsoft Visual C++ should be fine. Building with CMake is handled with the [cmake](https://github.com/alexcrichton/cmake-rs) crate, so cross-compilation should work out of the box.

## Hacking on chrono-engine-rs

Please make sure that you have checked out the chrono submodule in this repository, otherwise the project will not build. When cloning this can be achieved with the `--recursive` flag, i.e.
```bash
git clone https://github.com/dylanede/chrono-engine-rs.git --recursive
```

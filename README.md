[![Build Status](https://travis-ci.org/dem42/copper.svg?branch=master)](https://travis-ci.org/dem42/copper)

# Renderer in rust
Depends on gl_generator to generate opengl function bindings. For this we use the gl_generator's Global generator which will also provide the linking
that you would otherwise need to provide either in your build.rs or somehwere in the code as:
``` #[link="OpenGL32.lib"] extern {} ```

The gl_generator will generate unsafe function bindings and safe wrappers for these are added to gl module.

For opengl context creation, glfw is used, specifically the rust crate glfw-rs. However, for simplicty we don't build glfw as part of the build,
rather the `build.rs` script provides a search path for an already built glfw library.

The only other external dependency is on lodepng-rs for reading in pngs. This library is pure rust, but doesn't have the best performance.

This project started off following the opengl tutorial by ThinMatrix:
https://www.youtube.com/watch?v=VS8wlS9hF8E&list=PLRIWtICgwaX0u7Rf9zkZhLoLuZVfUksDP

## Updating glfw dependency
We don't build glfw from scratch. Rather, in our `build.rs` we point to a local directory that contains the compiled library. 

Sometimes, the github glfw-rs version changes and may depend on a newer version of glfw binary. In that case you need to download a new version.
This will only be necessary if you are installing this project in a clean directory since it does cache older version of its dependencies and tags them in the Cargo.lock. To force cargo to ignore the version that's locked you need to manually specify the version in Cargo.toml

Note that the `build.rs` and the Cargo.toml are located in the sub-crate "gl"

## Building C++ with MSVC
you have a weird system setup where your ucrtd (universal c runtime) is only available with windows 10 sdk 
but you have multiple sdks installed 8.0 and 8.1 and your visual studio 14 2015 expects to have the ucrtd

cmake will try to compile a test c++ project to see everything works and for this it needs to know which windows sdk to use
to override the default one use from the build/ folder
also remember that cmake caches stuff so always clear the cmake folder to start from fresh

```
cmake ../ -DCMAKE_SYSTEM_VERSION=10.0.14393.0
```
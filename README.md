[![Build Status](https://travis-ci.org/dem42/copper.svg?branch=master)](https://travis-ci.org/dem42/copper)

# Renderer in rust

A demonstration of a 3d renderer written from scratch in Rust. 

Features:
- Specular/diffuse lighting implementation
- Water with reflection/refraction and distortion maps
- PCF real-time shadows
- Particle effects (GPU instanced)
- Procedural terrain generation
- Post-processing effects: Gaussian blur, Bloom effect
- Implementation of 3D maths needed for rendering: Matrices, Vectors, LookAt functions, Euler angles and Quaternions
- Normal mapped entities, including calculation of tangent space transform
- Multiple point lights with attenuation
- Geometry shader support and demo
- Environment map support and demo
- Movable, follow-player camera
- Ray-casting for placing objects in the world 
- Skyboxes with day/night cycles
- In-game gui overlays and SDF-shaded gui text
- Texture atlases
- Background thread for loading and decoding textures

## Setup
Depends on gl_generator to generate opengl function bindings. For this we use the gl_generator's Global generator which will also provide the linking
that you would otherwise need to provide either in your build.rs or somehwere in the code as:
``` #[link="OpenGL32.lib"] extern {} ```

The gl_generator will generate unsafe function bindings to opengl functions. Safe wrappers for these are added in the gl sub-crate in this project.

For opengl context creation, glfw is used, specifically the rust crate glfw-rs. Glfw-rs is a rust wrapper on top of glfw so we need to also build glfw which is C++. We do so by depending on the tag that works for us as a git submodule and in `build.rs` we run the cmake that builds it (TODO: so far only working for windows)

The only other external dependency of note is on lodepng-rs for reading in pngs. This library is pure rust, but doesn't have the best performance.

This project started off following the opengl tutorial by ThinMatrix:
https://www.youtube.com/watch?v=VS8wlS9hF8E&list=PLRIWtICgwaX0u7Rf9zkZhLoLuZVfUksDP

## Updating glfw dependency
An alternative to building glfw from scratch is to depend on a binary. In our `build.rs` we could point to a local directory that contains the compiled library. 

Sometimes, the github glfw-rs version changes and may depend on a newer version of glfw binary. In that case you need to download a new version.
This will only be necessary if you are installing this project in a clean directory since it does cache older version of its dependencies and tags them in the Cargo.lock. To force cargo to ignore the version that's locked you need to manually specify the version in Cargo.toml

Note that the `build.rs` and the Cargo.toml are located in the sub-crate "gl"
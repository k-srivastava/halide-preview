# Halide Preview
Halide is meant to be a raytracer written in Rust that runs on the CPU.

In its current state, it supports the raytraced rendering of spheres, quadrilaterals and basic volumetrics. It has support for simple materials, reflection, scattering and lighting. It runs in parallel on the CPU using the `rayon` crate.
This repository is meant to serve as a basic guide for myself and others on easy CPU raytracing in Rust.

There are a few major features which Halide in its preview state lacks:
- BVH's are not implemented correctly.
- Texture support is missing.
- Major opitimizations are required (although, we can get away by deferring this to later stages because it still runs fairly fast for a testing build).

This repository will be archived and read-only and a future version of Halide with potential GPU based raytracing using SPIR-V will soon be up and running.

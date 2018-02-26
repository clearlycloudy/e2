# e2r

[![Build Status](https://travis-ci.org/clearlycloudy/e2r.svg?branch=master)](https://travis-ci.org/clearlycloudy/e2r)

A very experimental/proof of concept/minimalistic 3D rendering engine in Rust language. It is very work in process and major work is in under way to factor and lay out engine components.

# Current implementations:

matrix utilities

render backend using OpenGL

md5mesh & md5anim file format import

# Todos:

Refactoring major components for engine to be localized

Create camera utilities and trajectory controllers

Hook up a number of low level blocks into the kernel: model parsers ( .obj ), spatial accelerators( bvh, gjk )

Create model asset and texture managers

Add better debugging and console input facilities

Implement multithreading support for front end and some of the back end pipelines

Do feasibility study of using physically based rendering approach as an alternative rendering pipeline

# e2r

A very experimental/proof of concept/minimalistic 3D rendering engine in Rust language. It is very work in process and major work is in under way to factor and lay out engine components.

# Current implementations:

matrix utilities

render backend using OpenGL

md5mesh & md5anim file format import

# Todos:

File parsing using nom

Create camera utilities and trajectory controllers

Add texture support

Refactoring major components for engine

Hook up a number of low level blocks into the kernel: model parsers ( .obj ), spatial accelerators( bvh, gjk )

Implement multithreading support for front end and some of the back end pipelines

# Screenshots:

[![IMAGE ALT TEXT](http://img.youtube.com/vi/pDVDkFX23Tc/0.jpg)](https://youtu.be/pDVDkFX23Tc "md5mesh & md5anim")


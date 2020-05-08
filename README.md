# e2r

Experimental game engine in Rust. Work in progress. Goal: scalable client-server architecture and game.

# Current implementations:

matrix utilities

render backend using OpenGL

md5mesh & md5anim file format import

# Todos:

File parsing using nom for geometry files

Fine tune camera utilities and trajectory controllers

Add simple 3D texture support

Add support for 2D rendering for menu/inventory

Refactoring major components for engine for performance
- move game service logic to backend
- add socket communication layer between frontend and backend
- add multi-threading support

# Screenshots:

[![IMAGE ALT TEXT](http://img.youtube.com/vi/pDVDkFX23Tc/0.jpg)](https://youtu.be/pDVDkFX23Tc "md5mesh & md5anim")


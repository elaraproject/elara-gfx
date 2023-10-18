# Elara GFX

A GPU programming library for Project Elara, focused on functionality and speed.

**Note:** Elara GFX is developed concurrently with [NanoGL](https://github.com/Songtech-0912/nanogl). Unlike its sister library, Elara GFX _does not_ aim to be lightweight or zero-dependency, and will not be backwards-compatible until the release of 1.0.

**Shoutouts:** See [Acknowledgements](./ACKNOWLEDGEMENTS.md)

## Demos

These demos use a variety of fragment shaders from Shadertoy as well as custom-created shaders, and all use only the library's functionality. Note that by default the library renders to `.ppm` images, these can be converted to PNGs with ImageMagick.

![Black hole](samples/black_hole_render.png)
![Glowing circle](samples/circle.png)
![Gradient](samples/gradient.png)
![Grid](samples/grid.png)
![Orbits](samples/orbits.png)
![UI rendering](samples/ui-render.png)


## Install

Make sure to clone the repository and grab submodules:

```
git clone --recursive https://github.com/elaraproject/elara-gfx.git
```

On macOS and Windows (untested), Elara GFX should compile without any need to download additional libraries. On many Linux distributions, this is also the case, but if Elara GFX does not compile out of the box, follow these instructions.

### Debian-based Linux

```sh
sudo apt install xorg-dev mesa-utils libglu1-mesa-dev freeglut3-dev mesa-common-dev
```

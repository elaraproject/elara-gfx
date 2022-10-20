# Elara GFX

A GPU programming library for Project Elara, focused on functionality and speed.

**Note:** Elara GFX is developed concurrently with [NanoGL](https://github.com/Songtech-0912/nanogl). Unlike its sister library, Elara GFX _does not_ aim to be lightweight or zero-dependency.

**Shoutouts:** See [Acknowledgements](./ACKNOWLEDGEMENTS.md)

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

# HTGL: An OpenGL hiking trail

## An OpenGL wrapper library targeting OpenGL 3.3 in Rust

The version 3.3 of OpenGL is targeted as it's the newest version available on all three of Linux (when using the open drivers), OS X and Windows.

### Dependencies

- gl-rs, the library for which this one essentially is a wrapper.
- glfw-rs, only used in the test application, not by the lib itself. The idea is to the app away or at last make this dependency optional.

Naturally, the Cargo.toml is the definitive source for this kind of information.

### What it tries to do

- Safety and sensibility: It should not be possible to do insane things. However, the library still won't take responsibility out of your hands.
- Simplicity: The library should be straightforward to use. If you know OpenGL, it shouldn't be too hard to use HTGL. There definitely are new concepts around and the entire landscape of possible ways to use OpenGL is not open, just a narrow trail. That's the price of safety (and this library being a hobby project).
- Performance: This library won't conjure more performance out of thin air, and may not expose the fast paths perfectly, but at least it attempts to avoid redundant state changes.
- Work on Linux, Windows and OS X, but as my chances to test on all of them are limited, this is mostly a thing of principle right now, to not include platform specifics into the code.

### What it doesn't try to do

- To allow using everything in the underlying API. Especially the deprecated parts are not going to be exposed.
- Extensions and other special casing. Working around driver-specific bugs might happen, though. Things need to be judged in a case by case basis.
- Multi-threading, which, as far as I know, doesn't actually work with OpenGL that well. No performance benefits (or very limited benefits) are not worth the increased complexity.

### The name

Before OpenGL 3 was published, there was the code name Longs Peak, and now there is Vulkan. While hiking trails aren't the most mountainy things, the concept seemed like a fitting base for a name. The library also tries to provide a narrow but relatively safe path for OpenGL.
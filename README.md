# `raytracer_ini`
Basic raytracer that reads .ini files as its input for a scene and renders them.

## Basic usage

Render the `basic_scene.ini` scene with the `basic_observer.ini` observer at a resolution of 2000x2000 in png format: 
```
./raytracer_ini -s config/basic_scene.ini -O config/basic_observer.ini -o rendered_example.png 2000
```

You'll most likely define the observer sections and the scene sections in the same file. If that is the case may specify only a scene, as is the case with `final_scene.ini`.
```
./raytracer_ini -s config/final_scene.ini -o rendered_example.png 2000
```

Pass the `--help` flag for more information.

### Supported image formats

The final image format is determined by the output file extension. The available image formats are those [supported by the image crate](https://github.com/image-rs/image#supported-image-formats).

## Config files

The config files are written with `.ini` format. This means that each section is denoted by [brackets] and the values for each section are denoted as key=value pairs, and each section must have a **unique** name. For the config files specific to this raytracer each object in the scene, along with the overall scene parameters, observer camera, and projection plane, get a unique section. For objects the type of object (the type of **primitive**) is denoted by the start of the name of the section. For example, a section denoting a sphere must have its name start with "Sphere ...".

### Available primitives
- Spheres
- Cylinders (uncapped)
- Discs
- Planes (infinte)
- Triangles

### Example config files
- Well documented scene example: [config/basic_scene.ini](./config/basic_scene.ini) 
- Well documented observer example: [config/basic_observer.ini](./config/basic_observer.ini)

These files together produce the following image:
<img src="images/example_render.png" alt="A scene showcasing the various primitives, checkerboard texture and light effects available." width="800" />

There's more examples in the [config/](./config) folder that aren't documented.

## Missing functionality

This is a non-exhaustive list of things I think I could add at some point.

### Light model

- Currently transparent surfaces do create shadows but not full shadows, depending on their level of transparency the level of light that they block. But colored transparent surfaces don't alter the color of the light that passes through them, it only attenuates it.

- Speculative reflection doesn't seem to work properly on transparent surfaces (imagine the shine on a bubble).

### Primitives

- Let cylinders have caps (https://www.cl.cam.ac.uk/teaching/1999/AGraphHCI/SMAG/node2.html#SECTION00023200000000000000)
- Let primitives be arbitrarily cut by planes
- Finite planes (squares)

### Performance

- This is the first iteration of the raytracer and I haven't gone back to look at some of the math-y portions, I'm sure there's some optimizations to be had (specially with the ugly matrix code in vec3.rs)
- Adding parallelization

### Features

- Anti-aliasing (with flag to toggle)
- Add a flag to toggle shadows

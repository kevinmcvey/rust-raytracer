Rust-Raytracer
==============

Rust-Raytracer is a complete 3D renderer written in Rust!  This program simulates the movement of light in a user-provided scene in order to produce "realistic" images with great mathematic accuracy.  Some key features include:

* **Concurrency**: Lightweight "rust tasks" are spawned for each ray, taking advantage of the safety and power of Rust
* **Accuracy**: Phong and Lambertian shading schemes are used to make scene objects react realistically to light
* **Customization**: Users can create their own scenes by modifying load.ray
* **Universal Output**: PPM file output is lightweight and multi-platform, Rust-Raytracer renders directly to render.ppm

Using rust-raytracer is a snap: simply download geometry.rs and load.ray, compile geometry.rs, and run!

Rust-Raytracer was created with love by Kevin McVey, Kelvin Green, Nathaniel Hart, and Jon Goss from the University of Virginia as a part of [CS4414 - Operating Systems](http://rust-class.org/), the world's first collegiate course to use [Rust](http://www.rust-lang.org/).

Scene Setup Commands
--------------------

Rust-Raytracer automatically loads scene-graph definitions from the included load.ray file.  Its markup is documented below.  Note that the coordinate system has an origin at (0, 0, 0) and negative values are acceptable.  Colors are defined using floating point numbers ranging from 0 to 1; (0.0, 0.5, 1.0) for instance, would render the hex color #007FFF.

    #camera
    px py pz
    dx dy dz
    ux uy uz
    rx ry rz
    ha

This defines a camera placed as position (px, py, pz), facing in direction (dx, dy, dz), with up-vector (ux, uy, uz), and right-vector (rx, ry, rz).  The "height angle" is given by ra such that the "width angle" can be found by multiplying this value by the aspect ratio of the image.  0.5 is used for a square.

    #background
    r g b

This defines a background of color (r, g, b).

    #ambient
    r g b

This defines the scene's ambient light at color (r, g, b).

    #material
    er eg eb
    ar ag ab
    dr dg db
    sr sg sb sfo

This defines a material that can be applied to a shape.  The shape's emissive, ambient, diffuse, and specular light properties are defined as rgb color triplets as shown above.  "sfo" denotes the specular falloff of the specular highlight of the material.  This ranges from 0.0 to 128.0.

    #shape_sphere mat
    px py pz
    r

This defines a sphere centered at position (px, py, pz) with radius r.  The material given to it is an integer in place of "mat."  Note that materials are zero-indexed, 0 refers to the first material.

    #vertex
    px py pz
    nx ny nz

This defines a vertex in space placed at position (px, py, pz) with normal vector direction of (nx, ny, nz).

    #shape_triangle mat
    i1 i2 i3

This defines a triangle comprised of three previously defined vertices.  i1, i2, and i3 denote zero-indexed vertices already found in the ray file.  The material given to this shape is an integer in place of "mat."  Note that materials are zero-indexed, 0 refers to the first material.

    #light_dir
    r g b
    dx dy dz

This defines a directional light source of color rgb with light shining in the direction of vector (dx, dy, dz).

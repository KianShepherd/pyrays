# PyRays

A Python library that can be used to create scenes of simple objects and then raytrace them to create
photorealistic renders of the scene. The backend that actually  ratraces the scene is written in Rust
and is an extension of [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

![Example Render1](https://gitlab.com/kian_shepherd/pyrays/-/raw/main/examples/example1.jpg)

![Example Render2](https://gitlab.com/kian_shepherd/pyrays/-/raw/main/examples/example2.jpg)

![Example Render3](https://gitlab.com/kian_shepherd/pyrays/-/raw/main/examples/test_scene/example.png)

## Features

1. Materials 
    * Lambertian
    * Metal
    * Dielectric (IN PROGRESS)
    * Mirror
2. Lighting
    * Multiple Point Lights
    * Shadows
3. Camera
    * Movable
    * Defocus Blur
4. Shapes
    * Spheres
    * Triangles (with optional back face culling)
5. Multi-Threading
   * Benchmarks
     * Release mode on 8 Core CPU
     * 720p procedural gen
         * multi-threaded: 0h : 14m : 27s
         * single thread: 1h : 7m : 35s
         * 4.7x speedup
     * 400p test scene
         * multi-threaded: 0m : 19s
         * single thread: 1m : 51s
         * 5.8x speedup
6. Octree / K-tree Optimization (TODO)

# PyRays

A Python library that can be used to create scenes of simple objects and then raytrace them to create
photorealistic renders of the scene. The backend that actually  ratraces the scene is written in Rust
and is an extension of [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

![Example Render1](https://gitlab.com/kian_shepherd/pyrays/-/raw/main/examples/example_proc2/proc2.png)

![Example Render2](https://gitlab.com/kian_shepherd/pyrays/-/raw/main/examples/example_proc1/proc1.png)

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
    * Procedural terrain (height mapped plane of triangles, with height mapped colouring)
5. Optimizations
   * Multi-Threading
     * Release mode on 8 Core CPU
     * Note these benchmarks took place before the Octree optimization was implemented
     * 720p procedural gen
         * multi-threaded: 0h : 14m : 27s
         * single thread: 1h : 7m : 35s
           * Out of place note here about a more recent performance comparison.
             We can now raytrace a higher resolution image (2880 x 1620) at higher raycounts per pixel and depth,
             with 8 million triangles total in the scene at this runtime (this scene was 2000 triangles total).
         * 4.7x speedup
     * 400p test scene
         * multi-threaded: 0m : 19s
         * single thread: 1m : 51s
         * 5.8x speedup
    * Use rust intrisic float operations
        * Similar to the C `-ffast-math` flag for all hot path floating point operations
        * ~1.5x speedup on every ray object interaction
    * Replaced handmade Vec3 implementation with the Rust `glam` library for the Vec3A type
        * Using a nightly compiler we can opt into `core-simd` for multi-platform SIMD vector support.
        * We also opt into fast math here to further improve performance at the cost of some float accuracy.
          * I've done a few test renders of the same scene and there is no visible difference but a massive
            performance improvement.
    * Octree / Bounding Volume Hierarchies
        * Rather than check every object against each ray we can split the objects into smaller BVH's.
          This makes it feasible to raytrace scenes with millions of objects as we can ray object check
          multiple containers and thier sub containers until we get a much smaller list of objects to actually
          raytrace against
        * The speedup here varies but in a scene with 1000 objects the runtime went from about 4 minutes
          down to 9 seconds.
        * There is a small runtime cost to creating the BVH but it still massively outperforms not using one
          even on small scales.

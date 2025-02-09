"""Example scene of 3 spheres in a box missing a face, with lighting."""

import pyrays

image_width = 7680
aspect_ratio = 9.0 / 16.0
image_height = int(aspect_ratio * image_width)
samples_per_pixel = 750
max_depth = 250
v_fov = 90
aperture = 0.01
focal_distance = 3.5
camera_pos = [0, 0, -3.12]
camera_look_at = [0, 0, 0]
camera_up = [0, 1, 0]
multithreading = True

scene = pyrays.Scene(camera_pos, camera_look_at, camera_up, focal_distance, aperture, v_fov)

scene.add_light([-1.0, 1.5, -3.5])

scene.add_object(pyrays.Sphere([0.6, 0.0, -1.5], 0.5, pyrays.Metal([0.7, 0.6, 0.2], 0.3)))
scene.add_object(pyrays.Sphere([-0.9, -1.0, -1.2], 0.5, pyrays.Mirror()))
scene.add_object(pyrays.Sphere([-0.7, 0.8, -1.2], 0.5, pyrays.Diffuse([0.9, 0.0, 0.8])))

scene.add_object(pyrays.Square([2.0, -2.0, 0.0],
                               [-2.0, -2.0, 0.0],
                               [-2.0, 2.0, 0.0],
                               [2.0, 2.0, 0.0],
                               pyrays.Diffuse([0, 0.6, 0])))
scene.add_object(pyrays.Square([-2.0, -2.0, 0.0],
                               [-2.0, -2.0, -2.0],
                               [-2.0, 2.0, -2.0],
                               [-2.0, 2.0, 0.0],
                               pyrays.Diffuse([0.6, 0, 0])))
scene.add_object(pyrays.Square([2.0, -2.0, -2.0],
                               [2.0, -2.0, 0.0],
                               [2.0, 2.0, 0.0],
                               [2.0, 2.0, -2.0],
                               pyrays.Diffuse([0.9, 0.9, 0.0])))
scene.add_object(pyrays.Square([2.0, 2.0, 0.0],
                               [-2.0, 2.0, 0.0],
                               [-2.0, 2.0, -2.0],
                               [2.0, 2.0, -2.0],
                               pyrays.Diffuse([0, 0, 0.9])))
scene.add_object(pyrays.Square([-2.0, -2.0, 0.0],
                               [2.0, -2.0, 0.0],
                               [2.0, -2.0, -2.0],
                               [-2.0, -2.0, -2.0],
                               pyrays.Diffuse([0.7, 0.0, 0.9])))

image = scene.raytrace(image_width, image_height, samples_per_pixel, max_depth, multithreading)

image.show()
image.save('img.png')

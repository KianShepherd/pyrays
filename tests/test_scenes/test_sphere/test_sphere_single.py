import pyrays

image_width = 100
image_height = 57
samples_per_pixel = 100
max_depth = 25
v_fov = 90
aperture = 0.01
focal_distance = 1.5
camera_pos = [0, 0.0, -2.0]
camera_look_at = [0, 0, 0]
camera_up = [0, 1, 0]
multithreading = False

scene = pyrays.Scene(camera_pos, camera_look_at, camera_up, focal_distance, aperture, v_fov)

#scene.add_light([-1.0, 1.5, -3.5])


scene.add_object(pyrays.Sphere([0.0, 0.0, 0.0], 1, pyrays.Diffuse([0,  0, 1])))
image = scene.raytrace(image_width, image_height, samples_per_pixel, max_depth, multithreading)

image.save('tests/test_scenes/test_sphere/test_sphere_single.png')

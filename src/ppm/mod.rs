use std::{fs::{self, File}, io::{Read, Write}, rc::Rc, sync::Arc};
use std::fs::OpenOptions;

use crate::{hittable_list::HittableList, hittable::{Hittable, HitRecord}, rtweekend::random_double, vec3::Colour, ray::Ray};
use crate::vec3::{Vec3, Point3};
use crate::sphere::Sphere;
use crate::camera::Camera;

pub struct Ppm {}

impl Ppm {
    pub fn render(name: &str) {
        // Image
        let image_width = 400;
        let aspect_ratio = 16.0 / 9.0;
        let image_height = (image_width as f64 / aspect_ratio ) as i32;
        let samples_per_pixel = 100;



        // World
        let world: Box<dyn Hittable + 'static> = Box::new(HittableList {
            objects: vec![
                Box::new(Sphere::new(Point3::fill(0, 0, -1), 0.5)),
                Box::new(Sphere::new(Point3::fill(0.0, -100.5, -1.0), 100.0)),
            ],
        });

        // Camera
        let camera = Camera::new();


        // Render
        // Self::write_to_ppm("listing1.ppm", format!("P3\n{} {}\n255\n", image_width, image_height));
        let mut output_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(format!("{}.ppm", name))
            .unwrap();
        let mut text_output_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(format!("{}.txt", name))
            .unwrap();
        //print!("P3\n{} {}\n255\n", image_height, image_width);
        write!(output_file, "P3\n{} {}\n255\n", image_width, image_height).expect("Failed to write header");
        write!(text_output_file, "P3\n{} {}\n255\n", image_width, image_height).expect("Failed to write header");

        // let mut writer = BufWriter::new(output_file);
        for j in (0..image_height).rev() {
            println!("\rScanlines remaining: {} ", j);
            for i in 0..image_width {
                /* let u = i as f64 / (image_width as f64 - 1.0);
                let v = j as f64 / (image_height as f64 - 1.0);

                let mut ray: Ray = Ray {
                    origin,
                    direction: lower_left_corner + (u * horizontal) + (v * vertical),
                };

                let ray_colour = ray.get_colour(&world); */

                let mut pixel_colour = Colour::fill(0, 0, 0);

                for _s in 0..samples_per_pixel {
                    let u = (i as f64 + random_double()) / (image_width - 1) as f64;
                    let v = (j as f64 + random_double()) / (image_height - 1) as f64;

                    let mut ray = camera.get_ray(u, v);
                    pixel_colour += ray_colour(ray, &world)
                }
                
                write!(output_file, "{}", pixel_colour.format_colour(samples_per_pixel)).expect("Unable to write ray colour to ppm file.");
                write!(text_output_file, "{}", pixel_colour.format_colour(samples_per_pixel)).expect("Unable to write ray colour to ppm file.");

            }
        }
        println!();
        println!("Done!");
    }

    fn write_to_ppm(file_name: &str, content: String) {
        fs::write(file_name, content).expect("Failed to write content to {file_name}");
    }

    fn read_from_ppm(file_name: &str) -> String {
        let mut data = String::new();
        let mut file = File::open(file_name).expect("Unable to open file {file_name}");
        file.read_to_string(&mut data).expect("Unable to read data");

        return data;
    }
}


pub fn ray_colour(ray: Ray, world: &Box<dyn Hittable>) -> Colour {

    //let rec: HitRecord;
    let mut rec = HitRecord::default(); // I think this is throwing off the colour

    if world.hit(&ray, 0.0, crate::rtweekend::INFINITY, &mut rec) {
        let target: Point3 = rec.point + rec.normal + Vec3::random_in_unit_sphere();
        return 0.5 * ray_colour(Ray::new(rec.point,  target - rec.point), world)
    }
    
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);


    return (1.0 - t) * Colour::fill(1, 1, 1) + t * Colour::fill(0.5, 0.7, 1.0);

}
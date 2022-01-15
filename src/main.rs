mod raytracer;
mod shapes;
mod vec3;

use vec3::Vec3;

fn main() {
    // vec 3 stuff
    let v = Vec3::new(10.5, 69.5, 30.5);
    let w = Vec3::new(5.3, 69.5, -10.0);
    println!(
        "norm: {}\n normalized: {:?}\n v-w: {:?},\n w-v: {:?}\n w+v: {:?}\n normalized norm: {}",
        v.norm(),
        v.normalize(),
        v - w,
        w - v,
        v + w,
        w.normalize().norm()
    );

    let vnorm = v.normalize();
    let wnorm = w.normalize();

    println!(
        "\n normalized \"norm\" without sqrt:\n\tv: {}\n\tw: {}",
        vnorm.x.powi(2) + vnorm.y.powi(2) + vnorm.z.powi(2),
        wnorm.x.powi(2) + wnorm.y.powi(2) + wnorm.z.powi(2)
    );
}

use crate::raytracer::Ray;
use crate::vec3::Vec3;
use enum_dispatch::enum_dispatch;

struct Sphere {
    center: Vec3,
    r: f64,
}

impl ShapeCalculations for Sphere {
    fn get_intersection(&self, ray: Ray) -> Option<f64> {
        let anchor = ray.anchor;
        let dir = ray.dir;
        let center = self.center;

        let b = 2.0
            * (dir.x * (anchor.x - center.x)
                + dir.y * (anchor.y - center.y)
                + dir.z * (anchor.z - center.z));
        let c = (anchor.x - center.x).powi(2)
            + (anchor.y - center.y).powi(2)
            + (anchor.z - center.z).powi(2)
            - self.r * self.r;

        let determinant = (b * b - 4.0 * c).sqrt();

        if determinant.is_nan() {
            None
        } else {
            let t1 = (-b - determinant) / 2.0;
            let t2 = (-b + determinant) / 2.0;
            if t1 > 0.0 {
                Some(t1)
            } else if t2 < 0.0 {
                None
            } else {
                panic!("No está implementado el caso de la cámara dentro de una esfera");
                // Normalmente se retornaría t2
            }
        }
    }
}

#[enum_dispatch]
trait ShapeCalculations {
    /// Returns the distance "t" from the camera to the point
    fn get_intersection(&self, ray: Ray) -> Option<f64>;
}

#[enum_dispatch(ShapeCalculations)]
enum Shape {
    Sphere,
}

use rand::{self, Rng};

// Constants
pub const INFINITY: f64 = std::f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;



// Utility Functions
pub fn degrees_to_radians(degree: f64) -> f64 {
    return degree * PI / 180.0;
}

pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();

    return rng.gen::<f64>();
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    return min + (max-min) * random_double();
}


pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    
    match x {
        x if x < min => return min,
        x if x > max => return max,
        _ => return x
    
    }
}
// single, non mutable, precise memory address for R i.e. (static R: f64)
static R: f64 = 6371.0; // Earth radius in kilometers

pub fn haversine_dist(mut th1: f64, mut ph1: f64, mut th2: f64, ph2: f64) -> f64 {
    ph1 -= ph2;
    ph1 = ph1.to_radians();
    th1 = th1.to_radians();
    th2 = th2.to_radians();
    let dz: f64 = th1.sin() - th2.sin();
    let dx: f64 = ph1.cos() * th1.cos() - th2.cos();
    let dy: f64 = ph1.sin() * th1.cos();
    ((dx * dx + dy * dy + dz * dz).sqrt() / 2.0).asin() * 2.0 * R
}


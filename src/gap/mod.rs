pub fn haversine_dist(mut th1: f64, mut ph1: f64, mut th2: f64, ph2: f64) -> f64 {
    ph1 -= ph2;
    ph1 = ph1.to_radians();
    th1 = th1.to_radians();
    th2 = th2.to_radians();
    let dz: f64 = th1.sin() - th2.sin();
    let dx: f64 = ph1.cos() * th1.cos() - th2.cos();
    let dy: f64 = ph1.sin() * th1.cos();
    let r: f64 = 6371.0; // Earth radius in kilometers
    let km_to_mi = 1.609344_f64; // convert default kilometers to miles.
    let d: f64 = ((dx * dx + dy * dy + dz * dz).sqrt() / 2.0).asin() * 2.0 * r;
    let d = ((d / km_to_mi) * 10.0).round() / 10.0; // convert distance to 1 decimal point.
    d // To be returned
}

pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

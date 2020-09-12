// Convert String to static str.
fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub fn title(vec_len: usize) -> Vec<String> {
    let mut header: Vec<String> = Vec::new();

    for i in 0..vec_len {
        if i == 0 {
            header.push("KEY".to_owned())
        } else if i == vec_len - 1 {
            header.push("DISTANCE".to_owned())
        } else {
            let mut a: String = "STATE_".to_owned();
            let b: usize = i;
            let b: String = b.to_string();
            let b: &'static str = string_to_static_str(b);
            a.push_str(b);
            header.push(a);
        }
    }
    header
}

pub fn vec_row(i: isize, sum: f64, mut vec_row: Vec<&str>) -> Vec<&str> {
    let mut vec: Vec<&str> = Vec::new();

    let q = format!("{:?}", i);
    let s = format!("{:.1}", sum);
    let q: &'static str = string_to_static_str(q);
    let s: &'static str = string_to_static_str(s);

    vec.push(q);
    vec.append(&mut vec_row);
    vec.push(s);
    vec
}

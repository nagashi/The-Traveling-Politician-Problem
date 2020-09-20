// Return the factoral number
// used isize to match the ability
// of the underlying architecture 
pub fn factorial(num: isize) -> isize {
    match num {
        0 | 1 => 1,
        _ => factorial(num - 1) * num,
    }
}

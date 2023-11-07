

pub const fn const_eq_str(a: &str, b: &str) -> bool {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();

    if a_bytes.len() != b_bytes.len() {
        return false;
    }

    let mut i: usize = 0;
    while i < a_bytes.len() {
        if a_bytes[i] != b_bytes[i] {
            return false;
        }
        i += 1;
    }

    true
}

pub const fn const_str_position(v : &str, list : &[&str]) -> Option<usize> {
    let mut i : usize = 0;

    while i < list.len() {
        if const_eq_str(v, list[i]) {
            return Some(i);
        }
        i += 1;
    }
    None
}

pub const fn const_str_position_in_tuples<T>(v : &str, list : &[(&str, T)]) -> Option<usize> {
    let mut i : usize = 0;

    while i < list.len() {
        if const_eq_str(v, list[i].0) {
            return Some(i);
        }
        i += 1;
    }
    None
}

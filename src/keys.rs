pub fn is_exit(key: i32) -> bool {
    match key {
        27 | 3 | 4 => true,
        _ => false,
    }
}

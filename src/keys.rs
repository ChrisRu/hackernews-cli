pub fn is_exit(key: i32) -> bool {
    match key {
        // 113: q
        // 3: ctrl c
        // 4: ctrl d
        113 | 3 | 4 => true,
        _ => false,
    }
}

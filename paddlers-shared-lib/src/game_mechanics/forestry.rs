use chrono::Duration;

pub fn tree_size(age: Duration) -> usize {
    match age.num_hours() {
        h if h < 1 => 1,
        h if h < 4 => 2,
        h if h <= 45 => 3 + h as usize / 9,
        h if h < 72 => 9,
        _ => 10,
    }
}
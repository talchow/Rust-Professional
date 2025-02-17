
pub fn new_count_distinct(input_str: &str) -> usize {
    let mut hashset_input = HashSet::new();
    for i in input_str.split(",") {
        hashset_input.insert(i);
    }
    hashset_input.len()
}

pub fn read_input(filename: &str) -> String {
    std::fs::read_to_string(filename).expect("Failed to read input file")
}

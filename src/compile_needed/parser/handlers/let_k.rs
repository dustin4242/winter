pub fn handler(pos: usize, final_file: &mut Vec<String>) -> usize {
    final_file.push(format!(";let mut"));
    pos
}

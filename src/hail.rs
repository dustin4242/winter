pub fn types() -> Vec<String> {
    vec!["string", "i8", "i16", "i32"]
        .into_iter()
        .map(|x| x.to_string())
        .collect()
}

pub fn keywords() -> Vec<String> {
    vec!["let", "use", "exportasm"]
        .into_iter()
        .map(|x| x.to_string())
        .collect()
}

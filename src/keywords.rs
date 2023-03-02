pub fn keywords() -> Vec<String> {
    vec!["let", "use", "export"]
        .into_iter()
        .map(|x| x.to_string())
        .collect()
}

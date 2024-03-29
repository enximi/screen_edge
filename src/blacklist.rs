pub fn in_blacklist(window_handle: isize) -> bool {
    let window_title_blacklist = vec!["PUBG：绝地求生 "];
    let window_title = window_inspector::get_window_title(window_handle).unwrap_or("".to_string());
    window_title_blacklist.contains(&window_title.as_str())
}

use anyhow::Result;

pub fn in_blacklist(window_handle: isize) -> Result<bool> {
    let window_title_blacklist = vec!["PUBG：绝地求生 "];
    let window_title = window_inspector::get_window_title(window_handle)?;
    Ok(window_title_blacklist.contains(&window_title.as_str()))
}

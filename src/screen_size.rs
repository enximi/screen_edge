use anyhow::Result;
use enigo::Mouse;
use lazy_static::lazy_static;

lazy_static! {
    static ref SCREEN_SIZE: (i32, i32) = get_screen_size_().unwrap();
}

pub fn get_screen_size() -> (i32, i32) {
    *SCREEN_SIZE
}

fn get_screen_size_() -> Result<(i32, i32)> {
    let enigo = enigo::Enigo::new(&Default::default())?;
    enigo.main_display().map_err(Into::into)
}

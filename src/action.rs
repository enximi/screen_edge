use anyhow::Result;
use enigo::Keyboard;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    NextDesktop,
    PreviousDesktop,
    MultiTaskView,
    VolumeUp,
    VolumeDown,
    VolumeMute,
    CtrlTab,
    CtrlShiftTab,
    AltTab,
    AltShiftTab,
}

pub fn key_sequence(keys: &[enigo::Key]) -> Result<()> {
    let mut enigo = enigo::Enigo::new(&Default::default())?;
    for key in keys {
        enigo.key(*key, enigo::Direction::Press)?;
    }
    for key in keys.iter().rev() {
        enigo.key(*key, enigo::Direction::Release)?;
    }
    Ok(())
}

pub fn next_desktop() -> Result<()> {
    key_sequence(&vec![
        enigo::Key::Control,
        enigo::Key::LWin,
        enigo::Key::RightArrow,
    ])?;
    Ok(())
}

pub fn previous_desktop() -> Result<()> {
    key_sequence(&vec![
        enigo::Key::Control,
        enigo::Key::LWin,
        enigo::Key::LeftArrow,
    ])?;
    Ok(())
}

pub fn multi_task_view() -> Result<()> {
    key_sequence(&vec![enigo::Key::LWin, enigo::Key::Tab])?;
    Ok(())
}

pub fn volume_up() -> Result<()> {
    key_sequence(&vec![enigo::Key::VolumeUp])?;
    Ok(())
}

pub fn volume_down() -> Result<()> {
    key_sequence(&vec![enigo::Key::VolumeDown])?;
    Ok(())
}

pub fn volume_mute() -> Result<()> {
    key_sequence(&vec![enigo::Key::VolumeMute])?;
    Ok(())
}

pub fn ctrl_tab() -> Result<()> {
    key_sequence(&vec![enigo::Key::Control, enigo::Key::Tab])?;
    Ok(())
}

pub fn ctrl_shift_tab() -> Result<()> {
    key_sequence(&vec![
        enigo::Key::Control,
        enigo::Key::Shift,
        enigo::Key::Tab,
    ])?;
    Ok(())
}

pub fn alt_tab() -> Result<()> {
    key_sequence(&vec![enigo::Key::Alt, enigo::Key::Tab])?;
    Ok(())
}

pub fn alt_shift_tab() -> Result<()> {
    key_sequence(&vec![enigo::Key::Alt, enigo::Key::Shift, enigo::Key::Tab])?;
    Ok(())
}

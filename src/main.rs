use std::collections::HashMap;

use anyhow::{anyhow, Result};
use enigo::{Keyboard, Mouse};
use lazy_static::lazy_static;
use rdev::{Button, EventType};

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    for (position, operation_action) in &*CONFIG {
        for (operation, action) in operation_action {
            println!("{:?} {:?} {:?}", position, operation, action);
        }
    }
    screen_edge_start()?;
    Ok(())
}

fn screen_edge_start() -> Result<()> {
    let mut last_position: Option<ScreenPosition> = None;
    rdev::listen(move |event| match event.event_type {
        EventType::ButtonPress(button) => match button {
            Button::Middle => {
                let f = move || -> Result<()> {
                    let enigo = enigo::Enigo::new(&Default::default())?;
                    let (mouse_x, mouse_y) = enigo.location()?;
                    let screen_position = get_screen_position(mouse_x, mouse_y);
                    log::debug!("middle press at {:?}", screen_position);
                    execute(screen_position, UserOperation::MiddlePress)?;
                    Ok(())
                };
                let _ = f();
            }
            _ => {}
        },
        EventType::MouseMove { x, y } => {
            let now_position = get_screen_position(x as i32, y as i32);
            if !last_position.is_some_and(|last_position| last_position == now_position) {
                if last_position.is_none() || can_touch(last_position.unwrap(), now_position) {
                    log::debug!("touch {:?}", now_position);
                    let _ = execute(now_position, UserOperation::Touch);
                }
                last_position = Some(now_position);
            }
        }
        EventType::Wheel {
            delta_x: _,
            delta_y,
        } => {
            let f = move || -> Result<()> {
                let user_operation = if delta_y == 0 {
                    return Ok(());
                } else if delta_y > 0 {
                    UserOperation::ScrollUp
                } else {
                    UserOperation::ScrollDown
                };
                let enigo = enigo::Enigo::new(&Default::default())?;
                let (mouse_x, mouse_y) = enigo.location()?;
                let screen_position = get_screen_position(mouse_x, mouse_y);
                log::debug!("{:?} at {:?}", user_operation, screen_position);
                execute(screen_position, user_operation)?;
                Ok(())
            };
            let _ = f();
        }
        _ => {}
    })
    .map_err(|e| anyhow!("Error while listening to events: {:?}", e))?;
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ScreenPosition {
    LeftTop,
    Top1,
    Top2,
    Top3,
    RightTop,
    Left1,
    Left2,
    Left3,
    Center,
    Right1,
    Right2,
    Right3,
    LeftBottom,
    Bottom1,
    Bottom2,
    Bottom3,
    RightBottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum UserOperation {
    ScrollUp,
    ScrollDown,
    MiddlePress,
    MiddleRelease,
    Touch,
}

lazy_static! {
    static ref SCREEN_SIZE: (i32, i32) = get_screen_size().unwrap();
}

fn get_screen_size() -> Result<(i32, i32)> {
    let enigo = enigo::Enigo::new(&Default::default())?;
    let screen_size = enigo.main_display()?;
    Ok(screen_size)
}

fn get_screen_position(x: i32, y: i32) -> ScreenPosition {
    let (screen_width, screen_height) = *SCREEN_SIZE;
    let split1 = 0.2;
    let split2 = 0.8;
    if x <= 0 {
        if y <= 0 {
            ScreenPosition::LeftTop
        } else if y >= screen_height - 1 {
            ScreenPosition::LeftBottom
        } else {
            if y < (screen_height as f32 * split1).round() as i32 {
                ScreenPosition::Left1
            } else if y <= (screen_height as f32 * split2).round() as i32 {
                ScreenPosition::Left2
            } else {
                ScreenPosition::Left3
            }
        }
    } else if x >= screen_width - 1 {
        if y <= 0 {
            ScreenPosition::RightTop
        } else if y >= screen_height - 1 {
            ScreenPosition::RightBottom
        } else {
            if y < (screen_height as f32 * split1).round() as i32 {
                ScreenPosition::Right1
            } else if y <= (screen_height as f32 * split2).round() as i32 {
                ScreenPosition::Right2
            } else {
                ScreenPosition::Right3
            }
        }
    } else {
        if y <= 0 {
            if x < (screen_width as f32 * split1).round() as i32 {
                ScreenPosition::Top1
            } else if x <= (screen_width as f32 * split2).round() as i32 {
                ScreenPosition::Top2
            } else {
                ScreenPosition::Top3
            }
        } else if y >= screen_height - 1 {
            if x < (screen_width as f32 * split1).round() as i32 {
                ScreenPosition::Bottom1
            } else if x <= (screen_width as f32 * split2).round() as i32 {
                ScreenPosition::Bottom2
            } else {
                ScreenPosition::Bottom3
            }
        } else {
            ScreenPosition::Center
        }
    }
}

fn key_sequence(keys: &[enigo::Key]) -> Result<()> {
    let mut enigo = enigo::Enigo::new(&Default::default())?;
    for key in keys {
        enigo.key(*key, enigo::Direction::Press)?;
    }
    for key in keys.iter().rev() {
        enigo.key(*key, enigo::Direction::Release)?;
    }
    Ok(())
}

fn next_desktop() -> Result<()> {
    key_sequence(&vec![
        enigo::Key::Control,
        enigo::Key::LWin,
        enigo::Key::RightArrow,
    ])?;
    Ok(())
}

fn previous_desktop() -> Result<()> {
    key_sequence(&vec![
        enigo::Key::Control,
        enigo::Key::LWin,
        enigo::Key::LeftArrow,
    ])?;
    Ok(())
}

fn multi_task_view() -> Result<()> {
    key_sequence(&vec![enigo::Key::LWin, enigo::Key::Tab])?;
    Ok(())
}

fn volume_up() -> Result<()> {
    key_sequence(&vec![enigo::Key::VolumeUp])?;
    Ok(())
}

fn volume_down() -> Result<()> {
    key_sequence(&vec![enigo::Key::VolumeDown])?;
    Ok(())
}

fn volume_mute() -> Result<()> {
    key_sequence(&vec![enigo::Key::VolumeMute])?;
    Ok(())
}

fn ctrl_tab() -> Result<()> {
    key_sequence(&vec![enigo::Key::Control, enigo::Key::Tab])?;
    Ok(())
}

fn ctrl_shift_tab() -> Result<()> {
    key_sequence(&vec![
        enigo::Key::Control,
        enigo::Key::Shift,
        enigo::Key::Tab,
    ])?;
    Ok(())
}

fn alt_tab() -> Result<()> {
    key_sequence(&vec![enigo::Key::Alt, enigo::Key::Tab])?;
    Ok(())
}

fn alt_shift_tab() -> Result<()> {
    key_sequence(&vec![enigo::Key::Alt, enigo::Key::Shift, enigo::Key::Tab])?;
    Ok(())
}

fn execute(screen_position: ScreenPosition, user_operation: UserOperation) -> Result<()> {
    let action = get_action(screen_position, user_operation);
    if let Some(action) = action {
        match action {
            Action::NextDesktop => {
                log::info!("next desktop");
                next_desktop()?;
            }
            Action::PreviousDesktop => {
                log::info!("previous desktop");
                previous_desktop()?;
            }
            Action::MultiTaskView => {
                log::info!("multi task view");
                multi_task_view()?;
            }
            Action::VolumeUp => {
                log::info!("volume up");
                volume_up()?;
            }
            Action::VolumeDown => {
                log::info!("volume down");
                volume_down()?;
            }
            Action::VolumeMute => {
                log::info!("volume mute");
                volume_mute()?;
            }
            Action::CtrlTab => {
                log::info!("ctrl tab");
                ctrl_tab()?;
            }
            Action::CtrlShiftTab => {
                log::info!("ctrl shift tab");
                ctrl_shift_tab()?;
            }
            Action::AltTab => {
                log::info!("alt tab");
                alt_tab()?;
            }
            Action::AltShiftTab => {
                log::info!("alt shift tab");
                alt_shift_tab()?;
            }
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Action {
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

fn config() -> HashMap<ScreenPosition, HashMap<UserOperation, Action>> {
    let mut config = HashMap::new();
    config.insert(
        ScreenPosition::LeftTop,
        HashMap::from([(UserOperation::Touch, Action::MultiTaskView)]),
    );
    config.insert(
        ScreenPosition::Top2,
        HashMap::from([
            (UserOperation::ScrollDown, Action::CtrlTab),
            (UserOperation::ScrollUp, Action::CtrlShiftTab),
        ]),
    );
    config.insert(
        ScreenPosition::RightTop,
        HashMap::from([
            (UserOperation::ScrollUp, Action::VolumeUp),
            (UserOperation::ScrollDown, Action::VolumeDown),
            (UserOperation::MiddlePress, Action::VolumeMute),
        ]),
    );
    config.insert(
        ScreenPosition::Left2,
        HashMap::from([
            (UserOperation::ScrollUp, Action::PreviousDesktop),
            (UserOperation::ScrollDown, Action::NextDesktop),
            (UserOperation::MiddlePress, Action::MultiTaskView),
        ]),
    );
    config.insert(
        ScreenPosition::Bottom2,
        HashMap::from([
            (UserOperation::ScrollUp, Action::AltTab),
            (UserOperation::ScrollDown, Action::AltShiftTab),
        ]),
    );
    config
}

lazy_static! {
    static ref CONFIG: HashMap<ScreenPosition, HashMap<UserOperation, Action>> = config();
}

fn get_action(screen_position: ScreenPosition, user_operation: UserOperation) -> Option<Action> {
    CONFIG.get(&screen_position)?.get(&user_operation).copied()
}

fn touch_config() -> HashMap<ScreenPosition, Vec<ScreenPosition>> {
    // 角落
    let corner = vec![
        ScreenPosition::LeftTop,
        ScreenPosition::RightTop,
        ScreenPosition::LeftBottom,
        ScreenPosition::RightBottom,
    ];
    // 临近角落的边缘
    let edge_near_corner = vec![
        ScreenPosition::Top1,
        ScreenPosition::Top3,
        ScreenPosition::Bottom1,
        ScreenPosition::Bottom3,
        ScreenPosition::Left1,
        ScreenPosition::Left3,
        ScreenPosition::Right1,
        ScreenPosition::Right3,
    ];
    // 不临近角落的边缘
    let edge_not_near_corner = vec![
        ScreenPosition::Top2,
        ScreenPosition::Bottom2,
        ScreenPosition::Left2,
        ScreenPosition::Right2,
    ];
    // 从这些位置不能触发touch
    let mut config = HashMap::new();
    for position in &corner {
        config.insert(*position, vec![]);
    }
    // corner + edge_not_near_corner
    let corner_edge_not_near_corner = corner
        .iter()
        .chain(edge_not_near_corner.iter())
        .copied()
        .collect::<Vec<_>>();
    for position in &edge_near_corner {
        config.insert(*position, corner_edge_not_near_corner.clone());
    }
    for position in &edge_not_near_corner {
        config.insert(*position, edge_near_corner.clone());
    }
    config
}

lazy_static! {
    static ref TOUCH_CONFIG: HashMap<ScreenPosition, Vec<ScreenPosition>> = touch_config();
}

fn can_touch(from: ScreenPosition, to: ScreenPosition) -> bool {
    TOUCH_CONFIG.get(&from).map_or(false, |v| v.contains(&to))
}

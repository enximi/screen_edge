use anyhow::{anyhow, Result};
use enigo::Mouse;
use rdev::{Button, EventType};

use crate::action::{
    alt_shift_tab, alt_tab, ctrl_shift_tab, ctrl_tab, multi_task_view, next_desktop,
    previous_desktop, volume_down, volume_mute, volume_up, Action,
};
use crate::blacklist::in_blacklist;
use crate::config::{can_touch, get_action};
use crate::screen_position::{get_screen_position, ScreenPosition};
use crate::user_operation::UserOperation;

pub mod screen_position;

pub mod user_operation;

pub mod screen_size;

pub mod action;

pub mod blacklist;
pub mod config;

pub fn start() -> tokio::task::JoinHandle<Result<()>> {
    let handle = tokio::spawn(async { run() });
    handle
}

fn run() -> Result<()> {
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

fn execute(screen_position: ScreenPosition, user_operation: UserOperation) -> Result<()> {
    if let Some(action) = get_action(screen_position, user_operation) {
        if !in_blacklist(window_inspector::get_foreground_window_handle()) {
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
    }

    Ok(())
}

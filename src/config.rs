use crate::action::Action;
use crate::screen_position::ScreenPosition;
use crate::user_operation::UserOperation;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref CONFIG: HashMap<ScreenPosition, HashMap<UserOperation, Action>> = config();
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

pub fn get_action(
    screen_position: ScreenPosition,
    user_operation: UserOperation,
) -> Option<Action> {
    CONFIG.get(&screen_position)?.get(&user_operation).copied()
}

lazy_static! {
    static ref TOUCH_CONFIG: HashMap<ScreenPosition, Vec<ScreenPosition>> = touch_config();
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

pub fn can_touch(from: ScreenPosition, to: ScreenPosition) -> bool {
    TOUCH_CONFIG.get(&from).map_or(false, |v| v.contains(&to))
}

use crate::screen_size::get_screen_size;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScreenPosition {
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

pub fn get_screen_position(x: i32, y: i32) -> ScreenPosition {
    let (screen_width, screen_height) = get_screen_size();
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

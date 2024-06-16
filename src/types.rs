use bevy::prelude::*;

/// Utility struct for constructing the `offset: Vec4` property of the material and `padding: UiRect` for the node.
///
/// # Example:
///
/// ```
/// use bevy::prelude::{UiRect, Val, Vec4};
/// use bevy_round_ui::prelude::RoundUiOffset;
///
/// let rect_offset = RoundUiOffset {
///     top: 1.0,
///     left: 2.0,
///     bottom: 3.0,
///     right: 4.0,
/// };
///
/// let offset: Vec4 = rect_offset.into();
/// assert_eq!(offset.x, rect_offset.top);
/// assert_eq!(offset.y, rect_offset.left);
/// assert_eq!(offset.z, rect_offset.bottom);
/// assert_eq!(offset.w, rect_offset.right);
///
/// let ui_rect: UiRect = rect_offset.into();
/// assert_eq!(ui_rect.top, Val::Px(rect_offset.top));
/// assert_eq!(ui_rect.left, Val::Px(rect_offset.left));
/// assert_eq!(ui_rect.bottom, Val::Px(rect_offset.bottom));
/// assert_eq!(ui_rect.right, Val::Px(rect_offset.right));
/// ```
#[derive(Copy, Clone, Debug, Default, Reflect, PartialEq)]
pub struct RoundUiOffset {
    pub top: f32,
    pub left: f32,
    pub bottom: f32,
    pub right: f32,
}

impl RoundUiOffset {
    pub fn all(amount: f32) -> Self {
        Self {
            top: amount,
            left: amount,
            bottom: amount,
            right: amount,
        }
    }

    pub fn left(amount: f32) -> Self {
        Self {
            top: 0.,
            left: amount,
            bottom: 0.,
            right: 0.,
        }
    }

    pub fn right(amount: f32) -> Self {
        Self {
            top: 0.,
            left: 0.,
            bottom: 0.,
            right: amount,
        }
    }

    pub fn bottom(amount: f32) -> Self {
        Self {
            top: 0.,
            left: 0.,
            bottom: amount,
            right: 0.,
        }
    }

    pub fn top(amount: f32) -> Self {
        Self {
            top: amount,
            left: 0.,
            bottom: 0.,
            right: 0.,
        }
    }

    pub fn top_left(amount: f32) -> Self {
        Self {
            top: amount,
            left: amount,
            bottom: 0.,
            right: 0.,
        }
    }

    pub fn top_right(amount: f32) -> Self {
        Self {
            top: amount,
            left: 0.,
            bottom: 0.,
            right: amount,
        }
    }

    pub fn bottom_left(amount: f32) -> Self {
        Self {
            top: 0.,
            left: amount,
            bottom: amount,
            right: 0.,
        }
    }

    pub fn bottom_right(amount: f32) -> Self {
        Self {
            top: 0.,
            left: 0.,
            bottom: amount,
            right: amount,
        }
    }
}

impl From<RoundUiOffset> for Vec4 {
    fn from(val: RoundUiOffset) -> Self {
        Vec4::new(val.top, val.left, val.bottom, val.right)
    }
}

impl From<Vec4> for RoundUiOffset {
    fn from(val: Vec4) -> Self {
        Self {
            top: val.x,
            left: val.y,
            bottom: val.z,
            right: val.w,
        }
    }
}

impl From<RoundUiOffset> for UiRect {
    fn from(val: RoundUiOffset) -> Self {
        UiRect {
            left: Val::Px(val.left),
            right: Val::Px(val.right),
            top: Val::Px(val.top),
            bottom: Val::Px(val.bottom),
        }
    }
}

/// Utility struct for constructing the `border: Vec4` property of the material.
///
/// # Example:
///
/// ```
/// use bevy::prelude::Vec4;
/// use bevy_round_ui::prelude::RoundUiBorder;
///
/// let border = RoundUiBorder {
///     top_left: 1.0,
///     top_right: 2.0,
///     bottom_left: 3.0,
///     bottom_right: 4.0,
/// };
///
/// let border_vec: Vec4 = border.into();
/// assert_eq!(border_vec.x, border.bottom_right);
/// assert_eq!(border_vec.y, border.top_right);
/// assert_eq!(border_vec.z, border.bottom_left);
/// assert_eq!(border_vec.w, border.top_left);
/// ```
#[derive(Copy, Clone, Debug, Default, Reflect, PartialEq)]
pub struct RoundUiBorder {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_left: f32,
    pub bottom_right: f32,
}

impl RoundUiBorder {
    pub fn all(radius: f32) -> Self {
        Self {
            top_left: radius,
            top_right: radius,
            bottom_left: radius,
            bottom_right: radius,
        }
    }

    pub fn top_left(radius: f32) -> Self {
        Self {
            top_left: radius,
            top_right: 0.,
            bottom_left: 0.,
            bottom_right: 0.,
        }
    }

    pub fn top_right(radius: f32) -> Self {
        Self {
            top_left: 0.,
            top_right: radius,
            bottom_left: 0.,
            bottom_right: 0.,
        }
    }

    pub fn bottom_left(radius: f32) -> Self {
        Self {
            top_left: 0.,
            top_right: 0.,
            bottom_left: radius,
            bottom_right: 0.,
        }
    }

    pub fn bottom_right(radius: f32) -> Self {
        Self {
            top_left: 0.,
            top_right: 0.,
            bottom_left: 0.,
            bottom_right: radius,
        }
    }

    pub fn top(radius: f32) -> Self {
        Self {
            top_left: radius,
            top_right: radius,
            bottom_left: 0.,
            bottom_right: 0.,
        }
    }

    pub fn bottom(radius: f32) -> Self {
        Self {
            top_left: 0.,
            top_right: 0.,
            bottom_left: radius,
            bottom_right: radius,
        }
    }

    pub fn left(radius: f32) -> Self {
        Self {
            top_left: radius,
            top_right: 0.,
            bottom_left: radius,
            bottom_right: 0.,
        }
    }

    pub fn right(radius: f32) -> Self {
        Self {
            top_left: 0.,
            top_right: radius,
            bottom_left: 0.,
            bottom_right: radius,
        }
    }
}

impl From<RoundUiBorder> for Vec4 {
    fn from(val: RoundUiBorder) -> Self {
        Vec4::new(
            val.bottom_right,
            val.top_right,
            val.bottom_left,
            val.top_left,
        )
    }
}

impl From<Vec4> for RoundUiBorder {
    fn from(val: Vec4) -> Self {
        Self {
            bottom_right: val.x,
            top_right: val.y,
            bottom_left: val.z,
            top_left: val.w,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn round_ui_border_equality() {
        // TODO: Possibly remove? Not sure if this is worth testing?
        let border = RoundUiBorder {
            top_left: 1.,
            top_right: 2.,
            bottom_left: 3.,
            bottom_right: 4.,
        };
        assert_eq!(border, border.clone());
        assert_ne!(border, RoundUiBorder::all(1.));
    }

    #[test]
    fn round_ui_border_to_and_from_vec4() {
        let border = RoundUiBorder {
            top_left: 1.,
            top_right: 2.,
            bottom_left: 3.,
            bottom_right: 4.,
        };
        let border_vec4: Vec4 = border.into();
        let border_result: RoundUiBorder = border_vec4.into();
        assert_eq!(border_result, border);
    }

    #[test]
    fn round_ui_offset_equality() {
        // TODO: Possibly remove? Not sure if this is worth testing?
        let offset = RoundUiOffset {
            top: 1.,
            left: 2.,
            bottom: 3.,
            right: 4.,
        };
        assert_eq!(offset, offset.clone());
        assert_ne!(offset, RoundUiOffset::all(1.));
    }

    #[test]
    fn round_ui_offset_to_and_from_vec4() {
        let offset = RoundUiOffset {
            top: 1.,
            left: 2.,
            bottom: 3.,
            right: 4.,
        };
        let offset_vec4: Vec4 = offset.into();
        let offset_result: RoundUiOffset = offset_vec4.into();
        assert_eq!(offset_result, offset);
    }
}

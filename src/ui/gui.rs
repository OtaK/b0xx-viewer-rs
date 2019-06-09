use super::{app::ViewerApp, Ids};
use crate::ui::support::{BTN_RADIUS, WIN_H, WIN_W};

/// A set of reasonable stylistic defaults that works for the `gui` below.
pub fn theme() -> conrod_core::Theme {
    use conrod_core::position::{Align, Direction, Padding, Position, Relative};
    conrod_core::Theme {
        name: "B0XX theme".to_string(),
        padding: Padding::none(),
        x_position: Position::Relative(Relative::Align(Align::Start), None),
        y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
        background_color: conrod_core::color::DARK_CHARCOAL,
        shape_color: conrod_core::color::LIGHT_CHARCOAL,
        border_color: conrod_core::color::BLACK,
        border_width: 0.0,
        label_color: conrod_core::color::WHITE,
        font_id: None,
        font_size_large: 26,
        font_size_medium: 18,
        font_size_small: 12,
        widget_styling: conrod_core::theme::StyleMap::default(),
        mouse_drag_threshold: 0.0,
        double_click_threshold: std::time::Duration::from_millis(500),
    }
}

pub fn render_gui(ui: &mut conrod_core::UiCell, ids: &Ids, app: &mut ViewerApp) {
    use conrod_core::{color, widget, Colorable, Positionable, Sizeable, Widget};

    widget::Canvas::new()
        .color(color::rgb_bytes(19, 19, 19))
        .w_h(WIN_W.into(), WIN_H.into())
        .x_y(0., 0.)
        .crop_kids()
        .set(ids.frame, ui);

    make_button(app.state.start, ids.frame)
        .x_y(0., 40.)
        .set(ids.start_btn, ui);

    make_button(app.state.right, ids.frame)
        .x_y_relative_to(ids.start_btn, -100., 5.)
        .set(ids.right_btn, ui);

    make_button(app.state.down, ids.frame)
        .x_y_relative_to(ids.right_btn, -45., 15.)
        .set(ids.down_btn, ui);

    make_button(app.state.left, ids.frame)
        .x_y_relative_to(ids.down_btn, -45., -5.)
        .set(ids.left_btn, ui);

    make_button(app.state.l, ids.frame)
        .x_y_relative_to(ids.left_btn, -45., -15.)
        .set(ids.l_btn, ui);

    make_button(app.state.mod_x, ids.frame)
        .x_y_relative_to(ids.right_btn, 10., -120.)
        .set(ids.mod_x_btn, ui);

    make_button(app.state.mod_y, ids.frame)
        .x_y_relative_to(ids.mod_x_btn, 40., -20.)
        .set(ids.mod_y_btn, ui);

    make_button(app.state.b, ids.frame)
        .x_y_relative_to(ids.start_btn, 100., 5.)
        .set(ids.b_btn, ui);

    make_button(app.state.x, ids.frame)
        .x_y_relative_to(ids.b_btn, 45., 15.)
        .set(ids.x_btn, ui);

    make_button(app.state.z, ids.frame)
        .x_y_relative_to(ids.x_btn, 45., -5.)
        .set(ids.z_btn, ui);

    make_button(app.state.up, ids.frame)
        .x_y_relative_to(ids.z_btn, 45., -15.)
        .set(ids.up_btn, ui);

    make_button(app.state.y, ids.frame)
        .x_y_relative_to(ids.x_btn, 2., 45.)
        .set(ids.y_btn, ui);

    make_button(app.state.r, ids.frame)
        .x_y_relative_to(ids.b_btn, 2., 45.)
        .set(ids.r_btn, ui);

    make_button(app.state.a, ids.frame)
        .x_y_relative_to(ids.b_btn, -10., -120.)
        .set(ids.a_btn, ui);

    make_button(app.state.c_up, ids.frame)
        .x_y_relative_to(ids.a_btn, 1., 48.)
        .set(ids.c_up_btn, ui);

    make_button(app.state.c_left, ids.frame)
        .x_y_relative_to(ids.c_up_btn, -34., -24.)
        .set(ids.c_left_btn, ui);

    make_button(app.state.c_right, ids.frame)
        .x_y_relative_to(ids.c_up_btn, 34., -24.)
        .set(ids.c_right_btn, ui);

    make_button(app.state.c_down, ids.frame)
        .x_y_relative_to(ids.c_left_btn, 0., -48.)
        .set(ids.c_down_btn, ui);
}

#[inline(always)]
fn make_button(
    state: bool,
    parent: conrod_core::widget::Id,
) -> conrod_core::widget::Oval<conrod_core::widget::primitive::shape::oval::Full> {
    use conrod_core::{color, widget, Colorable, Sizeable, Widget};

    let color = if state {
        color::rgb_bytes(0, 235, 255)
    } else {
        color::DARK_CHARCOAL
    };

    widget::Circle::fill(BTN_RADIUS)
        .color(color)
        .parent(parent)
        .graphics_for(parent)
        .w_h(BTN_RADIUS, BTN_RADIUS)
}

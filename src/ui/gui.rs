use super::{app::ViewerApp, Ids};
use crate::cli::ViewerOptions;
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

pub fn render_gui(
    ui: &mut conrod_core::UiCell,
    ids: &Ids,
    app: &mut ViewerApp,
    options: ViewerOptions,
) {
    use conrod_core::{widget, Colorable, Positionable, Sizeable, Widget};

    widget::Canvas::new()
        .color(options.background_color)
        .w_h(WIN_W.into(), WIN_H.into())
        .x_y(0., 0.)
        .crop_kids()
        .set(ids.frame, ui);

    make_button(
        app.state.start,
        ids.frame,
        options.button_active_colors.start,
        options.button_inactive_colors.start,
    )
    .x_y(0., 40.)
    .set(ids.start_btn, ui);

    make_button(
        app.state.right,
        ids.frame,
        options.button_active_colors.right,
        options.button_inactive_colors.right,
    )
    .x_y_relative_to(ids.start_btn, -100., 5.)
    .set(ids.right_btn, ui);

    make_button(
        app.state.down,
        ids.frame,
        options.button_active_colors.down,
        options.button_inactive_colors.down,
    )
    .x_y_relative_to(ids.right_btn, -45., 15.)
    .set(ids.down_btn, ui);

    make_button(
        app.state.left,
        ids.frame,
        options.button_active_colors.left,
        options.button_inactive_colors.left,
    )
    .x_y_relative_to(ids.down_btn, -45., -5.)
    .set(ids.left_btn, ui);

    make_button(
        app.state.l,
        ids.frame,
        options.button_active_colors.l,
        options.button_inactive_colors.l,
    )
    .x_y_relative_to(ids.left_btn, -45., -15.)
    .set(ids.l_btn, ui);

    make_button(
        app.state.mod_x,
        ids.frame,
        options.button_active_colors.mod_x,
        options.button_inactive_colors.mod_x,
    )
    .x_y_relative_to(ids.right_btn, 10., -120.)
    .set(ids.mod_x_btn, ui);

    make_button(
        app.state.mod_y,
        ids.frame,
        options.button_active_colors.mod_y,
        options.button_inactive_colors.mod_y,
    )
    .x_y_relative_to(ids.mod_x_btn, 40., -20.)
    .set(ids.mod_y_btn, ui);

    make_button(
        app.state.b,
        ids.frame,
        options.button_active_colors.b,
        options.button_inactive_colors.b,
    )
    .x_y_relative_to(ids.start_btn, 100., 5.)
    .set(ids.b_btn, ui);

    make_button(
        app.state.x,
        ids.frame,
        options.button_active_colors.x,
        options.button_inactive_colors.x,
    )
    .x_y_relative_to(ids.b_btn, 45., 15.)
    .set(ids.x_btn, ui);

    make_button(
        app.state.z,
        ids.frame,
        options.button_active_colors.z,
        options.button_inactive_colors.z,
    )
    .x_y_relative_to(ids.x_btn, 45., -5.)
    .set(ids.z_btn, ui);

    make_button(
        app.state.up,
        ids.frame,
        options.button_active_colors.up,
        options.button_inactive_colors.up,
    )
    .x_y_relative_to(ids.z_btn, 45., -15.)
    .set(ids.up_btn, ui);

    make_button(
        app.state.y,
        ids.frame,
        options.button_active_colors.y,
        options.button_inactive_colors.y,
    )
    .x_y_relative_to(ids.x_btn, 2., 45.)
    .set(ids.y_btn, ui);

    make_button(
        app.state.r,
        ids.frame,
        options.button_active_colors.r,
        options.button_inactive_colors.r,
    )
    .x_y_relative_to(ids.b_btn, 2., 45.)
    .set(ids.r_btn, ui);

    make_button(
        app.state.a,
        ids.frame,
        options.button_active_colors.a,
        options.button_inactive_colors.a,
    )
    .x_y_relative_to(ids.b_btn, -10., -120.)
    .set(ids.a_btn, ui);

    make_button(
        app.state.c_up,
        ids.frame,
        options.button_active_colors.c_up,
        options.button_inactive_colors.c_up,
    )
    .x_y_relative_to(ids.a_btn, 1., 48.)
    .set(ids.c_up_btn, ui);

    make_button(
        app.state.c_left,
        ids.frame,
        options.button_active_colors.c_left,
        options.button_inactive_colors.c_left,
    )
    .x_y_relative_to(ids.c_up_btn, -34., -24.)
    .set(ids.c_left_btn, ui);

    make_button(
        app.state.c_right,
        ids.frame,
        options.button_active_colors.c_right,
        options.button_inactive_colors.c_right,
    )
    .x_y_relative_to(ids.c_up_btn, 34., -24.)
    .set(ids.c_right_btn, ui);

    make_button(
        app.state.c_down,
        ids.frame,
        options.button_active_colors.c_down,
        options.button_inactive_colors.c_down,
    )
    .x_y_relative_to(ids.c_left_btn, 0., -48.)
    .set(ids.c_down_btn, ui);
}

#[inline(always)]
fn make_button(
    state: bool,
    parent: conrod_core::widget::Id,
    active_color: conrod_core::Color,
    inactive_color: conrod_core::Color,
) -> conrod_core::widget::Oval<conrod_core::widget::primitive::shape::oval::Full> {
    use conrod_core::{widget, Colorable, Sizeable, Widget};

    widget::Circle::fill(BTN_RADIUS)
        .color(if state { active_color } else { inactive_color })
        .parent(parent)
        .graphics_for(parent)
        .w_h(BTN_RADIUS, BTN_RADIUS)
}

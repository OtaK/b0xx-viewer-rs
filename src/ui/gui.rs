use super::{app::*, Ids};
use crate::config::ViewerOptions;
use crate::ui::support::{BTN_RADIUS, WIN_H, WIN_W};

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
    options: &ViewerOptions,
) {
    use conrod_core::{widget, Colorable, Positionable, Sizeable, Widget};

    // Compute button margin only if necessary
    let btn_label_margin = if options.display_labels {
        BTN_RADIUS / 2. - ui.theme().font_size_small as f64
    } else {
        0.
    };

    widget::Canvas::new()
        .color(options.background_color.into())
        .w_h(WIN_W.into(), WIN_H.into())
        .x_y(0., 0.)
        .crop_kids()
        .set(ids.frame, ui);

    if app.status == ViewerAppStatus::Reconnecting
        || app.status == ViewerAppStatus::NeedsReconnection
    {
        conrod_core::widget::Rectangle::fill_with(
            [WIN_W.into(), WIN_H.into()],
            conrod_core::color::BLACK.with_alpha(0.8),
        )
        .w_h(WIN_W.into(), WIN_H.into())
        .x_y(0., 0.)
        .crop_kids()
        .set(ids.reconnect_bg, ui);

        conrod_core::widget::Text::new("Reconnecting...")
            .color(conrod_core::color::WHITE)
            .middle_of(ids.reconnect_bg)
            .set(ids.reconnect_label, ui);
    }

    let (btn, mut m_text) = make_button(
        app.state.start,
        ids.frame,
        options.button_active_colors.start,
        options.button_inactive_colors.start,
        options.display_labels,
    );

    btn.x_y(0., 40.).set(ids.start_btn, ui);

    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("SRT")
            .color(text_color)
            .mid_top_with_margin_on(ids.start_btn, btn_label_margin)
            .set(ids.start_label, ui);
    }

    let (btn, mut m_text) = make_button(
        app.state.right,
        ids.frame,
        options.button_active_colors.right,
        options.button_inactive_colors.right,
        options.display_labels,
    );

    btn.x_y_relative_to(ids.start_btn, -105., 5.)
        .set(ids.right_btn, ui);

    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("→")
            .color(text_color)
            .mid_top_with_margin_on(ids.right_btn, btn_label_margin)
            .set(ids.right_label, ui);
    }

    let (btn, mut m_text) = make_button(
        app.state.down,
        ids.frame,
        options.button_active_colors.down,
        options.button_inactive_colors.down,
        options.display_labels,
    );

    btn.x_y_relative_to(ids.right_btn, -42., 15.)
        .set(ids.down_btn, ui);

    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("↓")
            .color(text_color)
            .mid_top_with_margin_on(ids.down_btn, btn_label_margin)
            .set(ids.down_label, ui);
    }

    let (btn, mut m_text) = make_button(
        app.state.left,
        ids.frame,
        options.button_active_colors.left,
        options.button_inactive_colors.left,
        options.display_labels,
    );

    btn.x_y_relative_to(ids.down_btn, -45., -5.)
        .set(ids.left_btn, ui);

    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("←")
            .color(text_color)
            .mid_top_with_margin_on(ids.left_btn, btn_label_margin)
            .set(ids.left_label, ui);
    }

    let (btn, mut m_text) = make_button(
        app.state.l,
        ids.frame,
        options.button_active_colors.l,
        options.button_inactive_colors.l,
        options.display_labels,
    );

    btn.x_y_relative_to(ids.left_btn, -40., -22.)
        .set(ids.l_btn, ui);

    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("L")
            .color(text_color)
            .mid_top_with_margin_on(ids.l_btn, btn_label_margin)
            .set(ids.l_label, ui);
    }

    let (btn, mut m_text) = make_button(
        app.state.mod_x,
        ids.frame,
        options.button_active_colors.mod_x,
        options.button_inactive_colors.mod_x,
        options.display_labels,
    );

    btn.x_y_relative_to(ids.right_btn, 5., -90.)
        .set(ids.mod_x_btn, ui);

    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("MX")
            .color(text_color)
            .mid_top_with_margin_on(ids.mod_x_btn, btn_label_margin)
            .set(ids.mod_x_label, ui);
    }

    let (btn, mut m_text) = make_button(
        app.state.mod_y,
        ids.frame,
        options.button_active_colors.mod_y,
        options.button_inactive_colors.mod_y,
        options.display_labels,
    );

    btn.x_y_relative_to(ids.mod_x_btn, 38., -22.)
        .set(ids.mod_y_btn, ui);

    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("MY")
            .color(text_color)
            .mid_top_with_margin_on(ids.mod_y_btn, btn_label_margin)
            .set(ids.mod_y_label, ui);
    }

    let (btn, mut m_text) = make_button(
        app.state.b,
        ids.frame,
        options.button_active_colors.b,
        options.button_inactive_colors.b,
        options.display_labels,
    );

    btn.x_y_relative_to(ids.start_btn, 100., 5.)
        .set(ids.b_btn, ui);

    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("B")
            .color(text_color)
            .mid_top_with_margin_on(ids.b_btn, btn_label_margin)
            .set(ids.b_label, ui);
    }

    let (btn, mut m_text) = make_button(
        app.state.x,
        ids.frame,
        options.button_active_colors.x,
        options.button_inactive_colors.x,
        options.display_labels,
    );

    btn.x_y_relative_to(ids.b_btn, 42., 15.).set(ids.x_btn, ui);

    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("X")
            .color(text_color)
            .mid_top_with_margin_on(ids.x_btn, btn_label_margin)
            .set(ids.x_label, ui);
    }

    let (btn, mut m_text) = make_button(
        app.state.z,
        ids.frame,
        options.button_active_colors.z,
        options.button_inactive_colors.z,
        options.display_labels,
    );

    btn.x_y_relative_to(ids.x_btn, 43., -5.).set(ids.z_btn, ui);

    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("Z")
            .color(text_color)
            .mid_top_with_margin_on(ids.z_btn, btn_label_margin)
            .set(ids.z_label, ui);
    }

    let (btn, mut m_text) = make_button(
        app.state.up,
        ids.frame,
        options.button_active_colors.up,
        options.button_inactive_colors.up,
        options.display_labels,
    );

    btn.x_y_relative_to(ids.z_btn, 42., -18.)
        .set(ids.up_btn, ui);

    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("↑")
            .color(text_color)
            .mid_top_with_margin_on(ids.up_btn, btn_label_margin)
            .set(ids.up_label, ui);
    }

    let (btn, mut m_text) = make_button(
        app.state.y,
        ids.frame,
        options.button_active_colors.y,
        options.button_inactive_colors.y,
        options.display_labels,
    );

    btn.x_y_relative_to(ids.x_btn, 0., 42.).set(ids.y_btn, ui);

    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("Y")
            .color(text_color)
            .mid_top_with_margin_on(ids.y_btn, btn_label_margin)
            .set(ids.y_label, ui);
    }

    let (btn, mut m_text) = make_button(
        app.state.r,
        ids.frame,
        options.button_active_colors.r,
        options.button_inactive_colors.r,
        options.display_labels,
    );

    btn.x_y_relative_to(ids.b_btn, 0., 42.).set(ids.r_btn, ui);

    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("R")
            .color(text_color)
            .mid_top_with_margin_on(ids.r_btn, btn_label_margin)
            .set(ids.r_label, ui);
    }

    let (btn, mut m_text) = make_button(
        app.state.a,
        ids.frame,
        options.button_active_colors.a,
        options.button_inactive_colors.a,
        options.display_labels,
    );

    btn.x_y_relative_to(ids.b_btn, -10., -100.)
        .set(ids.a_btn, ui);

    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("A")
            .color(text_color)
            .mid_top_with_margin_on(ids.a_btn, btn_label_margin)
            .set(ids.a_label, ui);
    }

    let (btn, mut m_text) = make_button(
        app.state.c_up,
        ids.frame,
        options.button_active_colors.c_up,
        options.button_inactive_colors.c_up,
        options.display_labels,
    );

    btn.x_y_relative_to(ids.a_btn, 1., 48.)
        .set(ids.c_up_btn, ui);

    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("CU")
            .color(text_color)
            .mid_top_with_margin_on(ids.c_up_btn, btn_label_margin)
            .set(ids.c_up_label, ui);
    }

    let (btn, mut m_text) = make_button(
        app.state.c_left,
        ids.frame,
        options.button_active_colors.c_left,
        options.button_inactive_colors.c_left,
        options.display_labels,
    );

    btn.x_y_relative_to(ids.c_up_btn, -34., -24.)
        .set(ids.c_left_btn, ui);

    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("CL")
            .color(text_color)
            .mid_top_with_margin_on(ids.c_left_btn, btn_label_margin)
            .set(ids.c_left_label, ui);
    }

    let (btn, mut m_text) = make_button(
        app.state.c_right,
        ids.frame,
        options.button_active_colors.c_right,
        options.button_inactive_colors.c_right,
        options.display_labels,
    );

    btn.x_y_relative_to(ids.c_up_btn, 34., -24.)
        .set(ids.c_right_btn, ui);

    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("CR")
            .color(text_color)
            .mid_top_with_margin_on(ids.c_right_btn, btn_label_margin)
            .set(ids.c_right_label, ui);
    }

    let (btn, mut m_text) = make_button(
        app.state.c_down,
        ids.frame,
        options.button_active_colors.c_down,
        options.button_inactive_colors.c_down,
        options.display_labels,
    );

    btn.x_y_relative_to(ids.c_left_btn, 0., -48.)
        .set(ids.c_down_btn, ui);

    if let Some(text_color) = m_text.take() {
        conrod_core::widget::Text::new("CD")
            .color(text_color)
            .mid_top_with_margin_on(ids.c_down_btn, btn_label_margin)
            .set(ids.c_down_label, ui);
    }

    if options.is_r2_b0xx {
        let (btn, mut m_text) = make_button(
            app.state.mod_ls,
            ids.frame,
            options.button_active_colors.mod_ls,
            options.button_inactive_colors.mod_ls,
            options.display_labels,
        );

        btn.x_y_relative_to(ids.y_btn, 43., -5.)
            .set(ids.mod_ls_btn, ui);

        if let Some(text_color) = m_text.take() {
            conrod_core::widget::Text::new("LS")
                .color(text_color)
                .mid_top_with_margin_on(ids.mod_ls_btn, btn_label_margin)
                .set(ids.mod_ls_label, ui);
        }

        let (btn, mut m_text) = make_button(
            app.state.mod_ms,
            ids.frame,
            options.button_active_colors.mod_ms,
            options.button_inactive_colors.mod_ms,
            options.display_labels,
        );

        btn.x_y_relative_to(ids.mod_ls_btn, 40., -17.)
            .set(ids.mod_ms_btn, ui);

        if let Some(text_color) = m_text.take() {
            conrod_core::widget::Text::new("MS")
                .color(text_color)
                .mid_top_with_margin_on(ids.mod_ms_btn, btn_label_margin)
                .set(ids.mod_ms_label, ui);
        }
    }

    fps_counter(ui, ids, app);
}

#[cfg(not(feature = "fps"))]
fn fps_counter(_: &mut conrod_core::UiCell, _: &Ids, _: &mut ViewerApp) {}

#[cfg(feature = "fps")]
fn fps_counter(ui: &mut conrod_core::UiCell, ids: &Ids, app: &mut ViewerApp) {
    use conrod_core::{color, widget, Colorable, Positionable, Widget};

    let fps = app.fps.tick();
    widget::Text::new(&fps.to_string())
        .color(color::YELLOW)
        .top_right_with_margin_on(ids.frame, 10.)
        .floating(true)
        .set(ids.fps_counter, ui);
}

#[inline(always)]
fn make_button(
    state: bool,
    parent: conrod_core::widget::Id,
    active_color: crate::config::ViewerColor,
    inactive_color: crate::config::ViewerColor,
    display_labels: bool,
) -> (
    conrod_core::widget::Oval<conrod_core::widget::primitive::shape::oval::Full>,
    Option<conrod_core::Color>,
) {
    use conrod_core::{widget, Colorable, Sizeable, Widget};

    let color = if state { active_color } else { inactive_color };
    let text_color = if display_labels {
        let tmp: conrod_core::Color = color.clone().into();
        Some(tmp.plain_contrast())
    } else {
        None
    };

    (
        widget::Circle::fill(BTN_RADIUS)
            .color(color.into())
            .parent(parent)
            .graphics_for(parent)
            .w_h(BTN_RADIUS, BTN_RADIUS),
        text_color,
    )
}

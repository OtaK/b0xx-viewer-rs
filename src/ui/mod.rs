mod app;
mod gui;
mod support;

use self::{app::*, support::*};

use crate::{config::ViewerOptions, probe::*};

use conrod_core::widget_ids;
use glium::{self, Surface, glutin::event::ModifiersState};
use conrod_glium::Renderer;

const ALATA_FONT: &[u8] = include_bytes!("../../assets/fonts/Alata-Regular.ttf");

#[allow(unused_imports)]
use conrod_winit::{
    convert_event, convert_key, convert_mouse_button, convert_mouse_cursor, convert_window_event,
};

conrod_winit::v023_conversion_fns!();

widget_ids! {
    pub struct Ids {
        frame,
        reconnect_bg,
        reconnect_label,
        start_btn,
        y_btn,
        x_btn,
        b_btn,
        a_btn,
        l_btn,
        r_btn,
        z_btn,
        up_btn,
        down_btn,
        right_btn,
        left_btn,
        mod_x_btn,
        mod_y_btn,
        mod_ls_btn,
        mod_ms_btn,
        c_left_btn,
        c_right_btn,
        c_up_btn,
        c_down_btn,
        start_label,
        y_label,
        x_label,
        b_label,
        a_label,
        l_label,
        r_label,
        z_label,
        up_label,
        down_label,
        right_label,
        left_label,
        mod_x_label,
        mod_y_label,
        mod_ls_label,
        mod_ms_label,
        c_left_label,
        c_right_label,
        c_up_label,
        c_down_label,
        fps_counter,
    }
}

pub fn start_gui(mut rx: crossbeam_channel::Receiver<ControllerMessage>, options: ViewerOptions) {
    // Build the window.
    let mut events_loop = glium::glutin::event_loop::EventLoop::new();

    let window = glium::glutin::window::WindowBuilder::new()
        .with_decorations(!options.chromeless)
        .with_title(WIN_TITLE)
        .with_resizable(false)
        .with_inner_size::<glium::glutin::dpi::LogicalSize<u32>>((WIN_W, WIN_H).into());

    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_gl_robustness(if cfg!(profile = "release") {
            glium::glutin::Robustness::NoError
        } else {
            glium::glutin::Robustness::TryRobustLoseContextOnReset
        })
        .with_multisampling(4);

    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let display = GliumDisplayWinitWrapper(display);

    // Construct our `Ui`.
    let mut ui = conrod_core::UiBuilder::new([WIN_W as f64, WIN_H as f64])
        // TODO: Change theme depending if the controller is a b0xx or a f1 controller
        .theme(gui::b0xx_theme())
        .build();

    ui.set_num_redraw_frames(1);

    let alata_font = ui
        .fonts
        .insert(rusttype::Font::from_bytes(ALATA_FONT).unwrap());

    ui.theme.font_id = Some(alata_font);

    let ids = Ids::new(ui.widget_id_generator());

    let image_map: conrod_core::image::Map<glium::texture::CompressedSrgbTexture2d> =
        conrod_core::image::Map::new();

    let mut app = ViewerApp::default();

    let mut renderer = Renderer::new(&display).unwrap();

    let (glutin_tx, glutin_rx) = crossbeam_channel::bounded::<()>(1);

    'main: loop {
        // FIXME: Fix this after the traits are fixed
        // Reconnect to the device if needed
        // if app.status == ViewerAppStatus::NeedsReconnection {
        //     app.status = ViewerAppStatus::Reconnecting;
        //     debug!("Trying to reconnect...");
        //     drop(rx);
        //     rx = reconnect(&options.custom_tty);
        //     debug!("Reconnected successfully!");
        // }

        let mut maybe_state = match rx.iter().next() {
            Some(message) => match message {
                ControllerMessage::State(state) => {
                    app.status.set_running();
                    Some(state)
                }
                ControllerMessage::Error(e) => {
                    error!("{}", e);
                    app.status = ViewerAppStatus::NeedsReconnection;
                    None
                }
                ControllerMessage::Quit => {
                    break 'main;
                }
                ControllerMessage::Reconnect => {
                    app.status = ViewerAppStatus::NeedsReconnection;
                    None
                }
            },
            None => None,
        };

        // Redraw our window contents only and only if the state of inputs have
        // changed in the current cached report
        if let Some(new_state) = maybe_state.take() {
            if app.update_state(new_state) {
                ui.handle_event(conrod_core::event::Input::Redraw);
            }
        }

        // Window event processing
        use glium::glutin::platform::desktop::EventLoopExtDesktop as _;
        events_loop.run_return(|event, _, control_flow| {
            match event {
                glium::glutin::event::Event::WindowEvent { event, .. } => match event {
                    // Exit the program upon pressing `Escape`.
                    glium::glutin::event::WindowEvent::CloseRequested
                    | glium::glutin::event::WindowEvent::KeyboardInput {
                        input:
                            glium::glutin::event::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::event::VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => {
                        let _ = glutin_tx.send(());
                    },
                    // If ALT is held, allow the window to be click-dragged
                    glium::glutin::event::WindowEvent::ModifiersChanged(modifiers) => {
                        if modifiers.contains(ModifiersState::ALT) {
                            app.is_draggable = true;
                        } else {
                            app.is_draggable = false;
                            app.is_dragged = false;
                        }
                    },
                    glium::glutin::event::WindowEvent::MouseInput {
                        button: glium::glutin::event::MouseButton::Left,
                        state,
                        ..
                    } if app.is_draggable => {
                        app.is_dragged = state == glium::glutin::event::ElementState::Pressed;
                    },
                    _ => {}
                },
                glium::glutin::event::Event::DeviceEvent {
                    event: glium::glutin::event::DeviceEvent::MouseMotion { delta: (dx, dy)},
                    ..
                } if app.is_dragged => {
                    let prev_pos = display.0
                        .gl_window()
                        .window()
                        .outer_position()
                        .unwrap_or_else(|_| glium::glutin::dpi::PhysicalPosition::new(0, 0));

                    display.0
                        .gl_window()
                        .window()
                        .set_outer_position(glium::glutin::dpi::PhysicalPosition::new(
                            prev_pos.x as f64 + dx,
                            prev_pos.y as f64 + dy,
                        ));
                },
                _ => {}
            }
            *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
        });

        if let Ok(_) = glutin_rx.try_recv() {
            break 'main;
        }

        // Instantiate the parallelograph viewer GUI
        gui::render_gui(&mut ui.set_widgets(), &ids, &mut app, &options);

        // Draw the `Ui`.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display.0, primitives, &image_map);
            let mut frame = display.0.draw();
            frame.clear_color(0., 0., 0., 1.);
            renderer.draw(&display.0, &mut frame, &image_map).unwrap();
            frame.finish().unwrap();
        }
    }
}

pub const WIN_W: u32 = 600;
pub const WIN_H: u32 = 300;
pub const BTN_RADIUS: f64 = 40.;

#[cfg(not(feature = "fake_serial"))]
pub const WIN_TITLE: &str = "B0XX Input Viewer - by @OtaK_";

#[cfg(feature = "fake_serial")]
pub const WIN_TITLE: &str = "B0XX Input Viewer - by @OtaK_ [FAKE SERIAL MODE]";

pub struct GliumDisplayWinitWrapper(pub glium::Display);

impl conrod_winit::WinitWindow for GliumDisplayWinitWrapper {
    fn get_inner_size(&self) -> Option<(u32, u32)> {
        (**self.0.gl_window())
            // TODO: Upgrade to Glium/Glutin's new versions and switch out this code for it to work
            //.window()
            .get_inner_size()
            .map(Into::into)
    }
    fn hidpi_factor(&self) -> f32 {
        (**self.0.gl_window())
            //.window()
            .get_hidpi_factor() as _
    }
}

impl glium::backend::Facade for GliumDisplayWinitWrapper {
    fn get_context(&self) -> &std::rc::Rc<glium::backend::Context> {
        self.0.get_context()
    }
}

use crate::b0xx_state::B0xxState;

#[derive(Debug, Default, Clone)]
pub struct ViewerApp {
    pub state: B0xxState,
}

impl ViewerApp {
    pub fn update_state(&mut self, new_state: B0xxState) -> bool {
        if self.state == new_state {
            return false;
        }

        self.state = new_state;
        true
    }
}

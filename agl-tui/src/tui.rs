use crate::app::AppState;

pub struct Tui {
    pub state: AppState,
}

impl Tui {
    pub fn new() -> Self {
        Self { state: AppState::default() }
    }
}

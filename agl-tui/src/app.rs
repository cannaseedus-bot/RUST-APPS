#[derive(Debug, Default)]
pub struct AppState {
    pub active_panel: String,
    pub active_model: String,
    pub command_buffer: String,
    pub logs: Vec<String>,
}

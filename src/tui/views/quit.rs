use super::view::View;

pub struct QuitView;
impl View for QuitView {
    fn should_close(&self) -> bool {
        true
    }
    fn set_title(&self) {}
}

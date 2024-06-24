use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;

#[derive(Debug, Default)]
struct Playground {
    worm: Vec<(usize, usize)>,
}

impl Widget for Playground {
    fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
        todo!()
    }
}

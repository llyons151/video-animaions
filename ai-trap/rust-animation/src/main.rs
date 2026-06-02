use crossterm::event;
use ratatui::layout::{Alignment, Rect};
use ratatui::widgets::{Block, BorderType, Paragraph, Wrap, Padding};

fn border_box() -> Block<'static> {
    Block::bordered()
        .border_type(BorderType::Rounded)
        .title("Based Animation")
        .title_alignment(Alignment::Center)
}

fn inner_box(body_text: &'static str, box_height: u16) -> Paragraph<'static> {
    // Vertically center one line: borders take 2 rows, text 1, split the rest above.
    let top_pad = box_height.saturating_sub(3) / 2;
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .padding(Padding::new(0, 0, top_pad, 0))
        .title_alignment(Alignment::Center);

    Paragraph::new(body_text)
        .block(block)
        .centered()
        .wrap(Wrap { trim: true })
}

const BOX_TEXT: [&str; 5] = [
    "Just Use AI",
    "You Use It Wrong",
    "The Guilt Settles In",
    "The Over Correction",
    "You Fall Behind",
];

fn main() -> std::io::Result<()> {
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| {

                frame.render_widget(border_box(), frame.area());

                let box_width = 30;
                let box_height = 7;
                let hw = box_width / 2;  // half box, so points mark box centers
                let hh = box_height / 2;
                let cx = frame.area().width / 2;  // center of the screen
                let cy = frame.area().height / 2;

                // Pentagon points, clockwise from the top, so the cycle reads in
                // story order: Just Use AI -> Wrong -> Guilt -> Overcorrect -> Behind -> (loops).
                let positions = [
                    (cx - hw,      cy - 15 - hh, box_width, box_height), // top
                    (cx + 45 - hw, cy - 5  - hh, box_width, box_height), // upper right
                    (cx + 27 - hw, cy + 12 - hh, box_width, box_height), // lower right
                    (cx - 27 - hw, cy + 12 - hh, box_width, box_height), // lower left
                    (cx - 45 - hw, cy - 5  - hh, box_width, box_height), // upper left
                ];

                for i in 0..BOX_TEXT.len() {
                    let (x, y, width, height) = positions[i];
                    let area = Rect { x, y, width, height };
                    frame.render_widget(inner_box(BOX_TEXT[i], height), area);
                }
            })?;

            if event::read()?.is_key_press() {
                break Ok(());
            }
        }
    })
}

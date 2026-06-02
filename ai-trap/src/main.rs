use ratatui::widgets::{Block, BorderType, Paragraph, Wrap, Padding};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::{Alignment, Rect};
use std::time::{Duration, Instant};

fn border_box() -> Block<'static> {
    Block::bordered()
        .border_type(BorderType::Rounded)
        .title("Based Animation")
        .title_alignment(Alignment::Center)
}

fn inner_box(body_text: &'static str, box_height: u16) -> Paragraph<'static> {
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
    let mut visible = 0;
    let mut anim_start = Instant::now();
    ratatui::run(|terminal| {
        loop {
            let t = (anim_start.elapsed().as_secs_f32() / 0.35).min(1.0);
            terminal.draw(|frame| {

                frame.render_widget(border_box(), frame.area());

                frame.render_widget(
                    Paragraph::new(format!("t = {t:.2}")),
                    Rect { x: 0, y: 0, width: 12, height: 1 },
                );
                let box_width = 30;
                let box_height = 7;
                let hw = box_width / 2;
                let hh = box_height / 2;
                let cx = frame.area().width / 2;
                let cy = frame.area().height / 2;

                let positions = [
                    (cx - hw,      cy - 15 - hh, box_width, box_height),
                    (cx + 45 - hw, cy - 5  - hh, box_width, box_height),
                    (cx + 27 - hw, cy + 12 - hh, box_width, box_height),
                    (cx - 27 - hw, cy + 12 - hh, box_width, box_height),
                    (cx - 45 - hw, cy - 5  - hh, box_width, box_height),
                ];

                for i in 0..visible {
                    let (x, y, width, height) = positions[i];
                    let bt = if i == visible - 1 { t } else { 1.0 };
                    let w = (width as f32 * bt) as u16;
                    let h = (height as f32 * bt) as u16;
                    let area = Rect { x: x + (width - w) / 2, y: y + (height - h) / 2, width: w, height: h };
                    frame.render_widget(inner_box(BOX_TEXT[i], height), area);
                }
            })?;

            if event::poll(Duration::from_millis(16))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind != KeyEventKind::Press { continue; }
                    if key.code == KeyCode::Char('q') { break Ok(()); }
                    if key.code == KeyCode::Char(' ') {
                        if visible == BOX_TEXT.len() { continue; }
                        anim_start = Instant::now();
                        visible += 1;
                    }
                }
            }
        }
    })
}

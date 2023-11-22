///┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
///┃ 1 ┃ 2 ┃ 3 ┃ 4 ┃ 5 ┃ 6 ┃ 7 ┃ 8 ┃ 9 ┃ 10 ┃                           
///┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
use ratatui::{
    layout::*,
    widgets::*,
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    let layout = Layout::default()
        .direction( Direction::Horizontal )
        .constraints( vec! [
            Constraint::Percentage( 10 ),
            Constraint::Percentage( 85 ),
            Constraint::Percentage( 5 ),
        ]).split( frame.size() );

    frame.render_widget(
        Paragraph::new( format!( " 1 ┃ 2 ┃ 3 ┃ 4 ┃ 5 ┃ 6 ┃ 7 ┃ 8 ┃ 9 ┃ 10 " ) )
        .block( Block::new().borders( Borders::ALL ) )
        .alignment(Alignment::Center),
        layout[0],
    );

    frame.render_widget(
        Paragraph::new( format!( "{}", app.time ) )
        .block( Block::new().borders( Borders::ALL ) )
        .alignment(Alignment::Center),
        layout[2],
    )
}

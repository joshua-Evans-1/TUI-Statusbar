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
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Length( 5 ),
            Constraint::Length( 5 ),
            Constraint::Length( 5 ),
            Constraint::Length( 5 ),
            Constraint::Length( 5 ),
            Constraint::Length( 5 ),
            Constraint::Length( 5 ),
            Constraint::Length( 5 ),
            Constraint::Length( 5 ),
            Constraint::Length( 6 ),
            Constraint::Min( 0 ),

            Constraint::Length( 8 ),
            Constraint::Length( 8 ),
            Constraint::Length( 12 ),
        ]).split(frame.size());

    frame.render_widget(
        Paragraph::new( format!( " 1 " ) )
        .block( Block::new().borders( Borders::ALL ) )
        .alignment(Alignment::Center),
        layout[0],
    );


    frame.render_widget(
        Paragraph::new( format!( " 2 " ) )
        .block( Block::new().borders( Borders::ALL ) )
        .alignment(Alignment::Center),
        layout[1],
    );


    frame.render_widget(
        Paragraph::new( format!( " 3 " ) )
        .block( Block::new().borders( Borders::ALL ) )
        .alignment(Alignment::Center),
        layout[2],
    );


    frame.render_widget(
        Paragraph::new( format!( " 4 " ) )
        .block( Block::new().borders( Borders::ALL ) )
        .alignment(Alignment::Center),
        layout[3],
    );


    frame.render_widget(
        Paragraph::new( format!( " 5 " ) )
        .block( Block::new().borders( Borders::ALL ) )
        .alignment(Alignment::Center),
        layout[4],
    );


    frame.render_widget(
        Paragraph::new( format!( " 6 " ) )
        .block( Block::new().borders( Borders::ALL ) )
        .alignment(Alignment::Center),
        layout[5],
    );


    frame.render_widget(
        Paragraph::new( format!( " 7 " ) )
        .block( Block::new().borders( Borders::ALL ) )
        .alignment(Alignment::Center),
        layout[6],
    );


    frame.render_widget(
        Paragraph::new( format!( " 8 " ) )
        .block( Block::new().borders( Borders::ALL ) )
        .alignment(Alignment::Center),
        layout[7],
    );


    frame.render_widget(
        Paragraph::new( format!( " 9 " ) )
        .block( Block::new().borders( Borders::ALL ) )
        .alignment(Alignment::Center),
        layout[8],
    );


    frame.render_widget(
        Paragraph::new( format!( " 10 " ) )
        .block( Block::new().borders( Borders::ALL ) )
        .alignment(Alignment::Center),
        layout[9],
    );
    frame.render_widget(
        Paragraph::new( format!( " vol- " ) )
        .block( Block::new().borders( Borders::ALL ) )
        .alignment(Alignment::Center),
        layout[11],
    );   
    frame.render_widget(
        Paragraph::new( format!( " vol+ " ) )
        .block( Block::new().borders( Borders::ALL ) )
        .alignment(Alignment::Center),
        layout[12],
    );
    frame.render_widget(
        Paragraph::new( format!( "{}", app.time ) )
        .block( Block::new().borders( Borders::ALL ) )
        .alignment(Alignment::Center),
        layout[13],
    )
}

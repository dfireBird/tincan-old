use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode, KeyModifiers},
    execute,
    terminal::LeaveAlternateScreen,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
};
use io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::Modifier,
    style::{Color, Style},
    text::{Span, Spans, Text},
    widgets::Block,
    widgets::Borders,
    widgets::List,
    widgets::ListItem,
    widgets::Paragraph,
    Terminal,
};

use std::{error::Error, io};

use super::events::{Event, Events};
use super::state::State;

pub fn start_ui() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut events = Events::new();

    let mut state = State::default();

    // UI loop
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Min(1),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            // Help Message
            let (msg, style) = (
                vec![
                    Span::raw("Press "),
                    Span::styled("Ctrl-D", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to exit."),
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            );
            let mut help_text = Text::from(Spans::from(msg));
            help_text.patch_style(style);
            f.render_widget(Paragraph::new(help_text), chunks[0]);

            // Messages Block
            let messages: Vec<ListItem> = state
                .messages
                .iter()
                .enumerate()
                .map(|(i, m)| {
                    let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
                    ListItem::new(content)
                })
                .collect();
            let messages =
                List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
            f.render_widget(messages, chunks[1]);

            // Input Block
            let input = Paragraph::new(state.input.as_ref())
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Input"));
            f.render_widget(input, chunks[2]);
        })?;

        // Handle Input
        if let Event::Input(input) = events.next()? {
            match input.code {
                KeyCode::Enter => {
                    state.messages.push(state.input.drain(..).collect());
                }
                KeyCode::Char(c) if c == 'd' && input.modifiers == KeyModifiers::CONTROL => {
                    terminal_deinitialization(&mut terminal)?;
                    break;
                }
                KeyCode::Char(c) if c == 'c' && input.modifiers == KeyModifiers::CONTROL => {
                    terminal_deinitialization(&mut terminal)?;
                    break;
                }
                KeyCode::Char(c) => {
                    state.input.push(c);
                }
                KeyCode::Backspace => {
                    state.input.pop();
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn terminal_deinitialization(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
use tui::widgets::Wrap;
use tui::style::Style;
use tui::style::Modifier;
use tui::style::Color;
use tui::text::Span;
use tui::text::Spans;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout,Rect},
    widgets::{Block, Borders,Paragraph},
    Frame, Terminal,
};


fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    // create app and run it
    cli_formater::set_cmd_box_format();
    let res = run_app(&mut terminal);
    
    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    
    if let Err(err) = res {
        println!("{:?}", err)
    }
    
    
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f))?;
        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let body = Layout::default()              // HEADER,SPACING,BODY,...
    .direction(Direction::Vertical)
    .margin(0)
    .constraints([  Constraint::Length(6),
                    Constraint::Length(2),
                    Constraint::Percentage(53),
                    Constraint::Percentage(40)].as_ref()
    ).split(f.size());

    let header = Layout::default()
    .direction(Direction::Horizontal)
    .margin(0)
    .constraints([  Constraint::Percentage(70),
                    Constraint::Percentage(30)].as_ref()
    ).split(body[0]);

    let cmd_body = Layout::default()
    .direction(Direction::Horizontal)
    .margin(0)
    .constraints([  Constraint::Percentage(20),
                    Constraint::Percentage(80)].as_ref()
    ).split(body[2]);

    let body = Layout::default() 
    .direction(Direction::Vertical)
    .margin(0)
    .constraints([  Constraint::Length(8),
                    Constraint::Percentage(200)].as_ref()
    ).split(f.size());

    let log_body = Layout::default()
    .direction(Direction::Horizontal)
    .margin(0)
    .constraints([  Constraint::Percentage(30),
                    Constraint::Percentage(70)].as_ref()
    ).split(body[1]);



    
    let text = vec![
        Spans::from(vec![ 
            Span::raw("     ____ "),
            Span::styled("  ______ ",Style::default().fg(Color::Cyan)),
            Span::raw("    __           __                                               "),
        ]),
        Spans::from(vec![
            Span::raw("    / __ \\"),
            Span::styled(" / ____ \\ ",Style::default().fg(Color::Cyan)),
            Span::raw("__/ /_______   (_)._____ _____ ____   ____  _____ ___       "),
        ]),
        Spans::from(vec![
            Span::raw("   / /_/ /"),
            Span::styled("/ / __ `/",Style::default().fg(Color::Cyan)),
            Span::raw("/_  ____/ __ \\ / // ____// ___// __ \\ / __ \\ / __ `__ \\     "),
        ]),
        Spans::from(vec![
            Span::raw("  / ____/"),
            Span::styled("/ / /_/ /",Style::default().fg(Color::Cyan)),
            Span::raw(" /  /_  / / / // // |___ / /   / /_/ // /_/ // / / / / /        "),
        ]),
        Spans::from(vec![
            Span::raw(" /_/    "),
            Span::styled(" \\ \\____/_",Style::default().fg(Color::Cyan)),
            Span::raw("|_____|/_/_/_//_/"),
            Span::styled("_",Style::default().fg(Color::Cyan)),
            Span::raw("\\____//_/"),
            Span::styled("____",Style::default().fg(Color::Cyan)),
            Span::raw("\\____/"),
            Span::styled("_",Style::default().fg(Color::Cyan)),
            Span::raw("\\____//_/_/_/_/_/"),
            Span::styled("______. ",Style::default().fg(Color::Cyan)),
        ]),

        Spans::from(vec![
            Span::raw("         "),
            Span::styled(" \\______",Style::default().fg(Color::Cyan)),
            Span::styled("_______________________________________________________________\\ ",Style::default().fg(Color::Cyan)),
        ]),
    ];

    let menues = vec![
        Spans::from(vec![Span::styled("> Backup",Style::default().fg(Color::Cyan)),]),
        Spans::from(vec![Span::styled("> Syncronice",Style::default().fg(Color::Cyan)),]),
        Spans::from(vec![Span::styled("> Distribute",Style::default().fg(Color::Cyan)),]),
        Spans::from(vec![Span::styled("> Concatinated",Style::default().fg(Color::Cyan)),]),
    ];

    let title = Paragraph::new(text);
    let tabs = Paragraph::new(menues).block(Block::default().title("Tabs").borders(Borders::ALL));
    let cmd = Block::default().title("CMD").borders(Borders::ALL);
    let log = Block::default().title("LOG").borders(Borders::ALL);


    f.render_widget(tabs, header[1]);
    f.render_widget(title, header[0]);
    f.render_widget(cmd, cmd_body[0]);
    f.render_widget(log, log_body[1]);
       
}






pub mod cli_formater{
    use std::io::{stdout, Write};
    use crossterm::style::*;
    use crossterm::terminal::*;
    use crossterm::*;
    pub fn set_cmd_box_format()-> Result<()>{
        execute!(
            stdout(),
            SetBackgroundColor(Color::Rgb { r:0,g:0,b:15 }),
            SetSize(120, 33),
           ScrollUp(50)
        )?;Ok(())
    }

}




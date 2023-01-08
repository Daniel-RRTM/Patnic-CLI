use std::io::{stdout};
use crossterm::style::*;
use crossterm::terminal::*;
use crossterm::*;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders,Paragraph},
    Frame,
    style::{Style,Color},
    text::{Span,Spans},
};


pub fn draw_landing_ui<B: Backend>(f: &mut Frame<B>) {
    let cmd = Paragraph::new("").block(Block::default().title("CMD").borders(Borders::ALL));
    let log = Paragraph::new("").block(Block::default().title("LOG").borders(Borders::ALL));
    draw_ui(cmd,log,f);
}

pub fn draw_backup_ui<B: Backend>(f: &mut Frame<B>) {
    let text = Spans::from(vec![
        Span::styled("1 > create",Style::default().fg(Color::Cyan)),
    ]);

    let cmd = Paragraph::new(text).block(Block::default().title("CMD").borders(Borders::ALL));
    let log = Paragraph::new("").block(Block::default().title("LOG").borders(Borders::ALL));
    draw_ui(cmd,log,f);
}




fn draw_ui<B: Backend>(cmd:Paragraph, log:Paragraph, f: &mut Frame<B>) {
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


    let menues = vec![
        Spans::from(vec![Span::styled("     ^ Backup",Style::default().fg(Color::Cyan)),]),
        Spans::from(vec![Span::styled("     < Syncronice",Style::default().fg(Color::Cyan)),]),
        Spans::from(vec![Span::styled("     > Distribute",Style::default().fg(Color::Cyan)),]),
        Spans::from(vec![Span::styled("     v Concatinated",Style::default().fg(Color::Cyan)),]),
    ];
    
    let tabs = Paragraph::new(menues).block(Block::default().title("Tabs").borders(Borders::ALL));
    let title = Paragraph::new(get_ascii_art());


    f.render_widget(tabs, header[1]);
    f.render_widget(title, header[0]);
    f.render_widget(cmd, cmd_body[0]);
    f.render_widget(log, log_body[1]);
       
}



pub fn set_cmd_box_format()-> Result<()>{
    execute!(
        stdout(),
        SetBackgroundColor(crossterm::style::Color::Rgb { r:0,g:0,b:15 }),
        SetSize(120, 33),
       ScrollUp(50)
    )?;Ok(())
}



fn get_ascii_art()->Vec<tui::text::Spans<'static>>{
    return vec![
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
        ]),];
}
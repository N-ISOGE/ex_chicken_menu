use std::io::stdout;

use crossterm::{
    event::{self, KeyCode, KeyEventKind, KeyEvent},
    ExecutableCommand,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{self, prelude::*, widgets::*};


pub fn 치킨_main() -> std::io::Result<()> {
    // set up terminal
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut terminal =
        Terminal::new(CrosstermBackend::new(stdout()))?;

    let result_app = run_app(&mut terminal);

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    terminal.clear()?;

    if let Err(err) = result_app { println!("{err:?}"); }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> std::io::Result<()> {
    // main loop가 들어갈 곳
    loop {
        // 화면에 그리고
        terminal.draw(ui::<B>)?;

        if let Ok(res) = input_handler::<B>() {
            if res.kind == KeyEventKind::Press && res.code == KeyCode::Char('ㅂ') {
                return Ok(());
            }
        }
    }
}

// enum HandlerError {
//     NoInput
// }

fn input_handler<B: Backend>() -> std::io::Result<KeyEvent> {
    // 이벤트를 처리함
    if event::poll(std::time::Duration::from_millis(16))? {
        if let event::Event::Key(key) = event::read()? {
            return Ok(key);
        }
    }
    return Err(std::io::Error::other("no input"));
}

fn ui<B: Backend>(frame: &mut ratatui::terminal::Frame) { // 화면에 그리고
    let size = frame.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(3)
        ])
        .split(size);


    let upper_block = Block::default()
        .borders(Borders::NONE)
        .title(ratatui::widgets::block::Title::from("My chicken menu"));
    frame.render_widget(upper_block, chunks[0]);
    let middle_panel = Paragraph::new("안녕 Ratatui! (나가려면 'ㅂ'를 입력)")
        .white()
        .on_blue();
    frame.render_widget(middle_panel, chunks[1]);
    let lower_block = Block::default()
        .borders(Borders::ALL)
        .title(ratatui::widgets::block::Title::from("하단 상태 표시"));
    frame.render_widget(lower_block, chunks[2]);
}
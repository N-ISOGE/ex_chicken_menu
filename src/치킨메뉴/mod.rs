use std::io::stdout;

use crossterm::{event, terminal, ExecutableCommand};
use ratatui::{layout, prelude, widgets};

pub fn main() -> std::io::Result<()> {
    // set up terminal
    stdout().execute(terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let mut terminal = ratatui::Terminal::new(prelude::CrosstermBackend::new(stdout()))?;

    let result_app = run_app(&mut terminal);

    // restore terminal
    terminal::disable_raw_mode()?;
    stdout().execute(terminal::LeaveAlternateScreen)?;
    terminal.clear()?;

    if let Err(err) = result_app {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: prelude::Backend>(terminal: &mut prelude::Terminal<B>) -> std::io::Result<()> {
    // main loop가 들어갈 곳
    loop {
        // 화면 그리기
        terminal.draw(ui::<B>)?;

        // 입력 처리
        if let Ok(res) = input_handler::<B>() {
            if res.kind == event::KeyEventKind::Press && res.code == event::KeyCode::Char('ㅂ') {
                return Ok(());
            }
        }
    }
}

// enum HandlerError {
//     NoInput
// }

fn input_handler<B: prelude::Backend>() -> std::io::Result<event::KeyEvent> {
    // 이벤트를 처리함
    if crossterm::event::poll(std::time::Duration::from_millis(16))? {
        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            return Ok(key);
        }
    }
    return Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "no input",
    ));
}

fn ui<B: prelude::Backend>(frame: &mut ratatui::terminal::Frame) {
    // 화면에 그리기
    // 레이아웃 결정
    let size = frame.size();

    let chunks = layout::Layout::default()
        .direction(layout::Direction::Vertical)
        .constraints([
            prelude::Constraint::Length(2),
            prelude::Constraint::Min(0),
            prelude::Constraint::Length(3),
        ])
        .split(size);

    // 구역의 속성 설정

    let upper_block = widgets::Block::default()
        .borders(widgets::Borders::NONE)
        .title(widgets::block::Title::from("My chicken menu"));
    frame.render_widget(upper_block, chunks[0]);

    let lower_block = widgets::Block::default()
        .borders(widgets::Borders::ALL)
        .title(widgets::block::Title::from("하단 상태 표시"));
    frame.render_widget(lower_block, chunks[2]);

    {
        use ratatui::style::Stylize;
        let middle_panel = widgets::Paragraph::new("안녕 Ratatui! (나가려면 'ㅂ'를 입력)")
            .on_blue()
            .white();
        frame.render_widget(middle_panel, chunks[1]);
    }
}

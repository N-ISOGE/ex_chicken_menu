use std::io::{Result, stdout};

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    ExecutableCommand,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};

pub fn 치킨_main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // main loop가 들어갈 곳
    loop{
        // 화면에 그리고
        terminal.draw(|frame|{
            let area = frame.size();
            frame.render_widget(
                Paragraph::new("안녕 Ratatui! (나가려면 'ㅂ'를 입력)")
                    .white()
                    .on_blue(),
                area
            );
        })?;
        // 이벤트를 처리함
        if event::poll(std::time::Duration::from_millis(16))?{
            if let event::Event::Key(key)= event::read()?{
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('ㅂ'){
                    break;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
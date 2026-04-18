mod keypress;
mod rerender;
mod UI;
mod fileIO;
use std::io::{stdout, Write};
use std::io;
use std::path::PathBuf;
use crossterm::event::{KeyEvent, KeyModifiers};
use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode};
use crossterm::{
    event::KeyCode,
    execute,
    terminal::{Clear, ClearType},
    cursor::MoveTo,
};
use crate::fileIO::open_folder;
use crate::rerender::efficiet_render;

fn clear_screen() -> std::io::Result<()> {
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    enable_raw_mode()?;
    let input = keypress::Input::new()?;
    let mut running = true;
    let mut text = String::from("hello mfs");
    let (term_width, _) = terminal::size()?;
    let mut curr_content = String::new();
    let mut pre_content = String::new();
    let mut screen_kind = "codespace";
    clear_screen()?;

    while running {
        while let Some(code) = input.try_read() {
            match code {
                KeyEvent { code: KeyCode::Char('q'), modifiers: KeyModifiers::CONTROL, .. } => {
                    running = false;
                }
                KeyEvent { code: KeyCode::Char('o'), modifiers: KeyModifiers::CONTROL, .. } => {
                    screen_kind = "explorer";
                }
                KeyEvent { code: KeyCode::Char(c), .. } => { text.push(c); }
                KeyEvent { code: KeyCode::Enter, .. } => { text.push('\n'); }
                KeyEvent { code: KeyCode::Tab, .. } => {
                    for _ in 0..4 { text.push(' '); }
                }
                KeyEvent { code: KeyCode::Backspace, .. } => { text.pop(); }
                _ => {}
            }
        }

        match screen_kind {
            "codespace" => {
                curr_content = UI::codespace_UI(&text, term_width);
            }
            "explorer" => {
                let home = dirs::home_dir()
                    .unwrap_or_else(|| PathBuf::from("/"))
                    .to_str()
                    .unwrap_or("/")
                    .to_string();
                curr_content = UI::explorer_UI(&home, term_width, open_folder(&home)?);
            }
            _ => {}
        }

        efficiet_render(&pre_content, &curr_content)?;
        pre_content = curr_content.clone();
    }

    disable_raw_mode()?;
    Ok(())
}
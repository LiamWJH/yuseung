use std::collections::HashMap;
use crossterm::terminal;
use std::io::{self, Write};

pub fn idx_to_row_col(index: usize, content: &str) -> (u16, u16) {
    let mut row = 0u16;
    let mut col = 0u16;
    for (i, ch) in content.chars().enumerate() {
        if i == index {
            break;
        }
        if ch == '\n' {
            row += 1;
            col = 0;
        } else if ch == '\t' {
            col += 4;
        } else {
            col += 1;
        }
    }
    (row, col)
}

pub fn efficiet_render(old_buffer: &str, new_buffer: &str) -> Result<(), std::io::Error> {
    let longer_len = old_buffer.len().max(new_buffer.len());
    let old_chars: Vec<char> = old_buffer.chars().collect();
    let new_chars: Vec<char> = new_buffer.chars().collect();

    let mut diffs: HashMap<usize, Option<char>> = HashMap::new();

    for i in 0..longer_len {
        let old_ch = old_chars.get(i).copied();
        let new_ch = new_chars.get(i).copied();
        if old_ch != new_ch {
            diffs.insert(i, new_ch);
        }
    }

    let mut stdout = io::stdout().lock();

    let mut stdout = io::stdout().lock();
    for (index, ch) in &diffs {
        let (row, col) = idx_to_row_col(*index, new_buffer);
        match ch {
            Some(ch) => stdout.write_all(format!("\x1b[0m\x1b[{};{}H{ch}", row + 1, col + 1).as_bytes())?,
            None => stdout.write_all(format!("\x1b[{};{}H\x1b[K", row + 1, col + 1).as_bytes())?,
        }
    }
    stdout.flush()?;
    Ok(())
}
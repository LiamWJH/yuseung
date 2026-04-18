use crate::fileIO::FileOrDir;


pub fn codespace_UI(code_text: &str, term_width: u16) -> String {
    let br = "═".repeat(term_width as usize) + "\n";
    let title = box_UI("YUSEUNG", term_width, 1);
    let random_test_shit = "";

    format!(
"{title}
{code_text}"
    ).to_owned()
}

pub fn explorer_UI(path: &str, term_width: u16, entries: Vec<FileOrDir>) -> String {
    let path_box = box_UI(path, term_width, 1);

    let entries_list = entries.iter().map(|entry| {
        match entry {
            FileOrDir::File { name, .. } => format!("  📄 {name}"),
            FileOrDir::Folder { name, .. } => format!("  📁 {name}"),
        }
    }).collect::<Vec<String>>().join("\n");

    format!("{path_box}\n{entries_list}")
}

pub fn progressbar_UI(total_width: u16, progression: usize) -> String {
    let width = total_width as usize;
    let char_n = (progression * width) / 100;
    let remaining = width.saturating_sub(char_n);
    box_UI(
        &format!("{}{}", "█".repeat(char_n), " ".repeat(remaining)),
        total_width,
        3
    )
}

pub fn box_UI(content: &str, width: u16, height: u16) -> String {
    let top_x_bar = format!("┏{}┓", "━".repeat(width as usize - 2));
    let bottom_x_bar = format!("┗{}┛", "━".repeat(width as usize - 2));

    let x_padding_total = (width as usize - 2).saturating_sub(content.len());
    let x_pad_left = x_padding_total / 2;
    let x_pad_right = x_padding_total - x_pad_left;

    let middle = format!("┃{}{}{}┃",
        " ".repeat(x_pad_left),
        content,
        " ".repeat(x_pad_right)
    );
    let inner_width = width as usize - 2;
    let empty_row = format!("│{}│\n", " ".repeat(inner_width));

    let middle_lines = 1;
    let y_padding_total = (height as usize)
        .saturating_sub(2)
        .saturating_sub(middle_lines);

    let y_pad_top = y_padding_total / 2;
    let y_pad_bottom = y_padding_total - y_pad_top;

    let top_rows = empty_row.repeat(y_pad_top);
    let bottom_rows = empty_row.repeat(y_pad_bottom);

    format!("{top_x_bar}\n{top_rows}{middle}\n{bottom_rows}{bottom_x_bar}")
}


pub fn bold_text(content: &str) -> String {
    content.chars().map(|c| match c {
        'A' => '𝗔', 'B' => '𝗕', 'C' => '𝗖', 'D' => '𝗗', 'E' => '𝗘',
        'F' => '𝗙', 'G' => '𝗚', 'H' => '𝗛', 'I' => '𝗜', 'J' => '𝗝',
        'K' => '𝗞', 'L' => '𝗟', 'M' => '𝗠', 'N' => '𝗡', 'O' => '𝗢',
        'P' => '𝗣', 'Q' => '𝗤', 'R' => '𝗥', 'S' => '𝗦', 'T' => '𝗧',
        'U' => '𝗨', 'V' => '𝗩', 'W' => '𝗪', 'X' => '𝗫', 'Y' => '𝗬',
        'Z' => '𝗭',
        'a' => '𝗮', 'b' => '𝗯', 'c' => '𝗰', 'd' => '𝗱', 'e' => '𝗲',
        'f' => '𝗳', 'g' => '𝗴', 'h' => '𝗵', 'i' => '𝗶', 'j' => '𝗷',
        'k' => '𝗸', 'l' => '𝗹', 'm' => '𝗺', 'n' => '𝗻', 'o' => '𝗼',
        'p' => '𝗽', 'q' => '𝗾', 'r' => '𝗿', 's' => '𝘀', 't' => '𝘁',
        'u' => '𝘂', 'v' => '𝘃', 'w' => '𝘄', 'x' => '𝘅', 'y' => '𝘆',
        'z' => '𝘇',
        '0' => '𝟬', '1' => '𝟭', '2' => '𝟮', '3' => '𝟯', '4' => '𝟰',
        '5' => '𝟱', '6' => '𝟲', '7' => '𝟳', '8' => '𝟴', '9' => '𝟵',
        '!' => '！', '@' => '＠', '#' => '＃', '$' => '＄', '%' => '％',
        '^' => '＾', '&' => '＆', '*' => '＊', '(' => '（', ')' => '）',
        '-' => '－', '_' => '＿', '+' => '＋', '=' => '＝', '[' => '［',
        ']' => '］', '{' => '｛', '}' => '｝', '|' => '｜', '\\' => '＼',
        ';' => '；', ':' => '：', '\'' => '＇', '"' => '＂', ',' => '，',
        '.' => '．', '/' => '／', '?' => '？', '<' => '＜', '>' => '＞',
        ' ' => ' ',
        _ => c,
    }).collect()
}

pub fn italic_text(content: &str) -> String {
    content.chars().map(|c| match c {
        'A' => '𝘈', 'B' => '𝘉', 'C' => '𝘊', 'D' => '𝘋', 'E' => '𝘌',
        'F' => '𝘍', 'G' => '𝘎', 'H' => '𝘏', 'I' => '𝘐', 'J' => '𝘑',
        'K' => '𝘒', 'L' => '𝘓', 'M' => '𝘔', 'N' => '𝘕', 'O' => '𝘖',
        'P' => '𝘗', 'Q' => '𝘘', 'R' => '𝘙', 'S' => '𝘚', 'T' => '𝘛',
        'U' => '𝘜', 'V' => '𝘝', 'W' => '𝘞', 'X' => '𝘟', 'Y' => '𝘠',
        'Z' => '𝘡',
        'a' => '𝘢', 'b' => '𝘣', 'c' => '𝘤', 'd' => '𝘥', 'e' => '𝘦',
        'f' => '𝘧', 'g' => '𝘨', 'h' => '𝘩', 'i' => '𝘪', 'j' => '𝘫',
        'k' => '𝘬', 'l' => '𝘭', 'm' => '𝘮', 'n' => '𝘯', 'o' => '𝘰',
        'p' => '𝘱', 'q' => '𝘲', 'r' => '𝘳', 's' => '𝘴', 't' => '𝘵',
        'u' => '𝘶', 'v' => '𝘷', 'w' => '𝘸', 'x' => '𝘹', 'y' => '𝘺',
        'z' => '𝘻',
        _ => c,
    }).collect()
}

pub fn underline_text(content: &str) -> String {
    format!("\x1b[4m{}\x1b[24m", content)
}

pub fn highlight_bg_text(content: &str, term_width: u16) -> String {
    let padded = format!("{:<width$}", content, width = term_width as usize);
    format!("\x1b[48;2;50;50;150m{padded}\x1b[0m")
}
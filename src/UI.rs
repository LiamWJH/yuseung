pub fn return_UI(code_text: &str, term_width: u16) -> String {
    let br = "═".repeat(term_width as usize) + "\n";
    let title = box_UI("YUSEUNG", term_width, 1);
    let random_test_shit = "";

    format!(
"{title}
{code_text}"
    ).to_owned()
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
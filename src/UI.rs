pub fn return_UI(code_text: &str, term_width: u16) -> String {
    let br = "=".repeat(term_width as usize) + "\n";

    format!("YUSUENG\n{br}
    {code_text}").to_owned()
}
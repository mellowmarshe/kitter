pub fn escape_html(s: String) -> String {
    s.replace("<", "&lt;").replace(">", "&gt;")
}

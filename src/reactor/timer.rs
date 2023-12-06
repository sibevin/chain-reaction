pub fn format_timer_text(value: i32) -> String {
    format!("{:0>5}.{:0>2}", value / 100, value % 100)
}

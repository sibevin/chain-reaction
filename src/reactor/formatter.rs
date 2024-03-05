pub fn format_text(field: &str, value: u32) -> String {
    if field == "time" {
        format!("{:0>4}.{:0>2}", value / 100, value % 100)
    } else if field == "alpha_count" {
        format!("{:0>4}", value)
    } else if field == "score" {
        let value_str = format!("{:0>6}", value);
        let (first, second) = value_str.split_at(3);
        format!("{},{}", first, second)
    } else if field == "chain" {
        format!("{:0>4}", value)
    } else {
        format!("{}", value)
    }
}

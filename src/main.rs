use retry_policy_reference::retry_delay_ms;

fn main() {
    let attempts = std::env::args().skip(1).map(|value| {
        value.parse::<u32>().expect("attempt must be an unsigned 32-bit integer")
    });
    let results: Vec<String> = attempts.map(|attempt| retry_delay_ms(attempt).to_string()).collect();
    println!("[{}]", results.join(","));
}

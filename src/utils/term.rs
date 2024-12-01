pub fn print_start(label: &str) -> () {
    // Add some space before each run
    println!("\n\n⏳️ {0}...", label)
}

pub fn print_status(message: &str) -> () {
    println!("🔵 {0}...", message)
}

pub fn print_result(value: &str) -> () {
    println!("🟢 Result: {0}", value)
}
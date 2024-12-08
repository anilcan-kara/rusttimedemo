use chrono::prelude::*;

fn main() {
    // Get the current local time
    let local: DateTime<Local> = Local::now();

    // Format the date and time in a detailed manner
    // Example: "Current date and time: Monday, 2024-12-09 15:42:05"
    let formatted = local.format("Current date and time: %A, %Y-%m-%d %H:%M:%S").to_string();

    // Print to console
    println!("{}", formatted);
}

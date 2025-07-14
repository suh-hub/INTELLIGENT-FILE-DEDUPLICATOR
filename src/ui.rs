use colored::*;

/// Displays the HashLaser banner in a stylish format.
pub fn print_banner() {
    println!(
        "{}",
        r#"
██╗  ██╗ █████╗ ███████╗██╗  ██╗██╗      █████╗ ███████╗███████╗██████╗
██║  ██║██╔══██╗██╔════╝██║  ██║██║     ██╔══██╗██╔════╝██╔════╝██╔══██╗
███████║███████║███████╗███████║██║     ███████║███████╗█████╗  ██████╔╝
██╔══██║██╔══██║╚════██║██╔══██║██║     ██╔══██║╚════██║██╔══╝  ██╔══██╗
██║  ██║██║  ██║███████║██║  ██║███████╗██║  ██║███████║███████╗██║  ██║
╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚══════╝╚══════╝╚═╝  ╚═╝
"#
        .cyan()
    );

    println!(
        "{}",
        "🔒 Intelligent File Deduplicator CLI | v1.0.0\n".dimmed()
    );
}

/// Prints a success message in green.
pub fn print_success(message: &str) {
    println!("{} {}", "✅ ".green(), message.green());
}

/// Prints a warning message in yellow.
pub fn print_warning(message: &str) {
    println!("{} {}", "⚠️ ".yellow(), message.yellow());
}

/// Prints an error message in red.
pub fn print_error(message: &str) {
    eprintln!("{} {}", "❌ ".red(), message.red());
}

/// Prints an informational message in cyan.
pub fn print_info(message: &str) {
    println!("{} {}", "ℹ️ ".cyan(), message.cyan());
}

/// Prints a list of items with bullets.
pub fn print_list(items: &[String]) {
    for item in items {
        println!("{} {}", "•".bright_blue(), item);
    }
}

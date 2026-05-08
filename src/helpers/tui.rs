use console::{Emoji, style};
use indicatif::{ProgressBar, ProgressStyle};

pub fn show_loading(text: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈")
            .template("{spinner:.magenta} {msg}")
            .unwrap(),
    );
    pb.set_message(text.to_string());
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    return pb;
}

pub fn print_banner() {
    let term = console::Term::stdout();
    let _ = term.clear_screen();

    let gear = Emoji("⚙️ ", "");
    let spark = Emoji("✨ ", "");

    println!(
        "{}",
        style("┌──────────────────────────────────────────────────────────┐").magenta()
    );
    println!(
        "{} {} {} {}",
        style("│").magenta(),
        spark,
        style("CRUSTY AGENT v0.1.0").bold().cyan(),
        style("                       │").magenta()
    );
    println!(
        "{} {} {} {}",
        style("│").magenta(),
        gear,
        style("Connected via 9router").dim(),
        style("                     │").magenta()
    );
    println!(
        "{} {} {} {}",
        style("│").magenta(),
        Emoji("💎 ", ""),
        style("Model: Gemini-3.1-Pro").yellow().italic(),
        style("                     │").magenta()
    );
    println!(
        "{}",
        style("└──────────────────────────────────────────────────────────┘").magenta()
    );
    println!();
}

use console::{Emoji, style};
use dialoguer::{Select, theme::ColorfulTheme};
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

pub fn print_banner(
    model_name: &str,
    proxy_name: &str,
    proxy_platform: &str,
    host: &str,
    port: u64,
    is_proxy_online: bool,
) {
    let term = console::Term::stdout();
    let _ = term.clear_screen();

    let width = 50;
    let line = "─".repeat(width);

    println!("{}", style(format!("┌{}┐", line)).dim());

    println!(
        "{}  {}  {:<width$}  {}",
        style("│").dim(),
        style("●").cyan(),
        style("CRUSTY AGENT v0.1.0").bold(),
        style("│").dim()
    );

    println!(
        "{}  {:<width$}  {}",
        style("│").dim(),
        style("─".repeat(width)).dim(),
        style("│").dim()
    );

    let proxy_status = if is_proxy_online {
        style("running").green()
    } else {
        style("offline").red().blink()
    };

    println!(
        "{}  {:<width$}  {}",
        style("│").dim(),
        format!(
            "Proxy: {} (Platform: {} Status: {} | Address: {}:{})",
            proxy_name, proxy_platform, proxy_status, host, port
        ),
        style("│").dim()
    );
    println!(
        "{}  {:<width$}  {}",
        style("│").dim(),
        style(format!("Model: {}", model_name)).yellow(),
        style("│").dim()
    );

    println!("{}", style(format!("└{}┘", line)).dim());
    println!();
}

pub fn show_menu(options: Vec<&str>, ques: &str) -> Option<usize> {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt(ques)
        .items(&options)
        .default(0)
        .interact_opt()
        .unwrap()
}

use console::style;
use dialoguer::{Select, theme::ColorfulTheme};
use indicatif::{ProgressBar, ProgressStyle};

pub fn print_error(msg: &str) {
    eprintln!("{} {}", style("Error:").red().bold(), msg);
}

pub fn print_info(msg: &str) {
    println!("{} {}", style("Info:").blue().bold(), msg);
}

pub fn print_success(msg: &str) {
    println!("{} {}", style("Success:").green().bold(), msg);
}

pub fn print_warning(msg: &str) {
    println!("{} {}", style("Warning:").yellow().bold(), msg);
}

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

use console::{Alignment, pad_str};

pub fn print_banner(
    model_name: &str,
    proxy_name: &str,
    proxy_platform: &str,
    host: &str,
    port: u64,
    is_proxy_online: bool,
    session_id: &str,
) {
    let term = console::Term::stdout();
    let _ = term.clear_screen();

    let proxy_status = if is_proxy_online {
        style("running").green().to_string()
    } else {
        style("offline").red().blink().to_string()
    };

    let w = 54;
    let line = "─".repeat(w);

    println!("{}", style(format!("┌{}┐", line)).dim());

    let header = format!(
        " {} {}",
        style("●").cyan(),
        style("CRUSTY AGENT v0.1.0").bold()
    );
    println!(
        "{} {} {}",
        style("│").dim(),
        pad_str(&header, w, Alignment::Left, None),
        style("│").dim()
    );

    println!("{}", style(format!("├{}┤", line)).dim());

    let model_line = format!(" Model   : {}", style(model_name).yellow());
    println!(
        "{} {} {}",
        style("│").dim(),
        pad_str(&model_line, w, Alignment::Left, None),
        style("│").dim()
    );

    let proxy_line = format!(" Proxy   : {} ({})", proxy_name, proxy_platform);
    println!(
        "{} {} {}",
        style("│").dim(),
        pad_str(&proxy_line, w, Alignment::Left, None),
        style("│").dim()
    );

    let address_line = format!(" Address : {}:{}", host, port);
    println!(
        "{} {} {}",
        style("│").dim(),
        pad_str(&address_line, w, Alignment::Left, None),
        style("│").dim()
    );

    let status_line = format!(" Status  : {}", proxy_status);
    println!(
        "{} {} {}",
        style("│").dim(),
        pad_str(&status_line, w, Alignment::Left, None),
        style("│").dim()
    );

    let session_line = format!(" Session : {}", style(session_id).cyan());
    println!(
        "{} {} {}",
        style("│").dim(),
        pad_str(&session_line, w, Alignment::Left, None),
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

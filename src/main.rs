mod commands;
mod old_systemd;

use clap::Parser;
use is_root::is_root;


#[derive(Parser)]
enum Cli {
    Start(commands::start::Start),
    Status(commands::status::Status),
}

impl Cli {
    fn execute(&self, systemd: old_systemd::Systemd) {
        match self {
            Self::Start(start) => start.execute(systemd),
            Self::Status(status) => status.execute(systemd),
            _ => println!("No command provided"),
        }
    }
}


fn main() {
    let cli = Cli::parse();
    let user_mode = !is_root();
    let systemd = old_systemd::Systemd::new(user_mode);
    systemd.init();
    cli.execute(systemd);
}

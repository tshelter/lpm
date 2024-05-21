mod commands;
mod systemd;

use clap::Parser;
use clap::CommandFactory;
use is_root::is_root;


#[derive(Parser)]
enum Cli {
    #[clap(name = "run", about = "Run a command as a service")]
    Run(commands::run::Run),
    #[clap(name = "start", about = "Start a service")]
    Start(commands::start::Start),
    #[clap(name = "status", about = "Get the status of a service")]
    Status(commands::status::Status),
}

impl Cli {
    fn execute(&self, systemd: systemd::Systemd) {
        match self {
            Self::Run(run) => run.execute(systemd),
            Self::Start(start) => start.execute(systemd),
            Self::Status(status) => status.execute(systemd),
            _ => Cli::command().print_help().unwrap(),
        }
    }
}


fn main() {
    let cli = Cli::parse();
    let user_mode = !is_root();
    let systemd = systemd::Systemd::new(user_mode);
    systemd.init();
    cli.execute(systemd);
}

use clap::Parser;
use is_root::is_root;

mod commands;
mod systemd;

#[derive(Parser)]
enum Cli {
    #[clap(name = "run", about = "Run a command as a service")]
    Run(commands::run::Run),
    #[clap(name = "start", about = "Start a service")]
    Start(commands::start::Start),
    #[clap(name = "status", about = "Get the status of a service")]
    Status(commands::status::Status),
    #[clap(name = "stop", about = "Stop a service")]
    Stop(commands::stop::Stop),
    #[clap(name = "restart", about = "Restart a service")]
    Restart(commands::restart::Restart),
    #[clap(name = "enable", about = "Enable a service")]
    Enable(commands::enable::Enable),
    #[clap(name = "disable", about = "Disable a service")]
    Disable(commands::disable::Disable),
    #[clap(name = "reload", about = "Reload a service")]
    Reload(commands::reload::Reload),
    #[clap(name = "logs", about = "Display logs for a service", aliases = &["log"])]
    Logs(commands::logs::Logs),
    #[clap(name = "remove", about = "Remove a service", aliases = &["rm", "delete", "del"])]
    Remove(commands::remove::Remove),
    #[clap(name = "list", about = "List services", aliases = &["ls", "l"])]
    List(commands::list::List),
}

impl Cli {
    fn execute(&self, systemd: systemd::Systemd) {
        match self {
            Self::Run(run) => run.execute(systemd),
            Self::Start(start) => start.execute(systemd),
            Self::Status(status) => status.execute(systemd),
            Self::Stop(stop) => stop.execute(systemd),
            Self::Restart(restart) => restart.execute(systemd),
            Self::Enable(enable) => enable.execute(systemd),
            Self::Disable(disable) => disable.execute(systemd),
            Self::Reload(reload) => reload.execute(systemd),
            Self::Logs(logs) => logs.execute(systemd),
            Self::Remove(remove) => remove.execute(systemd),
            Self::List(list) => list.execute(systemd),
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

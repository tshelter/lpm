use clap::{Parser, Subcommand};

mod commands;
mod systemd;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    /// Run a command as a service
    Run(commands::run::Run),
    /// Start a service
    Start(commands::start::Start),
    /// Get the status of a service
    Status(commands::status::Status),
    /// Stop a service
    Stop(commands::stop::Stop),
    /// Restart a service
    Restart(commands::restart::Restart),
    /// Enable a service
    Enable(commands::enable::Enable),
    /// Disable a service
    Disable(commands::disable::Disable),
    /// Reload a service
    Reload(commands::reload::Reload),
    /// Display logs for a service
    Logs(commands::logs::Logs),
    /// Remove a service
    Remove(commands::remove::Remove),
    /// List services
    List(commands::list::List),
    /// Print the service file for a service
    Cat(commands::cat::Cat),
}

impl Cli {
    fn execute(&self, systemd: systemd::Systemd) {
        match &self.command {
            Commands::Run(run) => run.execute(systemd),
            Commands::Start(start) => start.execute(systemd),
            Commands::Status(status) => status.execute(systemd),
            Commands::Stop(stop) => stop.execute(systemd),
            Commands::Restart(restart) => restart.execute(systemd),
            Commands::Enable(enable) => enable.execute(systemd),
            Commands::Disable(disable) => disable.execute(systemd),
            Commands::Reload(reload) => reload.execute(systemd),
            Commands::Logs(logs) => logs.execute(systemd),
            Commands::Remove(remove) => remove.execute(systemd),
            Commands::List(list) => list.execute(systemd),
            Commands::Cat(cat) => cat.execute(systemd),
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let user_mode = users::get_current_uid() != 0;
    let systemd = systemd::Systemd::new(user_mode);
    systemd.init();
    cli.execute(systemd);
}

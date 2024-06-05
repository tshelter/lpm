use std::os::unix::prelude::CommandExt;

use crate::commands::get_service_name;

#[derive(clap::Parser)]
#[command(aliases = &["log"])]
pub struct Logs {
    /// Number of lines to display
    #[arg(short = 'n', long)]
    lines: Option<usize>,
    /// Follow the logs
    #[arg(short = 'f', long, default_value_t)]
    follow: bool,
    /// Show the extended logs
    #[arg(short = 'x', long, default_value_t)]
    catalog: bool,
    /// The name of the service to display logs for
    service: String,
}

impl Logs {
    pub fn execute(&self, systemd: crate::systemd::Systemd) {
        let service_name = get_service_name(&self.service);
        let mut args = vec![
            "--unit".to_string(),
            service_name,
            "--pager-end".to_string(),
        ];
        if let Some(lines) = self.lines {
            args.push("--lines".to_string());
            args.push(lines.to_string());
        }
        if self.follow {
            args.push("--follow".to_string());
        }
        if self.catalog {
            args.push("--catalog".to_string());
        }

        systemd.journalctl(args).exec();
    }
}

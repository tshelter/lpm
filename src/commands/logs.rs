use std::os::unix::prelude::CommandExt;

use crate::commands::get_service_name;

#[derive(clap::Parser)]
pub struct Logs {
    #[arg(index = 1, help = "The name of the service to display logs for")]
    service: String,
    #[arg(short = 'n', long, help = "Number of lines to display")]
    lines: Option<usize>,
    #[arg(short = 'f', long, help = "Follow the logs", default_value = "false")]
    follow: bool,
    #[arg(
        short = 'x',
        long,
        help = "Show the extended logs",
        default_value = "false"
    )]
    catalog: bool,
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

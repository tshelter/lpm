use std::os::unix::prelude::CommandExt;

use crate::commands::get_service_name;

#[derive(clap::Parser)]
pub struct Enable {
    #[arg(index = 1, help = "The name of the service to enable")]
    service: String,
}

impl Enable {
    pub fn execute(&self, systemd: crate::systemd::Systemd) {
        let service_name = get_service_name(&self.service);
        systemd.enable(&service_name).exec();
    }
}

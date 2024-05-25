use std::os::unix::prelude::CommandExt;

use crate::commands::get_service_name;

#[derive(clap::Parser)]
pub struct Reload {
    #[arg(index = 1, help = "The name of the service to reload")]
    service: String,
}

impl Reload {
    pub fn execute(&self, systemd: crate::systemd::Systemd) {
        let service_name = get_service_name(&self.service);
        systemd.reload(&service_name).exec();
    }
}

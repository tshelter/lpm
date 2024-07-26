use std::os::unix::prelude::CommandExt;

use crate::commands::get_service_name;

#[derive(clap::Parser)]
pub struct Restart {
    /// The name of the service to restart
    service: String,
}

impl Restart {
    pub fn execute(&self, systemd: crate::systemd::Systemd) {
        let service_name = get_service_name(&self.service);
        systemd.restart(&service_name).exec();
    }
}

use std::os::unix::prelude::CommandExt;

use crate::commands::get_service_name;

#[derive(clap::Parser)]
pub struct Disable {
    /// The name of the service to disable
    service: String,
}

impl Disable {
    pub fn execute(&self, systemd: crate::systemd::Systemd) {
        let service_name = get_service_name(&self.service);
        systemd.disable(&service_name).exec();
    }
}

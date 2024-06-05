use std::os::unix::prelude::CommandExt;

use crate::commands::get_service_name;

#[derive(clap::Parser)]
pub struct Cat {
    /// The name of the service to cat of
    service: String,
}

impl Cat {
    pub fn execute(&self, systemd: crate::systemd::Systemd) {
        let service_name = get_service_name(&self.service);
        systemd.cat(&service_name).exec();
    }
}

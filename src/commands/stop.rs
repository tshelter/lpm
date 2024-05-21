use std::os::unix::prelude::CommandExt;
use crate::commands::get_service_name;

#[derive(clap::Parser)]
pub struct Stop {
    #[arg(index = 1, help = "The name of the service to stop")]
    service: String,
}

impl Stop {
    pub fn execute(&self, systemd: crate::systemd::Systemd) {
        let service_name = get_service_name(&self.service);
        systemd.stop(&service_name).exec();
    }
}

use std::os::unix::prelude::CommandExt;

#[derive(clap::Parser)]
pub struct Status {
    #[arg(index = 1, help = "The name of the service to get the status of")]
    service: String,
}

impl Status {
    pub fn execute(&self, systemd: crate::systemd::Systemd) {
        let service_name = format!("lpm-{}", self.service);
        systemd.status(&service_name).exec();
    }
}

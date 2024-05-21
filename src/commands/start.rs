use std::os::unix::prelude::CommandExt;

#[derive(clap::Parser)]
pub struct Start {
    #[arg(index = 1, help = "The name of the service to start")]
    service: String,
}

impl Start {
    pub fn execute(&self, systemd: crate::systemd::Systemd) {
        let service_name = format!("lpm-{}", self.service);
        systemd.start(&service_name).exec();
    }
}

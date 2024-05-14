use clap::Parser;
use crate::old_systemd::Systemd;

#[derive(Parser)]
pub struct Start {
    service: String,
}

impl Start {
    pub fn execute(&self, systemd: Systemd) {
        let service_name = format!("lpm-{}", self.service);
        systemd.exec_start(&service_name);
    }
}

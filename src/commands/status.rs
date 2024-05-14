
#[derive(clap::Parser)]
pub struct Status {
    service: String,
}

impl Status {
    pub fn execute(&self, systemd: crate::old_systemd::Systemd) {
        let service_name = format!("lpm-{}", self.service);
        systemd.exec_status(&service_name);
    }
}

use crate::commands::get_service_name;

#[derive(clap::Parser)]
#[command(aliases = &["rm", "delete", "del"])]
pub struct Remove {
    /// The name of the service to remove
    service: String,
}

impl Remove {
    pub fn execute(&self, systemd: crate::systemd::Systemd) {
        let service_name = get_service_name(&self.service);
        systemd
            .disable(&service_name)
            .status()
            .expect("Failed to disable service");
        systemd
            .stop(&service_name)
            .status()
            .expect("Failed to stop service");
        systemd.uninstall_service(&service_name);
        systemd
            .daemon_reload()
            .status()
            .expect("Failed to stop service");
    }
}

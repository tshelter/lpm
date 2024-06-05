use crate::commands::get_service_name;

#[derive(clap::Parser)]
pub struct Run {
    /// The name of the service
    #[arg(short, long)]
    name: String,
    /// Copy the current environment to the service. Usually not required.
    #[arg(short = 'e', long, default_value_t)]
    copy_env: bool,
    /// A description of the service
    #[arg(short, long, default_value_t)]
    description: String,
    /// List of key=value pairs for the [Unit] section of the service file
    #[arg(short, long)]
    unit: Vec<String>,
    /// List of key=value pairs for the [Service] section of the service file
    #[arg(short, long)]
    service: Vec<String>,
    /// "List of key=value pairs for the [Install] section of the service file"
    #[arg(short, long)]
    install: Vec<String>,
    /// The command to run as a service. Wrap 'command in quotes' to pass arguments.
    command: String,
}

fn has_not_key(section: impl AsRef<[(String, String)]>, key: &str) -> bool {
    !section.as_ref().iter().any(|(k, _)| k == key)
}

impl Run {
    pub fn execute(&self, systemd: crate::systemd::Systemd) {
        let mut unit_unit = vec![("Description".to_string(), self.description.clone())];
        let mut unit_service = vec![(
            "ExecStart".to_string(),
            format!("/usr/bin/env {}", self.command),
        )];
        let mut unit_install = vec![];

        let inputs = [&self.unit, &self.service, &self.install];
        let mut outputs = [&mut unit_unit, &mut unit_service, &mut unit_install];

        for (input, output) in inputs.iter().zip(outputs.iter_mut()) {
            for item in *input {
                let parts: Vec<&str> = item.split('=').collect();
                output.push((parts[0].to_string(), parts[1].to_string()));
            }
        }

        if has_not_key(&unit_unit, "StartLimitIntervalSec") {
            unit_unit.push(("StartLimitIntervalSec".to_string(), "0".to_string()));
        }

        if has_not_key(&unit_service, "Restart") {
            unit_service.push(("Restart".to_string(), "always".to_string()));
        }
        if has_not_key(&unit_service, "RestartSec") {
            unit_service.push(("RestartSec".to_string(), "1".to_string()));
        }
        if has_not_key(&unit_service, "WantedBy") {
            unit_install.push(("WantedBy".to_string(), systemd.default_target.clone()));
        }

        if has_not_key(&unit_service, "WorkingDirectory") {
            unit_service.push((
                "WorkingDirectory".to_string(),
                std::env::current_dir()
                    .expect("Failed to get current directory")
                    .to_str()
                    .expect("Failed to convert current directory path to string")
                    .to_string(),
            ));
        }

        if self.copy_env {
            let env = std::env::vars().collect::<Vec<(String, String)>>();
            for (key, value) in env {
                unit_service.push(("Environment".to_string(), format!("{}='{}'", key, value)));
            }
        }

        let unit = crate::systemd::Unit {
            unit: unit_unit,
            service: unit_service,
            install: unit_install,
        };

        let service_name = get_service_name(&self.name);
        systemd.install_service(&service_name, &unit);
        systemd
            .daemon_reload()
            .status()
            .expect("Failed to reload systemd");
        systemd
            .enable(&service_name)
            .status()
            .expect("Failed to enable service");
        systemd
            .restart(&service_name)
            .status()
            .expect("Failed to restart service");
    }
}

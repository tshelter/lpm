use crate::commands::get_service_name;

#[derive(clap::Parser)]
pub struct Run {
    #[arg(
        index = 1,
        help = "The command to run as a service. Wrap 'command in quotes' to pass arguments."
    )]
    command: String,
    #[arg(short, long, help = "The name of the service")]
    name: String,
    #[arg(
        short = 'E',
        long,
        default_value = "false",
        help = "Inherit the current environment to the service. Usually not required."
    )]
    inherit_env: bool,
    #[arg(
        short = 'e',
        long,
        help = "List of key=value pairs to pass to the service as environment variables"
    )]
    env: Vec<String>,

    #[arg(short, long, default_value = "", help = "A description of the service")]
    description: String,
    #[arg(
        short,
        long,
        help = "List of key=value pairs for the [Unit] section of the service file"
    )]
    unit: Vec<String>,
    #[arg(
        short,
        long,
        help = "List of key=value pairs for the [Service] section of the service file"
    )]
    service: Vec<String>,
    #[arg(
        short,
        long,
        help = "List of key=value pairs for the [Install] section of the service file"
    )]
    install: Vec<String>,
}

#[inline(always)]
fn has_not_key(section: &Vec<(String, String)>, key: &str) -> bool {
    !section.iter().any(|(k, _)| k == key)
}

impl Run {
    pub fn execute(&self, systemd: crate::systemd::Systemd) {
        let mut unit_unit = vec![];
        let mut unit_service = vec![];
        let mut unit_install = vec![];

        let inputs = [&self.unit, &self.service, &self.install];
        let mut outputs = [&mut unit_unit, &mut unit_service, &mut unit_install];

        for (input, output) in inputs.iter().zip(outputs.iter_mut()) {
            for item in *input {
                let parts: Vec<&str> = item.split('=').collect();
                output.push((parts[0].to_string(), parts[1].to_string()));
            }
        }

        if has_not_key(&unit_unit, "Description") {
            unit_unit.push(("Description".to_string(), self.description.clone()));
        }
        if has_not_key(&unit_service, "ExecStart") {
            unit_service.push(("ExecStart".to_string(), format!("/usr/bin/env {}", self.command)));
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

        if self.inherit_env {
            let env = std::env::vars().collect::<Vec<(String, String)>>();
            for (key, value) in env {
                unit_service.push(("Environment".to_string(), format!("{}='{}'", key, value)));
            }
        }

        for item in &self.env {
            unit_service.push(("Environment".to_string(), item.clone()));
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

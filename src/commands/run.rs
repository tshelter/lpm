use std::{collections::HashMap, mem::take};

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
    /// List of key=value pairs for the [Install] section of the service file
    #[arg(short, long)]
    install: Vec<String>,
    /// The command to run as a service. Wrap 'command in quotes' to pass arguments.
    command: String,
}

impl Run {
    pub fn execute(&self, mut systemd: crate::systemd::Systemd) {
        let mut unit_unit = HashMap::from([("Description".to_string(), self.description.clone())]);
        let mut unit_service = HashMap::from([(
            "ExecStart".to_string(),
            format!("/usr/bin/env {}", self.command),
        )]);
        let mut unit_install = HashMap::new();

        let inputs = [&self.unit, &self.service, &self.install];
        let outputs = [&mut unit_unit, &mut unit_service, &mut unit_install];

        for (input, output) in inputs.into_iter().zip(outputs.into_iter()) {
            for item in input {
                let parts: Vec<&str> = item.split('=').collect();
                output.insert(parts[0].to_string(), parts[1].to_string());
            }
        }

        unit_service
            .entry("StartLimitIntervalSec".into())
            .or_insert("0".into());
        unit_service
            .entry("Restart".into())
            .or_insert("always".into());
        unit_service
            .entry("RestartSec".into())
            .or_insert("1".into());
        unit_service
            .entry("WantedBy".into())
            .or_insert_with(|| take(&mut systemd.default_target));
        unit_service
            .entry("WorkingDirectory".into())
            .or_insert_with(|| {
                std::env::current_dir()
                    .expect("Failed to get current directory")
                    .to_str()
                    .expect("Failed to convert current directory path to string")
                    .to_string()
            });

        if self.copy_env {
            for (key, value) in std::env::vars() {
                unit_service.insert("Environment".to_string(), format!("{}='{}'", key, value));
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

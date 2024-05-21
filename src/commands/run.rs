#[derive(clap::Parser)]
pub struct Run {
    #[arg(index = 1, help = "The command to run as a service. Wrap 'command in quotes' to pass arguments.")]
    command: String,
    #[arg(short, long, help = "The name of the service")]
    name: String,

    #[arg(short = 'e', long, default_value = "false", help = "Copy the current environment to the service. Usually not required.")]
    copy_env: bool,

    #[arg(short, long, default_value = "", help = "A description of the service")]
    description: String,
    #[arg(short, long, help = "List of key=value pairs for the [Unit] section of the service file")]
    unit: Vec<String>,
    #[arg(short, long, help = "List of key=value pairs for the [Service] section of the service file")]
    service: Vec<String>,
    #[arg(short, long, help = "List of key=value pairs for the [Install] section of the service file")]
    install: Vec<String>,
}

#[inline(always)]
fn has_not_key(section: &Vec<(String, String)>, key: &str) -> bool {
    !section.iter().any(|(k, _)| k == key)
}

impl Run {
    pub fn execute(&self, systemd: crate::systemd::Systemd) {

        let mut unit_unit = vec![("Description".to_string(), self.description.clone())];
        let mut unit_service =
            vec![("ExecStart".to_string(), format!("/usr/bin/env {}", self.command))];
        let mut unit_install = vec![];

        let inputs = [&self.unit, &self.service, &self.install];
        let mut outputs = [&mut unit_unit, &mut unit_service, &mut unit_install];

        for (input, output) in inputs.iter().zip(outputs.iter_mut()) {
            for item in *input {
                let parts: Vec<&str> = item.split('=').collect();
                output.push((parts[0].to_string(), parts[1].to_string()));
            }
        }

        if !unit_service.iter().any(|(key, _)| key == "Restart") {
            unit_service.push(("Restart".to_string(), "always".to_string()));
        }
        if !unit_service.iter().any(|(key, _)| key == "RestartSec") {
            unit_service.push(("RestartSec".to_string(), "1".to_string()));
        }
        if !unit_install.iter().any(|(key, _)| key == "WantedBy") {
            unit_install.push(("WantedBy".to_string(), systemd.default_target.clone()));
        }

        if !unit_service
            .iter()
            .any(|(key, _)| key == "WorkingDirectory")
        {
            unit_service.push((
                "WorkingDirectory".to_string(),
                std::env::current_dir()
                    .unwrap()
                    .to_str()
                    .unwrap()
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

        let service_name = format!("lpm-{}", self.name);
        systemd.install_service(&service_name, &unit);
        systemd.daemon_reload().spawn().expect("Failed to reload systemd");
        systemd.enable(&service_name).spawn().expect("Failed to enable service");
        systemd.restart(&service_name).spawn().expect("Failed to restart service");
        println!("Service {} was created. Run `lpm status {}` to check the status.", self.name, self.name);
    }
}

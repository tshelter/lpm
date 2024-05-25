use ini::Ini;
use std::env;
use std::fs;
use std::process::Command;
use tabled::Tabled;

#[derive(Tabled)]
pub struct Service {
    pub name: String,
    pub is_active: bool,
    pub is_enabled: bool,
    pub memory: String,
}

pub struct Unit {
    pub unit: Vec<(String, String)>,
    pub service: Vec<(String, String)>,
    pub install: Vec<(String, String)>,
}

pub struct Systemd {
    user_mode: bool,
    default_args: Vec<&'static str>,
    services_path: String,  // without trailing slash
    pub default_target: String,
}

const INITIALIZATION_ERROR: &str = r#"
Failed to initialize LPM. Please make sure that:
- Environment variables are set correctly: (XDG_RUNTIME_DIR)
- Default path for XDG_RUNTIME_DIR - /run/user/$UID exists
"#;

impl Systemd {
    pub fn new(user_mode: bool) -> Self {
        let mut default_args = vec![];
        if user_mode {
            default_args.push("--user");
        }

        let services_path = if user_mode {
            format!("{}/.config/systemd/user", env::var("HOME").unwrap())
        } else {
            String::from("/etc/systemd/system")
        };

        Systemd {
            user_mode,
            default_args,
            services_path,
            default_target: if user_mode {
                "default.target".to_string()
            } else {
                "multi-user.target".to_string()
            }
        }
    }

    #[inline(always)]
    pub fn init(&self) {
        // Creates a services directory if it doesn't exist
        fs::create_dir_all(&self.services_path).expect("Failed to create services directory");

        if self.user_mode {
            if env::var("XDG_RUNTIME_DIR").is_err() {
                let user_id = env::var("UID").unwrap();
                let xdg_runtime_dir = format!("/run/user/{}", user_id);

                if !fs::metadata(&xdg_runtime_dir).is_ok() {
                    fs::create_dir_all(&xdg_runtime_dir).expect(INITIALIZATION_ERROR);
                }

                env::set_var("XDG_RUNTIME_DIR", &xdg_runtime_dir);
            }

        }
    }

    #[inline(always)]
    fn systemctl(&self, args: Vec<&str>) -> Command {
        // TODO: change args to Vec<String>
        let mut command = Command::new("systemctl");
        for arg in &self.default_args {
            command.arg(arg);
        }
        for arg in args {
            command.arg(arg);
        }
        command
    }

    #[inline(always)]
    pub fn journalctl(&self, args: Vec<String>) -> Command {
        let mut command = Command::new("journalctl");
        for arg in &self.default_args {
            command.arg(arg);
        }
        for arg in args {
            command.arg(arg);
        }
        command
    }
}

impl Systemd {
    pub fn start(&self, service: &str) -> Command {
        self.systemctl(vec!["start", service])
    }

    pub fn stop(&self, service: &str) -> Command {
        self.systemctl(vec!["stop", service])
    }

    pub fn restart(&self, service: &str) -> Command {
        self.systemctl(vec!["restart", service])
    }

    pub fn reload(&self, service: &str) -> Command {
        self.systemctl(vec!["reload", service])
    }

    pub fn enable(&self, service: &str) -> Command {
        self.systemctl(vec!["enable", service])
    }

    pub fn disable(&self, service: &str) -> Command {
        self.systemctl(vec!["disable", service])
    }

    pub fn status(&self, service: &str) -> Command {
        self.systemctl(vec!["status", service])
    }

    pub fn daemon_reload(&self) -> Command {
        self.systemctl(vec!["daemon-reload"])
    }

    pub fn list_unit_files(&self, pattern: Option<&str>) -> Command {
        let mut args = vec!["list-unit-files"];
        if let Some(pattern) = pattern {
            args.push(pattern);
        }
        self.systemctl(args)
    }
}

impl Systemd {
    pub fn install_service(&self, service: &str, unit: &Unit) {
        let unit_path = format!("{}/{}.service", self.services_path, service);
        let mut ini = Ini::new();

        for (section_name, section_data) in [
            ("Unit", &unit.unit),
            ("Service", &unit.service),
            ("Install", &unit.install),
        ] {
            for (key, value) in section_data {
                ini.with_section(Some(section_name.to_string()))
                    .add(key, value);
            }
        }

        ini.write_to_file(unit_path)
            .expect("Failed to write unit file");
    }

    pub fn uninstall_service(&self, service: &str) {
        let unit_path = format!("{}/{}.service", self.services_path, service);
        fs::remove_file(unit_path).expect("Failed to remove unit file");
    }

    pub fn get_services(&self) -> Vec<Service> {
        let output = self.list_unit_files(Some("lpm-*.service")).output().unwrap();
        let output = String::from_utf8(output.stdout).unwrap();
        output.lines().filter(|line| line.contains("lpm-")).map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let service = parts[0].trim_end_matches(".service");
            let is_enabled = parts[1] == "enabled";
            let status = self.status(service).output().unwrap();
            let status = String::from_utf8(status.stdout).unwrap();
            let is_active = status.contains("Active: active");
            // or "" and include only second part
            let memory = status.lines().find(|line| line.contains("Memory:")).unwrap_or("").split_whitespace().last().unwrap_or("").to_string();
            // remove lpm- prefix from service name and .service suffix from service name
            let name = service.trim_start_matches("lpm-").trim_end_matches(".service").to_string();
            Service {
                name,
                is_active,
                is_enabled,
                memory,
            }
        }).collect::<Vec<Service>>()
    }
}

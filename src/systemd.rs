use std::env;
use std::fs;
use std::process::Command;

use ini::Ini;
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
    services_path: String, // without trailing slash
    pub default_target: String,
}

const INITIALIZATION_ERROR: &str = r#"
Failed to initialize LPM. Please make sure that:
- Environment variables are set correctly: (XDG_RUNTIME_DIR or DBUS_SESSION_BUS_ADDRESS)
- Default path for XDG_RUNTIME_DIR - /run/user/$UID exists
- Default path for dbus - $XDG_RUNTIME_DIR/bus exists
Ensure that linger is enabled for the user by running:
$ loginctl enable-linger $USER
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
            },
        }
    }

    #[inline(always)]
    pub fn init(&self) {
        // Creates a services directory if it doesn't exist
        fs::create_dir_all(&self.services_path).expect("Failed to create services directory");

        if self.user_mode {
            if env::var("DBUS_SESSION_BUS_ADDRESS").is_ok() {
                return;
            }
            if env::var("XDG_RUNTIME_DIR").is_err() {
                let user_id = users::get_current_uid();
                let xdg_runtime_dir = format!("/run/user/{}", user_id);
                fs::metadata(format!("{}/bus", xdg_runtime_dir)).expect(INITIALIZATION_ERROR);
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

    pub fn cat(&self, service: &str) -> Command {
        self.systemctl(vec!["cat", service])
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
        fs::remove_file(unit_path).unwrap_or_else(|_| {
            println!("Failed to remove unit file");
        });
    }

    pub fn get_services(&self) -> Vec<Service> {
        let output = self
            .list_unit_files(Some("lpm-*.service"))
            .output()
            .expect("Failed to list unit files");
        let output = String::from_utf8(output.stdout).unwrap();
        output
            .lines()
            .filter(|line| line.contains("lpm-"))
            .map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                let service = parts[0].trim_end_matches(".service");
                let is_enabled = parts[1] == "enabled";
                let status = self
                    .status(service)
                    .output()
                    .expect("Failed to get status of service");
                let status =
                    String::from_utf8(status.stdout).expect("Failed to get status of service");
                let is_active = status.contains("Active: active");
                // or "" and include only second part
                let memory = status
                    .lines()
                    .find(|line| line.contains("Memory:"))
                    .unwrap_or("")
                    .split_whitespace()
                    .last()
                    .unwrap_or("")
                    .to_string();
                // remove lpm- prefix from service name and .service suffix from service name
                let name = trim_start_once(trim_end_once(service, ".service"), "lpm-").to_string();
                Service {
                    name,
                    is_active,
                    is_enabled,
                    memory,
                }
            })
            .collect::<Vec<Service>>()
    }
}

#[inline(always)]
fn trim_start_once<'a>(s: &'a str, prefix: &str) -> &'a str {
    s.strip_prefix(prefix).unwrap_or(s)
}

#[inline(always)]
fn trim_end_once<'a>(s: &'a str, suffix: &str) -> &'a str {
    s.strip_suffix(suffix).unwrap_or(s)
}

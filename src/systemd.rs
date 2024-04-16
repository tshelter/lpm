use ini::Ini;
use std::env;
use std::fs;
use std::os::unix::process::CommandExt;
use std::process::Command;

pub struct Unit {
    pub unit: Vec<(String, String)>,
    pub service: Vec<(String, String)>,
    pub install: Vec<(String, String)>,
}

pub struct Systemd {
    default_args: Vec<&'static str>,
    services_path: String,
}

impl Systemd {
    pub(crate) fn new(user_mode: bool) -> Self {
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
            default_args,
            services_path,
        }
    }

    pub fn init(&self) {
        // Creates a services directory if it doesn't exist
        fs::create_dir_all(&self.services_path).expect("Failed to create services directory");
    }

    fn run_systemctl(&self, args: Vec<&str>) -> std::process::Output {
        let mut command = Command::new("systemctl");
        for arg in &self.default_args {
            command.arg(arg);
        }
        for arg in args {
            command.arg(arg);
        }
        let output = command.output().expect("Failed to execute command");
        output
    }

    fn exec_systemctl(&self, args: Vec<&str>) {
        Command::new("systemctl")
            .args(&self.default_args)
            .args(args)
            .exec();
    }

    fn exec_command(&self, args: Vec<&str>, command: &str) {
        Command::new(command).args(args).exec();
    }

    pub fn install(&self, service: &str, unit: &Unit) {
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

    pub fn uninstall(&self, service: &str) {
        let unit_path = format!("{}/{}.service", self.services_path, service);
        fs::remove_file(unit_path).expect("Failed to remove unit file");
    }

    pub fn daemon_reload(&self) {
        self.run_systemctl(vec!["daemon-reload"]);
    }

    pub fn start(&self, service: &str) {
        self.run_systemctl(vec!["start", service]);
    }

    pub fn stop(&self, service: &str) {
        self.run_systemctl(vec!["stop", service]);
    }

    pub fn restart(&self, service: &str) {
        self.run_systemctl(vec!["restart", service]);
    }

    pub fn reload(&self, service: &str) {
        self.run_systemctl(vec!["reload", service]);
    }

    pub fn is_active(&self, service: &str) -> bool {
        let output = Command::new("systemctl")
            .args(&self.default_args)
            .args(&["is-active", service])
            .output()
            .expect("Failed to execute command");
        output.status.success()
    }

    pub fn enable(&self, service: &str) {
        self.run_systemctl(vec!["enable", service]);
    }

    pub fn disable(&self, service: &str) {
        self.run_systemctl(vec!["disable", service]);
    }

    pub fn is_enabled(&self, service: &str) -> bool {
        let output = Command::new("systemctl")
            .args(&self.default_args)
            .args(&["is-enabled", service])
            .output()
            .expect("Failed to execute command");
        output.status.success()
    }

    pub fn status(&self, service: &str) {
        self.exec_systemctl(vec!["status", service])
    }

    pub fn list_unit_files(&self, pattern: Option<&str>) -> std::process::Output {
        let mut args = vec!["list-unit-files"];
        if let Some(pattern) = pattern {
            args.push(pattern);
        }
        self.run_systemctl(args)
    }

    pub fn logs(&self, service: &str) {
        let mut args = self.default_args.clone();
        args.extend(vec!["--follow", "--full", "--unit", service]);
        self.exec_command(args, "journalctl")
    }
}

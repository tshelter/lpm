mod systemd;

use is_root::is_root;

use clap::{Parser};

#[derive(Parser)]
#[command(name = "lpm", version, about = "A CLI for managing processes")]
struct Cli {
    #[command(subcommand)]
    cmd: Option<Command>,
}

#[derive(Parser)]
enum Command {
    #[command(about = "Setup")]
    Setup,
    #[command(about = "Run a command")]
    Run {
        #[arg(index = 1)]
        command: String,
        #[arg(long, short = 'd', default_value = "")]
        description: String,
        #[arg(long, default_value = "true")]
        copy_env: bool,
        #[arg(long, short = 'u')]
        unit: Vec<String>,
        #[arg(long, short = 's')]
        service: Vec<String>,
        #[arg(long, short = 'i')]
        install: Vec<String>,
        #[arg(long, short = 'n')]
        name: String,
    },
    #[command(about = "Start a process")]
    Start {
        #[arg(index = 1)]
        name: String,
    },
    #[command(about = "Stop a process")]
    Stop {
        #[arg(index = 1)]
        name: String,
    },
    #[command(about = "Reload a process")]
    Reload {
        #[arg(index = 1)]
        name: String,
    },
    #[command(about = "Restart a process")]
    Restart {
        #[arg(index = 1)]
        name: String,
    },
    #[command(about = "Get status of a process")]
    Status {
        #[arg(index = 1)]
        name: String,
    },
    #[command(aliases = & ["l", "ls"], about = "List all processes")]
    List,
    #[command(aliases = & ["remove", "del", "rm"], about = "Delete a process")]
    Delete {
        #[arg(index = 1)]
        name: String,
    },
    #[command(about = "Set agent URL")]
    Agent {
        #[arg(index = 1)]
        url: String,
    },
    #[command(about = "Get logs of a process", aliases = & ["log"])]
    Logs {
        #[arg(index = 1)]
        name: String,
    },
}

fn main() {
    let user_mode = !is_root();
    let systemd = systemd::Systemd::new(user_mode);
    systemd.init();
    let cli = Cli::parse();

    match cli.cmd {
        Some(Command::Setup) => {
            // Handle setup command
            println!("Setup command executed");
        }
        Some(Command::Run { command, description, copy_env, unit, service, install, name }) => {
            // Handle run command
            println!("Run command executed: command={}, copy_env={}, unit={:?}, name={:?}", command, copy_env, unit, name);

            let mut unit_unit = vec![
                ("Description".to_string(), description),
            ];
            let mut unit_service = vec![
                ("ExecStart".to_string(), format!("/usr/bin/env {}", command)),
            ];
            let mut unit_install = vec![];

            let inputs = [&unit, &service, &install];
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

            if !unit_service.iter().any(|(key, _)| key == "WorkingDirectory") {
                unit_service.push(("WorkingDirectory".to_string(), std::env::current_dir().unwrap().to_str().unwrap().to_string()));
            }

            if copy_env {
                let mut env = std::env::vars().collect::<Vec<(String, String)>>();
                for (key, value) in env {
                    unit_service.push(("Environment".to_string(), format!("{}='{}'", key, value)));
                }
            }

            let unit = systemd::Unit {
                unit: unit_unit,
                service: unit_service,
                install: unit_install,
            };

            let service_name = format!("lpm-{}", name);
            systemd.install(&service_name, &unit);
            systemd.daemon_reload();
            systemd.restart(&service_name);
        }
        Some(Command::Start { name }) => {
            // Handle start command
            println!("Start command executed: name={}", name);
        }
        Some(Command::Stop { name }) => {
            // Handle stop command
            println!("Stop command executed: name={}", name);
        }
        Some(Command::Reload { name }) => {
            // Handle reload command
            println!("Reload command executed: name={}", name);
        }
        Some(Command::Restart { name }) => {
            // Handle restart command
            println!("Restart command executed: name={}", name);
        }
        Some(Command::Status { name }) => {
            // Handle status command
            println!("Status command executed: name={}", name);
        }
        Some(Command::List) => {
            // Handle list command
            println!("List command executed");
        }
        Some(Command::Delete { name }) => {
            // Handle delete command
            println!("Delete command executed: name={}", name);
        }
        Some(Command::Agent { url }) => {
            // Handle agent command
            println!("Agent command executed: url={}", url);
        }
        Some(Command::Logs { name }) => {
            // Handle log command
            println!("Log command executed: name={}", name);
            let service_name = format!("lpm-{}", name);
            systemd.logs(&service_name);
        }
        None => {
            // No command provided
            println!("No command provided");
        }
    }
}

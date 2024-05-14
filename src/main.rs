mod systemd;

use clap::Parser;
use is_root::is_root;
use std::process::exit;

#[derive(Parser)]
#[command(name = "lpm", version, about = "A CLI for managing processes")]
struct Cli {
    #[command(subcommand)]
    cmd: Option<Command>,
}

struct AgentDaemonArgs {
    url: String,
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
    AgentDaemon(AgentDaemonArgs),
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
        Some(Command::Run {
            command,
            description,
            copy_env,
            unit,
            service,
            install,
            name,
        }) => {
            // Handle run command
            println!(
                "Run command executed: command={}, copy_env={}, unit={:?}, name={:?}",
                command, copy_env, unit, name
            );

            let mut unit_unit = vec![("Description".to_string(), description)];
            let mut unit_service =
                vec![("ExecStart".to_string(), format!("/usr/bin/env {}", command))];
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

            if copy_env {
                let env = std::env::vars().collect::<Vec<(String, String)>>();
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
            let service_name = format!("lpm-{}", name);
            systemd.start(&service_name);
        }
        Some(Command::Stop { name }) => {
            let service_name = format!("lpm-{}", name);
            systemd.stop(&service_name);
        }
        Some(Command::Reload { name }) => {
            let service_name = format!("lpm-{}", name);
            systemd.reload(&service_name);
        }
        Some(Command::Restart { name }) => {
            let service_name = format!("lpm-{}", name);
            systemd.restart(&service_name);
        }
        Some(Command::Status { name }) => {
            let service_name = format!("lpm-{}", name);
            systemd.status(&service_name);
        }
        Some(Command::List) => {
            let output = systemd.list_unit_files(Some("lpm-*"));

            println!("Available units:");
            let stdout =
                String::from_utf8(output.stdout).expect("Failed to convert stdout to string");
            for line in stdout.lines().skip(1).take(stdout.lines().count() - 2) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() > 0 {
                    println!("{}", parts[0][4..parts[0].len() - 8].to_string());
                }
            }
        }
        Some(Command::Delete { name }) => {
            let service_name = format!("lpm-{}", name);
            systemd.uninstall(&service_name);
            systemd.daemon_reload();
        }
        Some(Command::Agent { url }) => {
            println!("Currently agent is not implemented");
            println!("Agent URL: {}", url);
            exit(1);
        }
        Some(Command::Logs { name }) => {
            let service_name = format!("lpm-{}", name);
            systemd.logs(&service_name);
        }
        Some(Command::AgentDaemon(args)) => {
            println!("Currently agent daemon is not implemented");
            println!("Agent URL: {}", args.url);
            exit(1);
        }
        None => {
            println!("No command provided, use --help for usage information");
        }
    }
}

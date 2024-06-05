use tabled::Table;

#[derive(clap::Parser)]
#[command(aliases = &["ls", "l"])]
pub struct List {
    /// Show list in raw format
    #[arg(short, long, default_value_t)]
    raw: bool,
}

impl List {
    pub fn execute(&self, systemd: crate::systemd::Systemd) {
        let services = systemd.get_services();

        if self.raw {
            for service in services {
                println!(
                    "{} {} {} {}",
                    service.name, service.is_active, service.is_enabled, service.memory,
                );
            }
        } else {
            println!("{}", Table::new(&services).to_string());
        }
    }
}

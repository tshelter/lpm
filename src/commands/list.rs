use tabled::Table;

#[derive(clap::Parser)]
pub struct List {}

impl List {
    pub fn execute(&self, systemd: crate::systemd::Systemd) {
        let services = systemd.get_services();
        println!("{}", Table::new(&services).to_string());
    }
}

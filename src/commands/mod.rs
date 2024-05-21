pub mod disable;
pub mod enable;
pub mod logs;
pub mod reload;
pub mod remove;
pub mod restart;
pub mod run;
pub mod start;
pub mod status;
pub mod stop;

pub fn get_service_name(name: &str) -> String {
    format!("lpm-{}", name)
}

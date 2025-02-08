use playasia::config::get_config;
use playasia::server::run;

fn main() -> std::io::Result<()> {
    let config = get_config().expect("Failed to read config");
    let address = config.application.connection_string();
    run(address)
}

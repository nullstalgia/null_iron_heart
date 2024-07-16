use config::{Config, ConfigError, File as ConfigFile};
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::Write;
use toml;

#[derive(Debug, Deserialize, Serialize)]
#[allow(unused)]
pub struct MiscSettings {
    // TODO log level
    write_bpm_to_file: bool,
    write_bpm_file_path: String,
    log_sessions_to_csv: bool,
    log_sessions_csv_path: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(unused)]
pub struct BLESettings {
    pub never_ask_to_save: bool,
    pub saved_address: String,
    pub saved_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(unused)]
pub struct OSCSettings {
    pub host_ip: String,
    pub target_ip: String,
    pub port: u16,
    pub pulse_length_ms: u16,
    pub only_positive_floathr: bool,
    pub address_prefix: String,
    pub param_hrm_connected: String,
    pub param_beat_toggle: String,
    pub param_beat_pulse: String,
    pub param_bpm_int: String,
    pub param_bpm_float: String,
    pub param_latest_rr_int: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(unused)]
pub struct Settings {
    pub osc: OSCSettings,
    pub ble: BLESettings,
    misc: MiscSettings,
}

const CONFIG_NAME: &str = "null_iron_heart.toml";

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let exe_path = env::current_exe().expect("Failed to get executable path");
        let config_path = exe_path
            .parent()
            .expect("Executable has no parent directory")
            .join(CONFIG_NAME);

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(ConfigFile::from(config_path).required(false))
            .set_default("osc.host_ip", "0.0.0.0")
            .unwrap()
            .set_default("osc.target_ip", "127.0.0.1")
            .unwrap()
            .set_default("osc.port", 9000)
            .unwrap()
            .set_default("osc.pulse_length_ms", 100)
            .unwrap()
            .set_default("osc.only_positive_floathr", false)
            .unwrap()
            .set_default("osc.address_prefix", "/avatar/parameters/")
            .unwrap()
            .set_default("osc.param_hrm_connected", "isHRConnected")
            .unwrap()
            .set_default("osc.param_beat_toggle", "HeartBeatToggle")
            .unwrap()
            .set_default("osc.param_beat_pulse", "isHRBeat")
            .unwrap()
            .set_default("osc.param_bpm_int", "HR")
            .unwrap()
            .set_default("osc.param_bpm_float", "floatHR")
            .unwrap()
            .set_default("osc.param_latest_rr_int", "RRInterval")
            .unwrap()
            .set_default("ble.never_ask_to_save", false)
            .unwrap()
            .set_default("ble.saved_address", "")
            .unwrap()
            .set_default("ble.saved_name", "")
            .unwrap()
            .set_default("misc.write_bpm_to_file", false)
            .unwrap()
            .set_default("misc.write_bpm_file_path", "bpm.txt")
            .unwrap()
            .set_default("misc.log_sessions_to_csv", false)
            .unwrap()
            .set_default("misc.log_sessions_csv_path", "session_logs")
            .unwrap()
            .build()?;

        // Now that we're done, let's access our configuration
        // println!("debug: {:?}", s.get_bool("debug"));
        // println!("database: {:?}", s.get::<String>("database.url"));

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
    pub fn save(&self) -> Result<(), std::io::Error> {
        let exe_path = env::current_exe().expect("Failed to get executable path");
        let config_path = exe_path
            .parent()
            .expect("Executable has no parent directory")
            .join(CONFIG_NAME);

        let toml_string = toml::to_string(self).expect("Failed to serialize config");

        let mut file = File::create(config_path).expect("Failed to create config file");
        file.write_all(toml_string.as_bytes())
            .expect("Failed to write to config file");

        Ok(())
    }
}

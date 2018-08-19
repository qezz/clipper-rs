use std;
use std::path::Path;
use std::fs::File;

use failure;
use clap;
use serde_json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub port: Option<String>,
    pub address: Option<String>,
    pub config_path: Option<String>,
    pub executable: Option<String>,
    pub flags: Option<String>,
    _logfile: Option<String>,
}

impl<'a> Config {
    fn empty() -> Config {
        Config {
            port: None,
            address: None,
            config_path: None,
            executable: None,
            flags: None,
            _logfile: None,
        }
    }

    pub fn from_file(path: &str) -> Result<Config, failure::Error> {
        let json_file_path = Path::new(path);
        let json_file = File::open(json_file_path)?;
        let deserialized: Config = serde_json::from_reader(json_file)?;
        Ok(deserialized)
    }

    pub fn from_args(matches: &'a clap::ArgMatches<'a>) -> Config {
        let mut cfg = Config::empty();

        if let Some(cfg_path) = matches.value_of("config") {
            match Config::from_file(cfg_path) {
                Ok(c) => { cfg = c },
                Err(e) => { warn!("Cannot read config from file: {:?}", e); },
            }
        }

        // Update config if command line options are provided
        Config {
            port: cfg.port.or(matches.value_of("port").map(From::from)),
            // TODO: Add IPv6 support
            address: cfg.address.or(matches.value_of("address").map(From::from)),
            // config_path has been already updated
            config_path: cfg.config_path,
            executable: cfg.executable.or(matches.value_of("executable").map(From::from)),
            flags: cfg.flags.or(matches.value_of("flags").map(From::from)),

            // TODO: support logfile
            _logfile: cfg._logfile.or(None),
        }
    }

    // TODO: Remove this when support for multiple listeners is landed
    pub fn has_addr(&self) -> bool {
        self.address.is_some()
    }

    pub fn addr(&self) -> std::net::SocketAddr {
        format!("{}:{}",
                self.address.clone().unwrap(),
                self.port.clone().unwrap()).parse().unwrap()
    }

}

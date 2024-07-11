use std::env;

use minigrep::Config;

fn main() {

    let args = env::args();
    
    let config: Config = match Config::build_config(args) {
        Ok(config) => config,
        Err(error) => panic!("{}", error),
    };

    if let Err(e) = minigrep::run(config) {
        panic!("{}", e);
    }

}
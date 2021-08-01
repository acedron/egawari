use crate::config;

/// Tests the creation and reading of the config file.
#[test]
fn config_file() {
    let mut conf = config::get_config().unwrap();
    let mut conf_copy = config::get_config().unwrap();
    conf.input.name = String::from("SynPS/2 Synaptics TouchPad");
    conf_copy.input.name = String::from("SynPS/2 Synaptics TouchPad");
    config::save_config(conf).unwrap();
    assert_eq!(config::get_config().unwrap().input.name, conf_copy.input.name);
}

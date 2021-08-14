use crate::config;

/// Tests the creation and reading of the config file.
#[test]
fn config_file() {
    let mut conf = config::get_config().unwrap();
    conf.input.name = String::from("SynPS/2 Synaptics TouchPad");
    config::save_config(&conf).unwrap();

    let input_name = &conf.input.name;
    assert_eq!(config::get_config().unwrap().input.name, *input_name);
}

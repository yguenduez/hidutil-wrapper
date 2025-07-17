/*
hidutil property --set '{"UserKeyMapping":[
  {"HIDKeyboardModifierMappingSrc":0x700000039,"HIDKeyboardModifierMappingDst":0x70000002B},
  {"HIDKeyboardModifierMappingSrc":0x70000002B,"HIDKeyboardModifierMappingDst":0x700000039}
]}'
 */

#[derive(serde::Deserialize, serde::Serialize)]
struct UserKeyMapping {
    #[serde(rename = "HIDKeyboardModifierMappingSrc")]
    hid_src: u64,
    #[serde(rename = "HIDKeyboardModifierMappingDst")]
    hid_dst: u64,
}

impl UserKeyMapping {
    pub fn new(src: u64, dst: u64) -> Self {
        Self {
            hid_src: src,
            hid_dst: dst,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Default)]
struct KeyMapping {
    #[serde(rename = "UserKeyMapping")]
    user_key_mapping: Vec<UserKeyMapping>,
}

impl From<Vec<UserKeyMapping>> for KeyMapping {
    fn from(value: Vec<UserKeyMapping>) -> Self {
        Self {
            user_key_mapping: value,
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        panic!("map or none as argument")
    }

    let key_mapping = match args[1].as_str() {
        "map" => {
            const TABBED_KEY: u64 = 0x700000039;
            const CAPS_LOCK_KEY: u64 = 0x70000002B;

            let mapping_1 = UserKeyMapping::new(TABBED_KEY, CAPS_LOCK_KEY);
            let mapping_2 = UserKeyMapping::new(CAPS_LOCK_KEY, TABBED_KEY);
            vec![mapping_1, mapping_2].into()
        }
        _ => KeyMapping::default(),
    };

    std::process::Command::new("hidutil")
        .arg("property")
        .arg("--set")
        .arg(serde_json::to_string(&key_mapping).unwrap())
        .output()
        .unwrap();
}

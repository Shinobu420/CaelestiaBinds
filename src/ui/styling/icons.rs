use std::collections::HashMap;
use std::sync::OnceLock;

pub fn get_icon(key: &str) -> String {
    fn icon_table() -> &'static HashMap<&'static str, &'static str> {
        static TABLE: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();

        TABLE.get_or_init(|| {
            let mut m: HashMap<&str, &str> = HashMap::new();
            m.insert("super", "");
            m.insert("shift", " 󰘶 ");
            m.insert("return", "󰌑");
            m.insert("enter", "󰌑");
            m.insert("semicolon", ";");
            m.insert("delete", "DEL");
            m.insert("tab", "TAB");
            m.insert("left", "󰜱");
            m.insert("right", "󰜴");
            m.insert("up", "󰜷");
            m.insert("down", "󰜮");
            m.insert("mouse_down", "󱕐");
            m.insert("mouse_up", "󱕑");
            m.insert("mouse:272", "󰍽");
            m.insert("mouse:273", "󰍽");
            m.insert("xf86audioraisevolume", "");
            m.insert("xf86audiolowervolume", "");
            m.insert("xf86audiomute", "");
            m.insert("xf86audiomicmute", "󰍭");
            m.insert("xf86monbrightnessup", "󰃠");
            m.insert("xf86monbrightnessdown", "󰃞");
            m.insert("xf86audionext", "󰙡");
            m.insert("xf86audiopause", "");
            m.insert("xf86audioplay", "");
            m.insert("xf86audioprev", "󰙣");
            m
        })
    }

    let key_lower: String = key.to_ascii_lowercase();

    if let Some(&icon) = icon_table().get(key_lower.as_str()) {
        return icon.to_string();
    }

    key.to_string()
}

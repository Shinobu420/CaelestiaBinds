#[cfg(test)]
mod config_roundtrip_tests {
    use crate::config::user::UserConfig;
    use crate::hyprland::SearchOptions;
    use crate::ui::types::{ColumnVisibility, Theme};
    use std::fs;
    use tempfile::TempDir;

    fn setup_temp_config(temp_dir: &TempDir) -> std::path::PathBuf {
        let config_path = temp_dir.path().join("caelestiabind_test.json");
        config_path
    }

    #[test]
    fn test_default_roundtrip() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = setup_temp_config(&temp_dir);

        let original = UserConfig::default();
        let json = serde_json::to_string_pretty(&original).unwrap();
        fs::write(&config_path, json).unwrap();

        let loaded_json = fs::read_to_string(&config_path).unwrap();
        let loaded: UserConfig = serde_json::from_str(&loaded_json).unwrap();

        assert_eq!(
            serde_json::to_string(&original).unwrap(),
            serde_json::to_string(&loaded).unwrap()
        );
    }

    #[test]
    fn test_custom_config_roundtrip() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = setup_temp_config(&temp_dir);

        let original = UserConfig {
            theme: Theme::Light,
            column_visibility: ColumnVisibility {
                keybind: false,
                command: true,
            },
            search_options: SearchOptions {
                keybind: true,
                command: true,
            },
            zen_mode: true,
            search_keybind: egui::Key::Backslash
        };

        let json = serde_json::to_string_pretty(&original).unwrap();
        fs::write(&config_path, json).unwrap();

        let loaded_json = fs::read_to_string(&config_path).unwrap();
        let loaded: UserConfig = serde_json::from_str(&loaded_json).unwrap();

        assert_eq!(
            serde_json::to_string(&original).unwrap(),
            serde_json::to_string(&loaded).unwrap()
        );
    }

    #[test]
    fn test_serialization_format_stability() {
        let config = UserConfig::default();
        let json = serde_json::to_string_pretty(&config).unwrap();

        assert!(json.contains("\"theme\""));
        assert!(json.contains("\"column_visibility\""));
        assert!(json.contains("\"search_options\""));
        assert!(json.contains("\"zen_mode\""));
    }

    #[test]
    fn test_default_values() {
        let config = UserConfig::default();
        assert!(matches!(config.theme, Theme::Dark));
        assert!(!config.zen_mode);
        assert!(config.column_visibility.keybind);
        assert!(!config.column_visibility.command);
    }

    #[test]
    fn test_partial_deserialization() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = setup_temp_config(&temp_dir);

        let partial_json = r#"{
            "theme": "Light",
            "column_visibility": {
                "keybind": true,
                "command": true
            },
            "search_options": {
                "keybind": true,
                "command": true
            },
            "zen_mode": false
        }"#;

        fs::write(&config_path, partial_json).unwrap();
        let loaded_json = fs::read_to_string(&config_path).unwrap();
        let loaded: Result<UserConfig, _> = serde_json::from_str(&loaded_json);

        assert!(loaded.is_ok());
    }
}

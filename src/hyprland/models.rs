use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchOptions {
    pub keybind: bool,
    pub command: bool,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            keybind: true,
            command: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBindEntry {
    pub modifiers: String,
    pub key: String,
    pub command: String,
    pub description: String,
}

impl KeyBindEntry {
    pub const fn new(modifiers: String, key: String, command: String, description: String) -> Self {
        Self {
            modifiers,
            key,
            command,
            description,
        }
    }

    pub fn matches(&self, query: &str, options: &SearchOptions) -> bool {
        let query_lower = query.to_lowercase();
        let keybind_match = options.keybind
            && (self.modifiers.to_lowercase().contains(&query_lower)
                || self.key.to_lowercase().contains(&query_lower));
        let command_match = options.command && self.command.to_lowercase().contains(&query_lower);

        keybind_match || command_match
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBindings {
    pub entries: Vec<KeyBindEntry>,
}

impl KeyBindings {
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: KeyBindEntry) {
        self.entries.push(entry);
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self::new()
    }
}

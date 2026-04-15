use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum SortColumn {
    Keybind,
    Command,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum SortState {
    Ascending,
    Descending,
    None,
}

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum Theme {
    Dark,
    Light,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ColumnVisibility {
    pub keybind: bool,
    pub command: bool,
}

impl Default for ColumnVisibility {
    fn default() -> Self {
        Self {
            keybind: true,
            command: false,
        }
    }
}

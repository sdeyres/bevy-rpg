use std::{collections::HashMap, fmt};

use bevy::prelude::*;

use crate::config::pickup::DEFAULT_RADIUS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemKind {
    Plant1,
    Plant2,
    Plant3,
    Plant4,
}

impl ItemKind {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Plant1 => "Herb",
            Self::Plant2 => "Flower",
            Self::Plant3 => "Mushroom",
            Self::Plant4 => "Fern",
        }
    }
}

impl fmt::Display for ItemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.display_name())
    }
}

#[derive(Component, Debug)]
pub struct Pickable {
    pub kind: ItemKind,
    pub radius: f32,
}

impl Pickable {
    pub fn new(kind: ItemKind) -> Self {
        Self {
            kind,
            radius: DEFAULT_RADIUS,
        }
    }
}

#[derive(Resource, Default, Debug)]
pub struct Inventory {
    items: HashMap<ItemKind, u32>,
}

impl Inventory {
    pub fn add(&mut self, kind: ItemKind) -> u32 {
        let entry = self.items.entry(kind).or_insert(0);
        *entry += 1;
        *entry
    }

    pub fn summary(&self) -> String {
        if self.items.is_empty() {
            return "Empty".to_string();
        }

        let mut parts: Vec<String> = self
            .items
            .iter()
            .map(|(kind, count)| format!("{}: {}", kind, count))
            .collect();
        parts.sort();
        parts.join(", ")
    }
}

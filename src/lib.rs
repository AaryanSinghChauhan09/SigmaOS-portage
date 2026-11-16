// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-portage: portage component model.
//!
//! Real implementation: portage_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortageItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortageError {
    EmptyPortageItemName,
    DuplicatePortageItem(String),
    EmptyEventSubject,
}

impl fmt::Display for PortageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPortageItemName => write!(f, "portage_item name must not be empty"),
            Self::DuplicatePortageItem(id) => write!(f, "duplicate portage_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for PortageError {}

pub struct Portage {
    pub name: String,
    portage_items: BTreeMap<String, PortageItem>,
    events: BTreeMap<String, Event>,
}

impl Portage {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            portage_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_portage_item(&mut self, item: PortageItem) -> Result<(), PortageError> {
        if item.name.trim().is_empty() {
            return Err(PortageError::EmptyPortageItemName);
        }
        let id = item.id.clone();
        if self.portage_items.contains_key(&id) {
            return Err(PortageError::DuplicatePortageItem(id.clone()));
        }
        self.portage_items.insert(id, item);
        Ok(())
    }

    pub fn portage_item_count(&self) -> usize {
        self.portage_items.len()
    }

    pub fn get_portage_item(&self, id: &str) -> Option<&PortageItem> {
        self.portage_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), PortageError> {
        if event.subject.trim().is_empty() {
            return Err(PortageError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_portage_item(id: &str, name: &str) -> PortageItem {
        PortageItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Portage::new("Portage");
        assert_eq!(m.name, "Portage");
        assert_eq!(m.portage_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_portage_item() {
        let mut m = Portage::new("Portage");
        m.add_portage_item(sample_portage_item("a1", "Alpha")).unwrap();
        assert_eq!(m.portage_item_count(), 1);
        assert!(m.get_portage_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Portage::new("Portage");
        assert!(m.add_portage_item(sample_portage_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Portage::new("Portage");
        m.add_portage_item(sample_portage_item("x", "One")).unwrap();
        assert!(m.add_portage_item(sample_portage_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Portage::new("Portage");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}

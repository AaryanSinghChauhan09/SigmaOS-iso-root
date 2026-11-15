//! SigmaOS-iso-root: iso-root component model.

use std::collections::BTreeMap;
use std::fmt;

/// Primary domain entity for iso-root component.
#[derive(Debug, Clone, PartialEq)]
pub struct IsoRootItem {
    pub id: String,
    pub name: String,
    pub item_type: String,
    pub enabled: bool,
}

/// Secondary domain entity for iso-root component.
#[derive(Debug, Clone, PartialEq)]
pub struct IsoRootConfig {
    pub id: String,
    pub value: String,
    pub description: String,
}

/// Errors for the iso-root component domain.
#[derive(Debug, PartialEq)]
pub enum IsoRootError {
    EmptyName,
    Duplicate(String),
    NotFound(String),
}

impl fmt::Display for IsoRootError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "name cannot be empty"),
            Self::Duplicate(id) => write!(f, "duplicate id: {id}"),
            Self::NotFound(id) => write!(f, "not found: {id}"),
        }
    }
}

impl std::error::Error for IsoRootError {}

/// Registry and manager for iso-root component.
pub struct IsoRootManager {
    pub name: String,
    items: BTreeMap<String, IsoRootItem>,
    configs: BTreeMap<String, IsoRootConfig>,
}

impl IsoRootManager {
    /// Create a new manager instance.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            items: BTreeMap::new(),
            configs: BTreeMap::new(),
        }
    }

    /// Add a primary item with fail-fast validation.
    pub fn add_item(&mut self, item: IsoRootItem) -> Result<(), IsoRootError> {
        if item.name.is_empty() {
            return Err(IsoRootError::EmptyName);
        }
        if self.items.contains_key(&item.id) {
            return Err(IsoRootError::Duplicate(item.id.clone()));
        }
        self.items.insert(item.id.clone(), item);
        Ok(())
    }

    /// Retrieve a primary item by id.
    pub fn get_item(&self, id: &str) -> Option<&IsoRootItem> {
        self.items.get(id)
    }

    /// Count of primary items.
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// Add a secondary configuration entry.
    pub fn add_config(&mut self, cfg: IsoRootConfig) {
        self.configs.insert(cfg.id.clone(), cfg);
    }

    /// Count of configuration entries.
    pub fn config_count(&self) -> usize {
        self.configs.len()
    }

    /// Enable or disable an item.
    pub fn set_enabled(&mut self, id: &str, enabled: bool) -> Result<(), IsoRootError> {
        self.items
            .get_mut(id)
            .map(|item| item.enabled = enabled)
            .ok_or_else(|| IsoRootError::NotFound(id.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_iso_root_item(id: &str, name: &str) -> IsoRootItem {
        IsoRootItem {
            id: id.to_string(),
            name: name.to_string(),
            item_type: "iso-root-default".to_string(),
            enabled: true,
        }
    }

    fn make_iso_root_config(id: &str, val: &str) -> IsoRootConfig {
        IsoRootConfig {
            id: id.to_string(),
            value: val.to_string(),
            description: format!("{} config", id),
        }
    }

    #[test]
    fn test_create_aaa() {
        let mgr = IsoRootManager::new("test");
        assert_eq!(mgr.name, "test");
        assert_eq!(mgr.count(), 0);
        assert_eq!(mgr.config_count(), 0);
    }

    #[test]
    fn test_add_item_aaa() {
        let mut mgr = IsoRootManager::new("test");
        let item = make_iso_root_item("item1", "First Item");
        assert!(mgr.add_item(item).is_ok());
        assert_eq!(mgr.count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut mgr = IsoRootManager::new("test");
        let item = make_iso_root_item("item1", "");
        assert_eq!(mgr.add_item(item), Err(IsoRootError::EmptyName));
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut mgr = IsoRootManager::new("test");
        let item1 = make_iso_root_item("item1", "First");
        let item2 = make_iso_root_item("item1", "Duplicate");
        mgr.add_item(item1).unwrap();
        assert_eq!(mgr.add_item(item2), Err(IsoRootError::Duplicate("item1".to_string())));
    }

    #[test]
    fn test_config_addition_aaa() {
        let mut mgr = IsoRootManager::new("test");
        let cfg = make_iso_root_config("cfg1", "value1");
        mgr.add_config(cfg);
        assert_eq!(mgr.config_count(), 1);
    }

    #[test]
    fn test_get_item_aaa() {
        let mut mgr = IsoRootManager::new("test");
        let item = make_iso_root_item("item1", "First Item");
        mgr.add_item(item).unwrap();
        assert!(mgr.get_item("item1").is_some());
        assert!(mgr.get_item("none").is_none());
    }

    #[test]
    fn test_set_enabled_aaa() {
        let mut mgr = IsoRootManager::new("test");
        let item = make_iso_root_item("item1", "First Item");
        mgr.add_item(item).unwrap();
        assert!(mgr.set_enabled("item1", false).is_ok());
        assert!(!mgr.get_item("item1").unwrap().enabled);
        assert_eq!(mgr.set_enabled("none", true), Err(IsoRootError::NotFound("none".to_string())));
    }
}

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::core::errors::TriangleError;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TriangleAddress {
    path: Vec<u8>,
}

impl TriangleAddress {
    pub fn new(path: Vec<u8>) -> Self {
        Self { path }
    }

    pub fn root() -> Self {
        Self { path: vec![] }
    }

    pub fn append(&self, index: u8) -> Self {
        let mut new_path = self.path.clone();
        new_path.push(index);
        Self { path: new_path }
    }

    pub fn parent(&self) -> Option<Self> {
        if self.path.is_empty() {
            None
        } else {
            let mut parent_path = self.path.clone();
            parent_path.pop();
            Some(Self { path: parent_path })
        }
    }
}

impl fmt::Display for TriangleAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let path_str = self
            .path
            .iter()
            .map(|&num| num.to_string())
            .collect::<Vec<String>>()
            .join(".");
        write!(f, "{}", path_str)
    }
}

impl FromStr for TriangleAddress {
    type Err = TriangleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(TriangleAddress::root());
        }

        let path: Result<Vec<u8>, _> = s.split('.').map(|part| part.parse::<u8>()).collect();

        match path {
            Ok(path) => Ok(TriangleAddress::new(path)),
            Err(_) => Err(TriangleError::InvalidAddressFormat),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_creation_and_display() {
        let address = TriangleAddress::new(vec![0, 1, 2]);
        assert_eq!(address.to_string(), "0.1.2");
    }

    #[test]
    fn test_address_from_str() {
        let address_str = "0.1.2";
        let address = TriangleAddress::from_str(address_str).unwrap();
        assert_eq!(address, TriangleAddress::new(vec![0, 1, 2]));
    }

    #[test]
    fn test_append() {
        let address = TriangleAddress::root();
        let child_address = address.append(0).append(1);
        assert_eq!(child_address.to_string(), "0.1");
    }

    #[test]
    fn test_parent() {
        let address = TriangleAddress::from_str("0.1.2").unwrap();
        let parent = address.parent().unwrap();
        assert_eq!(parent.to_string(), "0.1");
        let root = parent.parent().unwrap().parent().unwrap();
        assert_eq!(root, TriangleAddress::root());
        assert!(root.parent().is_none());
    }
}

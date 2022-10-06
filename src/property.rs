use crate::symbol::{Symbol, SymbolProperty};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SelectOption<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<T>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MultiSelectOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub enum Property {
    Vertical {
        multi_select: Vec<MultiSelectOption>,
    },
    Horizontal {
        multi_select: Vec<MultiSelectOption>,
    },
    External {
        multi_select: Vec<MultiSelectOption>,
    },
    Version {
        select: SelectOption<String>,
    },
    Symbol {
        select: SelectOption<Symbol>,
    },
    Temporary {
        multi_select: Vec<MultiSelectOption>,
    },
}

#[derive(Deserialize, Debug, Clone)]
pub struct Properties {
    #[serde(rename = "Symbol")]
    pub symbol: SymbolProperty,
}

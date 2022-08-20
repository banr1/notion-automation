use serde::Serialize;

use crate::column::MultiSelectColumn;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum FilterKind {
    And(Vec<Filter>),
    Or(Vec<Filter>),
}

#[derive(Serialize)]
#[serde(tag = "property")]
#[allow(dead_code)]
pub enum Filter {
    Vertical {
        multi_select: MultiSelectFilter,
    },
    // Horizontal,
    // External,
    #[serde(rename = "Num of Vertical")]
    NumOfVertical {
        formula: FormulaFilter,
    },
    Temporary {
        multi_select: MultiSelectFilter,
    },
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum NumberFilter {
    Equals(i32),
    DoesNotEqual(i32),
    GreaterThan(i32),
    LessThan(i32),
    GreaterThanOrEqualTo(i32),
    LessThanOrEqualTo(i32),
    IsEmpty(bool),
    IsNotEmpty(bool),
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum MultiSelectFilter {
    Contains(MultiSelectColumn),
    DoesNotContain(MultiSelectColumn),
    IsEmpty(bool),
    IsNotEmpty(bool),
}

#[derive(Serialize)]
#[allow(dead_code)]
pub enum FormulaFilter {
    #[serde(rename = "string")]
    StringFilter,
    #[serde(rename = "number")]
    NumberFilter(NumberFilter),
    // CheckboxFilter,
    // DateFilter,
}

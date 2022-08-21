use serde::Serialize;

use crate::column::{External, Horizontal, Temporary, Version, Vertical};

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
        multi_select: MultiSelectFilter<Vertical>,
    },
    Horizontal {
        multi_select: MultiSelectFilter<Horizontal>,
    },
    External {
        multi_select: MultiSelectFilter<External>,
    },
    Version {
        select: SelectFilter<Version>,
    },
    #[serde(rename = "Num of Vertical")]
    NumOfVertical {
        formula: FormulaFilter,
    },
    Temporary {
        multi_select: MultiSelectFilter<Temporary>,
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
pub enum SelectFilter<T> {
    Equals(T),
    DoesNotEqual(T),
    IsEmpty(bool),
    IsNotEmpty(bool),
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub enum MultiSelectFilter<T> {
    Contains(T),
    DoesNotContain(T),
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

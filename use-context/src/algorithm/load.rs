use bincode::{Decode, Encode};
use get_size::GetSize;
use serde::{Deserialize, Serialize};

/// Тип назначения груза
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Decode, Encode, GetSize)]
pub enum AssignmentType {
    #[serde(alias = "ballast")]
    Ballast,
    #[serde(alias = "stores")]
    Stores,
    #[serde(alias = "cargo_load")]
    CargoLoad,
    #[serde(alias = "unspecified")]
    Unspecified,
}
//
impl std::fmt::Display for AssignmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AssignmentType::Ballast => "Ballast",
                AssignmentType::Stores => "Stores",
                AssignmentType::CargoLoad => "CargoLoad",
                AssignmentType::Unspecified => "Unspecified",
            },
        )
    }
}
/// Тип штучного груза судна
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, GetSize)]
pub enum UnitCargoType {
    #[serde(alias = "timber")]
    Timber,
    #[serde(alias = "container")]
    Container,
    #[serde(alias = "grain_bulkhead")]
    GrainBulkhead,
    #[serde(alias = "undefined")]
    Undefined,
}
//
impl std::fmt::Display for UnitCargoType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UnitCargoType::Timber => "Timber",
                UnitCargoType::Container => "Container",
                UnitCargoType::GrainBulkhead => "GrainBulkhead",
                UnitCargoType::Undefined => "Undefined",                
            },
        )
    }
}
//! OpenSCENARIO version handling

/// Supported OpenSCENARIO versions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenScenarioVersion {
    /// OpenSCENARIO 1.0
    V1_0,
    /// OpenSCENARIO 1.1
    V1_1,
    /// OpenSCENARIO 1.2
    V1_2,
}

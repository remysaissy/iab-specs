/// Common types shared between OpenRTB versions.
///
/// ## OpenRTB-Specific Types
///
/// Some objects are specific to the OpenRTB transaction protocol and are not part of AdCOM:
/// - `SupplyChain` / `SupplyChainNode`: Supply chain transparency objects
// OpenRTB-specific common types
mod supply_chain;
mod supply_chain_node;

pub use supply_chain::*;
pub use supply_chain_node::*;

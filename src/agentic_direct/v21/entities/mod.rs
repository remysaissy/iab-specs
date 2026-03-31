//! Agentic Direct v2.1 entities and data structures

pub mod account;
pub mod creative;
pub mod order;
pub mod order_state_machine;
pub mod organization;
pub mod product;

pub use account::{Account, AccountBuilder, AccountStatus};
pub use creative::{Creative, CreativeBuilder};
pub use order::{Order, OrderBuilder};
pub use order_state_machine::{
    can_transition_order, valid_order_transitions_from, OrderTransition, OrderTransitionBuilder,
    VALID_ORDER_TRANSITIONS,
};
pub use organization::{
    Address, AddressBuilder, Contact, ContactBuilder, Organization, OrganizationBuilder,
    OrganizationType,
};
pub use product::{Product, ProductBuilder};

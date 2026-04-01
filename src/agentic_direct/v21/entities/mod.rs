//! Agentic Direct v2.1 entities and data structures

pub mod account;
pub mod assignment;
pub mod creative;
pub mod line;
pub mod line_state_machine;
pub mod order;
pub mod order_state_machine;
pub mod organization;
pub mod product;

pub use account::{Account, AccountBuilder, AccountStatus};
pub use assignment::{Assignment, AssignmentBuilder};
pub use creative::{Creative, CreativeBuilder};
pub use line::{FrequencyCap, FrequencyCapBuilder, Line, LineBuilder};
pub use line_state_machine::{
    LineTransition, LineTransitionBuilder, VALID_LINE_TRANSITIONS, can_transition_line,
    valid_line_transitions_from,
};
pub use order::{Order, OrderBuilder};
pub use order_state_machine::{
    OrderTransition, OrderTransitionBuilder, VALID_ORDER_TRANSITIONS, can_transition_order,
    valid_order_transitions_from,
};
pub use organization::{
    Address, AddressBuilder, Contact, ContactBuilder, Organization, OrganizationBuilder,
    OrganizationType,
};
pub use product::{Product, ProductBuilder};

mod app_ads_txt_root;

pub use app_ads_txt_root::*;
// Re-export shared types from ads_txt
pub use crate::ads_txt::{AdsTxtSystem, SellerRelationType};

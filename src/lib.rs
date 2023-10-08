#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink::prelude::string::String;
use ink::prelude::vec::Vec;
use ink::primitives::AccountId;

pub mod contract_ref;
mod primitives;

pub use primitives::AccountIdOrDomain;

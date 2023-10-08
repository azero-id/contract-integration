use crate::contract_ref::{AznsRouter, AznsRouterRef};
use crate::{AccountId, String};

#[derive(scale::Encode, scale::Decode, PartialEq)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout, Debug)
)]
pub enum AccountIdOrDomain {
    AccountId(AccountId),
    Domain(String),
}

impl AccountIdOrDomain {
    pub fn get_address(&self, router_addr: AccountId) -> Option<AccountId> {
        match self {
            Self::AccountId(addr) => Some(*addr),
            Self::Domain(domain) => {
                let router: AznsRouterRef = router_addr.into();
                router.get_address(domain.clone()).ok()
            }
        }
    }
}

impl From<AccountId> for AccountIdOrDomain {
    fn from(value: AccountId) -> Self {
        Self::AccountId(value)
    }
}

impl From<String> for AccountIdOrDomain {
    fn from(value: String) -> Self {
        Self::Domain(value)
    }
}

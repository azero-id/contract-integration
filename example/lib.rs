#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod example {
    // 1. Import `AccountIdOrDomain` type
    use azns_integration::AccountIdOrDomain;

    #[ink(storage)]
    pub struct Example {
        // 2. Store AZERO-ID's router-contract address
        domain_router: AccountId,
    }

    impl Example {
        #[ink(constructor)]
        pub fn new(domain_router: AccountId) -> Self {
            Self { domain_router }
        }

        /// Returns the AccountId associated with the `user`
        #[ink(message)]
        pub fn simple_integration(&self, user: AccountIdOrDomain) -> Option<AccountId> {
            user.get_address(self.domain_router) // Resolves user to AccountId
        }
    }
}

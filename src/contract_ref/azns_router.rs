use crate::{AccountId, String, Vec};

pub type AznsRouterRef = ink::contract_ref!(AznsRouter, ink::env::DefaultEnvironment);

#[ink::trait_definition]
pub trait AznsRouter {
    #[ink(message, selector = 0xe6da7bf0)]
    fn get_all_registries(&self) -> Vec<AccountId>;

    #[ink(message, selector = 0x15a5d20a)]
    fn get_registry(&self, tld: String) -> Option<AccountId>;

    #[ink(message, selector = 0xd259f7ba)]
    fn get_address(&self, domain: String) -> Result<AccountId, u8>;

    #[ink(message, selector = 0xdf3a358e)]
    fn get_primary_domains(
        &self,
        account: AccountId,
        tld: Option<String>,
    ) -> Vec<(AccountId, String)>;
}

#![no_std]

multiversx_sc::imports!();

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait Empty: ContractBase {
    #[init]
    fn init(&self) {}

    #[endpoint(deployAnotherContract)]
    fn deploy_another_contract(
        &self,
        source_address: ManagedAddress<Self::Api>
    ) {
        self.send_raw()
            .deploy_from_source_contract(
                600_000_000,
                &BigUint::zero(),
                &source_address,
                CodeMetadata::PAYABLE_BY_SC | CodeMetadata::UPGRADEABLE,
                &ManagedArgBuffer::new()
            );
    }

    #[endpoint(upgradeAnotherContract)]
    fn upgrade_another_contract(
        &self,
        sc_address: ManagedAddress<Self::Api>,
        source_address: ManagedAddress<Self::Api>
    ) {
        self.send_raw()
            .upgrade_from_source_contract(
                &sc_address,
                600_000_000,
                &BigUint::zero(),
                &source_address,
                CodeMetadata::PAYABLE_BY_SC | CodeMetadata::UPGRADEABLE,
                &ManagedArgBuffer::new()
            );
    }

    #[proxy]
    fn contract_proxy(&self, address: ManagedAddress<Self::Api>) -> crate::Proxy<Self::Api>;
}

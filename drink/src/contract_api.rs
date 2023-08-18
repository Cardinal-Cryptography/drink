//! Contracts API.

use frame_support::{sp_runtime::AccountId32, weights::Weight};
use pallet_contracts::{CollectEvents, DebugInfo, Determinism};
use pallet_contracts_primitives::{
    Code, CodeUploadResult, ContractExecResult, ContractInstantiateResult,
};

use crate::{runtime::Runtime, EventRecordOf, Sandbox};

/// Interface for contract-related operations.
pub trait ContractApi<R: Runtime> {
    /// Interface for `bare_instantiate` contract call.
    #[allow(clippy::too_many_arguments)]
    fn deploy_contract(
        &mut self,
        contract_bytes: Vec<u8>,
        value: u128,
        data: Vec<u8>,
        salt: Vec<u8>,
        origin: AccountId32,
        gas_limit: Weight,
        storage_deposit_limit: Option<u128>,
    ) -> ContractInstantiateResult<AccountId32, u128, EventRecordOf<R>>;

    /// Interface for `bare_upload_code` contract call.
    fn upload_contract(
        &mut self,
        contract_bytes: Vec<u8>,
        origin: AccountId32,
        storage_deposit_limit: Option<u128>,
    ) -> CodeUploadResult<<R as frame_system::Config>::Hash, u128>;

    /// Interface for `bare_call` contract call.
    fn call_contract(
        &mut self,
        address: AccountId32,
        value: u128,
        data: Vec<u8>,
        origin: AccountId32,
        gas_limit: Weight,
        storage_deposit_limit: Option<u128>,
    ) -> ContractExecResult<u128, EventRecordOf<R>>;
}

impl<R: Runtime> ContractApi<R> for Sandbox<R> {
    fn deploy_contract(
        &mut self,
        contract_bytes: Vec<u8>,
        value: u128,
        data: Vec<u8>,
        salt: Vec<u8>,
        origin: AccountId32,
        gas_limit: Weight,
        storage_deposit_limit: Option<u128>,
    ) -> ContractInstantiateResult<AccountId32, u128, EventRecordOf<R>> {
        self.externalities.execute_with(|| {
            pallet_contracts::Pallet::<R>::bare_instantiate(
                origin,
                value,
                gas_limit,
                storage_deposit_limit,
                Code::Upload(contract_bytes),
                data,
                salt,
                DebugInfo::UnsafeDebug,
                CollectEvents::UnsafeCollect,
            )
        })
    }

    fn upload_contract(
        &mut self,
        contract_bytes: Vec<u8>,
        origin: AccountId32,
        storage_deposit_limit: Option<u128>,
    ) -> CodeUploadResult<<R as frame_system::Config>::Hash, u128> {
        self.externalities.execute_with(|| {
            pallet_contracts::Pallet::<R>::bare_upload_code(
                origin,
                contract_bytes,
                storage_deposit_limit,
                Determinism::Enforced,
            )
        })
    }

    fn call_contract(
        &mut self,
        address: AccountId32,
        value: u128,
        data: Vec<u8>,
        origin: AccountId32,
        gas_limit: Weight,
        storage_deposit_limit: Option<u128>,
    ) -> ContractExecResult<u128, EventRecordOf<R>> {
        self.externalities.execute_with(|| {
            pallet_contracts::Pallet::<R>::bare_call(
                origin,
                address,
                value,
                gas_limit,
                storage_deposit_limit,
                data,
                DebugInfo::UnsafeDebug,
                CollectEvents::UnsafeCollect,
                Determinism::Enforced,
            )
        })
    }
}

/// Converts bytes to a '\n'-split string.
pub fn decode_debug_buffer(buffer: &[u8]) -> Vec<String> {
    let decoded = buffer.iter().map(|b| *b as char).collect::<String>();
    decoded.split('\n').map(|s| s.to_string()).collect()
}

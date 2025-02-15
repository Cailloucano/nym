use crate::currency::{DecCoin, RegisteredCoins};
use crate::deprecated::DelegationEvent;
use crate::error::TypesError;
use crate::mixnode::MixNodeCostParams;
use cosmwasm_std::Decimal;
use mixnet_contract_common::{Delegation as MixnetContractDelegation, MixId};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "generate-ts", derive(ts_rs::TS))]
#[cfg_attr(
    feature = "generate-ts",
    ts(export_to = "ts-packages/types/src/types/rust/Delegation.ts")
)]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
pub struct Delegation {
    pub owner: String,
    pub mix_id: MixId,
    pub amount: DecCoin,
    pub height: u64,
    pub proxy: Option<String>, // proxy address used to delegate the funds on behalf of another address
}

impl Delegation {
    pub fn from_mixnet_contract(
        delegation: MixnetContractDelegation,
        reg: &RegisteredCoins,
    ) -> Result<Self, TypesError> {
        Ok(Delegation {
            owner: delegation.owner.to_string(),
            mix_id: delegation.mix_id,
            amount: reg.attempt_convert_to_display_dec_coin(delegation.amount.into())?,
            height: delegation.height,
            proxy: delegation.proxy.map(|d| d.to_string()),
        })
    }
}

#[cfg_attr(feature = "generate-ts", derive(ts_rs::TS))]
#[cfg_attr(
    feature = "generate-ts",
    ts(export_to = "ts-packages/types/src/types/rust/DelegationWithEverything.ts")
)]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, JsonSchema)]
pub struct DelegationWithEverything {
    pub owner: String,
    pub mix_id: MixId,
    pub node_identity: String,
    pub amount: DecCoin,
    pub accumulated_by_delegates: Option<DecCoin>,
    pub accumulated_by_operator: Option<DecCoin>,
    pub block_height: u64,
    pub delegated_on_iso_datetime: String,
    pub cost_params: Option<MixNodeCostParams>,
    pub avg_uptime_percent: Option<u8>,

    #[cfg_attr(feature = "generate-ts", ts(type = "string | null"))]
    pub stake_saturation: Option<Decimal>,

    pub uses_vesting_contract_tokens: bool,
    pub unclaimed_rewards: Option<DecCoin>,

    // DEPRECATED, IF POSSIBLE TRY TO DISCONTINUE USE OF IT!
    pub pending_events: Vec<DelegationEvent>,
}

#[cfg_attr(feature = "generate-ts", derive(ts_rs::TS))]
#[cfg_attr(
    feature = "generate-ts",
    ts(export_to = "ts-packages/types/src/types/rust/DelegationResult.ts")
)]
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Eq, Debug)]
pub struct DelegationResult {
    source_address: String,
    target_address: String,
    amount: Option<DecCoin>,
}

#[cfg_attr(feature = "generate-ts", derive(ts_rs::TS))]
#[cfg_attr(
    feature = "generate-ts",
    ts(export_to = "ts-packages/types/src/types/rust/DelegationSummaryResponse.ts")
)]
#[derive(Deserialize, Serialize)]
pub struct DelegationsSummaryResponse {
    pub delegations: Vec<DelegationWithEverything>,
    pub total_delegations: DecCoin,
    pub total_rewards: DecCoin,
}

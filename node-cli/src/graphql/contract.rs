use keys::b58encode_check;
use proto2::chain::transaction::Contract as ContractPb;

#[derive(juniper::GraphQLObject)]
pub struct TransferContract {
    owner_address: String,
    to_address: String,
    amount: f64,
}

#[derive(juniper::GraphQLObject)]
pub struct TransferAssetContract {
    owner_address: String,
    to_address: String,
    /// after ALLOW_SAME_TOKEN_NAME
    token_id: Option<i32>,
    /// before ALLOW_SAME_TOKEN_NAME
    token_name: Option<String>,
    amount: f64,
}

#[derive(juniper::GraphQLObject)]
pub struct Vote {
    vote_address: String,
    count: f64,
}

#[derive(juniper::GraphQLObject)]
pub struct WitnessCreateContract {
    owner_address: String,
    url: String,
}

#[derive(juniper::GraphQLObject)]
pub struct VoteWitnessContract {
    owner_address: String,
    votes: Vec<Vote>,
}

#[derive(juniper::GraphQLEnum, PartialEq, Eq)]
pub enum ResourceCode {
    Bandwidth,
    Energy,
}

#[derive(juniper::GraphQLObject)]
pub struct FreezeBalanceContract {
    owner_address: String,
    receiver_address: String,
    frozen_balance: f64,
    frozen_duration: i32,
    resource: ResourceCode,
}

#[derive(juniper::GraphQLObject)]
pub struct Parameter {
    key: i32,
    value: f64, // max: 9007199254740992
}

#[derive(juniper::GraphQLObject)]
pub struct ProposalCreateContract {
    owner_address: String,
    parameters: Vec<Parameter>,
}

#[derive(juniper::GraphQLObject)]
pub struct ProposalApproveContract {
    owner_address: String,
    proposal_id: i32,
    is_approve: bool,
}

#[derive(juniper::GraphQLObject)]
pub struct SmartContract {
    name: String,
    origin_address: String,
    contract_address: Option<String>,
    /// ABI as JSON string.
    abi: Option<String>,
    code: String,
    /// Percent, 0 to 100.
    user_resource_percent: i32,
    origin_energy_limit: f64,
    code_hash: Option<String>,
    // When smart contract is created by a trigger smart contract call.
    txn_id: Option<String>,
}

#[derive(juniper::GraphQLObject)]
pub struct CreateSmartContract {
    owner_address: String,
    new_smart_contract: SmartContract,
    call_value: f64, // moved from inner struct
    call_token_value: f64,
    call_token_id: i32,
}

#[derive(juniper::GraphQLObject)]
pub struct TriggerSmartContract {
    owner_address: String,
    contract_address: String,
    data: String,
    call_value: f64,
    call_token_value: f64,
    call_token_id: i32,
}

#[derive(juniper::GraphQLUnion)]
pub enum Contract {
    TransferContract(TransferContract),
    TransferAssetContract(TransferAssetContract),
    WitnessCreateContract(WitnessCreateContract),
    VoteWitnessContract(VoteWitnessContract),
    FreezeBalanceContract(FreezeBalanceContract),
    ProposalCreateContract(ProposalCreateContract),
    ProposalApproveContract(ProposalApproveContract),
    CreateSmartContract(CreateSmartContract),
    TriggerSmartContract(TriggerSmartContract),
    // AssetIssueContract(AssetIssueContract),
    // AccountCreateContract = 0,
    // VoteAssetContract = 3,
    //  = 6,
    // WitnessUpdateContract = 8,
    /*
    ParticipateAssetIssueContract = 9,
    AccountUpdateContract = 10, */
    /*UnfreezeBalanceContract = 12,
    WithdrawBalanceContract = 13,
    UnfreezeAssetContract = 14,
    UpdateAssetContract = 15,
    */
    /*
    ProposalDeleteContract = 18,
    SetAccountIdContract = 19,
    UpdateSettingContract = 33,
    ExchangeCreateContract = 41,
    ExchangeInjectContract = 42,
    ExchangeWithdrawContract = 43,
    ExchangeTransactionContract = 44,
    UpdateEnergyLimitContract = 45,
    AccountPermissionUpdateContract = 46,
    ClearABIContract = 48,
    UpdateBrokerageContract = 49,
    ShieldedTransferContract = 51,
    */
}

impl From<ContractPb> for Contract {
    fn from(pb: ContractPb) -> Self {
        use prost::Message;
        use proto2::chain::ContractType;
        use proto2::contract as contract_pb;

        let raw = &pb.parameter.as_ref().unwrap().value[..];

        match ContractType::from_i32(pb.r#type) {
            Some(ContractType::TransferContract) => {
                let cntr = contract_pb::TransferContract::decode(raw).unwrap();
                let inner = TransferContract {
                    owner_address: b58encode_check(&cntr.owner_address),
                    to_address: b58encode_check(&cntr.to_address),
                    amount: cntr.amount as _,
                };
                Contract::TransferContract(inner)
            }
            Some(ContractType::TransferAssetContract) => {
                let cntr = contract_pb::TransferAssetContract::decode(raw).unwrap();
                let inner = TransferAssetContract {
                    owner_address: b58encode_check(&cntr.owner_address),
                    to_address: b58encode_check(&cntr.to_address),
                    token_id: cntr.asset_name.parse().ok(),
                    token_name: Some(cntr.asset_name),
                    amount: cntr.amount as _,
                };
                Contract::TransferAssetContract(inner)
            }
            Some(ContractType::FreezeBalanceContract) => {
                let cntr = contract_pb::FreezeBalanceContract::decode(raw).unwrap();
                let inner = FreezeBalanceContract {
                    owner_address: b58encode_check(&cntr.owner_address),
                    receiver_address: b58encode_check(&cntr.receiver_address),
                    frozen_balance: cntr.frozen_balance as _,
                    frozen_duration: cntr.frozen_balance as _,
                    resource: if cntr.resource == 0 {
                        ResourceCode::Bandwidth
                    } else {
                        ResourceCode::Energy
                    },
                };
                Contract::FreezeBalanceContract(inner)
            }
            Some(ContractType::WitnessCreateContract) => {
                let cntr = contract_pb::WitnessCreateContract::decode(raw).unwrap();
                let inner = WitnessCreateContract {
                    owner_address: b58encode_check(&cntr.owner_address),
                    url: String::from_utf8(cntr.url).unwrap(),
                };
                Contract::WitnessCreateContract(inner)
            }
            Some(ContractType::VoteWitnessContract) => {
                let cntr = contract_pb::VoteWitnessContract::decode(raw).unwrap();
                let inner = VoteWitnessContract {
                    owner_address: b58encode_check(&cntr.owner_address),
                    votes: cntr
                        .votes
                        .iter()
                        .map(|vote| Vote {
                            vote_address: b58encode_check(&vote.vote_address),
                            count: vote.vote_count as _,
                        })
                        .collect(),
                };
                Contract::VoteWitnessContract(inner)
            }
            Some(ContractType::CreateSmartContract) => {
                let cntr = contract_pb::CreateSmartContract::decode(raw).unwrap();
                let smart_cntr = cntr.new_contract.as_ref().unwrap();

                let new_smart_contract = SmartContract {
                    origin_address: b58encode_check(&smart_cntr.origin_address),
                    name: smart_cntr.name.clone(),
                    abi: smart_cntr
                        .abi
                        .as_ref()
                        .map(|abi| &abi.entries)
                        .and_then(|entries| serde_json::to_string(entries).ok()),
                    code: hex::encode(&smart_cntr.bytecode),
                    user_resource_percent: smart_cntr.consume_user_resource_percent as _,
                    origin_energy_limit: smart_cntr.origin_energy_limit as _,
                    contract_address: if !smart_cntr.contract_address.is_empty() {
                        Some(b58encode_check(&smart_cntr.contract_address))
                    } else {
                        None
                    },
                    code_hash: if !smart_cntr.code_hash.is_empty() {
                        Some(hex::encode(&smart_cntr.code_hash))
                    } else {
                        None
                    },
                    txn_id: if !smart_cntr.txn_id.is_empty() {
                        Some(hex::encode(&smart_cntr.txn_id))
                    } else {
                        None
                    },
                };
                let inner = CreateSmartContract {
                    owner_address: b58encode_check(&cntr.owner_address),
                    new_smart_contract,
                    call_value: smart_cntr.call_value as _,
                    call_token_value: cntr.call_token_value as _,
                    call_token_id: cntr.call_token_id as _,
                };
                Contract::CreateSmartContract(inner)
            }
            Some(ContractType::TriggerSmartContract) => {
                let cntr = contract_pb::TriggerSmartContract::decode(raw).unwrap();
                let inner = TriggerSmartContract {
                    owner_address: b58encode_check(&cntr.owner_address),
                    contract_address: b58encode_check(&cntr.contract_address),
                    call_value: cntr.call_value as _,
                    data: hex::encode(&cntr.data),
                    call_token_value: cntr.call_token_value as _,
                    call_token_id: cntr.call_token_id as _,
                };
                Contract::TriggerSmartContract(inner)
            }
            Some(ContractType::ProposalCreateContract) => {
                let cntr = contract_pb::ProposalCreateContract::decode(raw).unwrap();
                let inner = ProposalCreateContract {
                    owner_address: b58encode_check(&cntr.owner_address),
                    parameters: cntr
                        .parameters
                        .iter()
                        .map(|(&k, &v)| Parameter {
                            key: k as _,
                            value: v as _,
                        })
                        .collect(),
                };
                Contract::ProposalCreateContract(inner)
            }
            Some(ContractType::ProposalApproveContract) => {
                let cntr = contract_pb::ProposalApproveContract::decode(raw).unwrap();
                let inner = ProposalApproveContract {
                    owner_address: b58encode_check(&cntr.owner_address),
                    proposal_id: cntr.proposal_id as _,
                    is_approve: cntr.is_approval,
                };
                Contract::ProposalApproveContract(inner)
            }
            Some(typ) => unimplemented!("unhandled type {:?}", typ),
            None => unreachable!(),
        }
    }
}

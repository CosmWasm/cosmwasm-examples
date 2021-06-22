# Ping Pong

Purpose of this repo is showing basic functionalities of CosmWasm smart contracts: State, Messages, Handler
Front facing dApp will be available soon.

## Logic

1. developer deploys smart contract and instantiates
2  Frontend: user creates, signs and broadcasts `ExecuteMsg::Ping`
3. Smart contract: keeps track of amount of ping requests in storage. Increments count and responds `pong` string
in `Data` field and in `event`.
4. Frontend: User queries total count of ping.

## Messages

```rust
// Instantiates a smart contract instance with 0 pong set
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // Pings contract, increases ping count
    Ping {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current ping count as a json-encoded number
    GetCount {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub ping_count: u32,
}
```

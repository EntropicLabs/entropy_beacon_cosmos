mod internal {
    use cosmwasm_std::{to_binary, Binary, CosmosMsg, StdResult, WasmMsg};
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    use crate::EntropyCallbackMsg;

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
    #[serde(rename_all = "snake_case")]
    pub enum ReceiverExecuteMsg {
        ReceiveEntropy(EntropyCallbackMsg),
    }

    impl EntropyCallbackMsg {
        pub fn into_binary(self) -> StdResult<Binary> {
            let msg = ReceiverExecuteMsg::ReceiveEntropy(self);
            to_binary(&msg)
        }
        pub fn into_cosmos_msg<T: Into<String>>(self, contract_addr: T) -> StdResult<CosmosMsg> {
            let msg = self.into_binary()?;
            let execute = WasmMsg::Execute {
                contract_addr: contract_addr.into(),
                msg,
                funds: vec![],
            };
            Ok(execute.into())
        }
    }
}

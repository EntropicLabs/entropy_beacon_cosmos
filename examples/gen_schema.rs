use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use entropy_beacon_cosmos::{
    msg::{ExecuteMsg, QueryMsg},
    provide::{
        KeyStatusQuery, KeyStatusResponse, LastEntropyQuery, LastEntropyResponse, SubmitEntropyMsg,
        WhitelistPublicKeyMsg,
    },
};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(WhitelistPublicKeyMsg), &out_dir);
    export_schema(&schema_for!(SubmitEntropyMsg), &out_dir);
    export_schema(&schema_for!(KeyStatusQuery), &out_dir);
    export_schema(&schema_for!(KeyStatusResponse), &out_dir);
    export_schema(&schema_for!(LastEntropyQuery), &out_dir);
    export_schema(&schema_for!(LastEntropyResponse), &out_dir);
}

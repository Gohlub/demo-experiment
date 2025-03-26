use serde::{Deserialize, Serialize};
use process_macros::SerdeJsonInto;

wit_bindgen::generate!({
    path: "target/wit",
    world: "types-app-framework-demo-uncentered-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

/// Generated caller utilities for RPC function stubs

pub use hyperware_app_common::SendResult;
pub use hyperware_app_common::send;
use hyperware_process_lib::Address;
use serde_json::json;

/// Generated RPC stubs for the client interface
pub mod client {
    use crate::*;

    /// Generated stub for `temp` remote RPC call
    pub async fn temp_remote_rpc(target: &Address) -> SendResult<f32> {
        let request = json!({"Temp" : {}});
        send::<f32>(&request, target, 30).await
    }
    
    
}

/// Generated RPC stubs for the curator interface
pub mod curator {
    use crate::*;

    /// Generated stub for `temp` remote RPC call
    pub async fn temp_remote_rpc(target: &Address) -> SendResult<f32> {
        let request = json!({"Temp" : {}});
        send::<f32>(&request, target, 30).await
    }
    
    
}

/// Generated RPC stubs for the indexer interface
pub mod indexer {
    use crate::*;

    /// Generated stub for `temp` remote RPC call
    pub async fn temp_remote_rpc(target: &Address) -> SendResult<f32> {
        let request = json!({"Temp" : {}});
        send::<f32>(&request, target, 30).await
    }
    
    
}


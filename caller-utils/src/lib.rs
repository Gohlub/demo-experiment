use serde::{Deserialize, Serialize};
use process_macros::SerdeJsonInto;
use std::collections::HashMap;

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

/// Generated RPC stubs for the indexer interface
pub mod indexer {
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

/// Generated RPC stubs for the client interface
pub mod client {
    use crate::*;

    /// Generated stub for `leet` remote RPC call
    pub async fn leet_remote_rpc(target: &Address, test: u32) -> SendResult<u32> {
        let request = json!({"Leet": test});
        send::<u32>(&request, target, 30).await
    }
    
    /// Generated stub for `leet` local RPC call
    pub async fn leet_local_rpc(target: &Address, test: u32) -> SendResult<u32> {
        let request = json!({"Leet": test});
        send::<u32>(&request, target, 30).await
    }
    
    /// Generated stub for `just-leet` remote RPC call
    pub async fn just_leet_remote_rpc(target: &Address) -> SendResult<u32> {
        let request = json!({"JustLeet" : {}});
        send::<u32>(&request, target, 30).await
    }
    
    
}


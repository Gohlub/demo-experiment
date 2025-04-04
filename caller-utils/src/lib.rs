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

// Import specific types from each interface
pub use crate::wit_custom::LoadBalancerStatus;
pub use crate::wit_custom::IndexerStat;
pub use crate::wit_custom::IndexResult;
pub use crate::wit_custom::ClientStats;
pub use crate::wit_custom::IndexerLoadInfo;
pub use crate::wit_custom::LoadBalancerStatus;
pub use crate::wit_custom::IndexerStat;
pub use crate::wit_custom::IndexResult;
pub use crate::wit_custom::ClientStats;
pub use crate::wit_custom::IndexerLoadInfo;

/// Generated RPC stubs for the client interface
pub mod client {
    use crate::*;

    /// Generated stub for `set-load-balancer` remote RPC call
    pub async fn set_load_balancer_remote_rpc(target: &Address, lb_address: String) -> SendResult<bool> {
        let request = json!({"SetLoadBalancer": lb_address});
        send::<bool>(&request, target, 30).await
    }
    
    /// Generated stub for `set-load-balancer` local RPC call
    pub async fn set_load_balancer_local_rpc(target: &Address, lb_address: String) -> SendResult<bool> {
        let request = json!({"SetLoadBalancer": lb_address});
        send::<bool>(&request, target, 30).await
    }
    
    /// Generated stub for `set-direct-indexer` remote RPC call
    pub async fn set_direct_indexer_remote_rpc(target: &Address, indexer_address: String) -> SendResult<bool> {
        let request = json!({"SetDirectIndexer": indexer_address});
        send::<bool>(&request, target, 30).await
    }
    
    /// Generated stub for `set-direct-indexer` local RPC call
    pub async fn set_direct_indexer_local_rpc(target: &Address, indexer_address: String) -> SendResult<bool> {
        let request = json!({"SetDirectIndexer": indexer_address});
        send::<bool>(&request, target, 30).await
    }
    
    /// Generated stub for `toggle-load-balancer-usage` remote RPC call
    pub async fn toggle_load_balancer_usage_remote_rpc(target: &Address) -> SendResult<bool> {
        let request = json!({"ToggleLoadBalancerUsage" : {}});
        send::<bool>(&request, target, 30).await
    }
    
    /// Generated stub for `toggle-load-balancer-usage` local RPC call
    pub async fn toggle_load_balancer_usage_local_rpc(target: &Address) -> SendResult<bool> {
        let request = json!({"ToggleLoadBalancerUsage" : {}});
        send::<bool>(&request, target, 30).await
    }
    
    /// Generated stub for `get-stats` remote RPC call
    pub async fn get_stats_remote_rpc(target: &Address) -> SendResult<ClientStats> {
        let request = json!({"GetStats" : {}});
        send::<ClientStats>(&request, target, 30).await
    }
    
    /// Generated stub for `get-stats` local RPC call
    pub async fn get_stats_local_rpc(target: &Address) -> SendResult<ClientStats> {
        let request = json!({"GetStats" : {}});
        send::<ClientStats>(&request, target, 30).await
    }
    
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
    
    /// Generated stub for `lb-add-to-index` remote RPC call
    pub async fn lb_add_to_index_remote_rpc(target: &Address, index_name: String, item: String) -> SendResult<IndexResult> {
        let request = json!({"LbAddToIndex": (index_name, item)});
        send::<IndexResult>(&request, target, 30).await
    }
    
    /// Generated stub for `lb-add-to-index` local RPC call
    pub async fn lb_add_to_index_local_rpc(target: &Address, index_name: String, item: String) -> SendResult<IndexResult> {
        let request = json!({"LbAddToIndex": (index_name, item)});
        send::<IndexResult>(&request, target, 30).await
    }
    
    /// Generated stub for `lb-remove-from-index` remote RPC call
    pub async fn lb_remove_from_index_remote_rpc(target: &Address, index_name: String, item: String) -> SendResult<IndexResult> {
        let request = json!({"LbRemoveFromIndex": (index_name, item)});
        send::<IndexResult>(&request, target, 30).await
    }
    
    /// Generated stub for `lb-remove-from-index` local RPC call
    pub async fn lb_remove_from_index_local_rpc(target: &Address, index_name: String, item: String) -> SendResult<IndexResult> {
        let request = json!({"LbRemoveFromIndex": (index_name, item)});
        send::<IndexResult>(&request, target, 30).await
    }
    
    /// Generated stub for `lb-get-index` remote RPC call
    pub async fn lb_get_index_remote_rpc(target: &Address, index_name: String) -> SendResult<IndexResult> {
        let request = json!({"LbGetIndex": index_name});
        send::<IndexResult>(&request, target, 30).await
    }
    
    /// Generated stub for `lb-get-index` local RPC call
    pub async fn lb_get_index_local_rpc(target: &Address, index_name: String) -> SendResult<IndexResult> {
        let request = json!({"LbGetIndex": index_name});
        send::<IndexResult>(&request, target, 30).await
    }
    
    /// Generated stub for `lb-list-indices` remote RPC call
    pub async fn lb_list_indices_remote_rpc(target: &Address) -> SendResult<IndexResult> {
        let request = json!({"LbListIndices" : {}});
        send::<IndexResult>(&request, target, 30).await
    }
    
    /// Generated stub for `lb-list-indices` local RPC call
    pub async fn lb_list_indices_local_rpc(target: &Address) -> SendResult<IndexResult> {
        let request = json!({"LbListIndices" : {}});
        send::<IndexResult>(&request, target, 30).await
    }
    
    
}

/// Generated RPC stubs for the load_balancer interface
pub mod load_balancer {
    use crate::*;

    /// Generated stub for `register-indexer` remote RPC call
    pub async fn register_indexer_remote_rpc(target: &Address, indexer_address: String, capacity: u32) -> SendResult<bool> {
        let request = json!({"RegisterIndexer": (indexer_address, capacity)});
        send::<bool>(&request, target, 30).await
    }
    
    /// Generated stub for `register-indexer` local RPC call
    pub async fn register_indexer_local_rpc(target: &Address, indexer_address: String, capacity: u32) -> SendResult<bool> {
        let request = json!({"RegisterIndexer": (indexer_address, capacity)});
        send::<bool>(&request, target, 30).await
    }
    
    /// Generated stub for `unregister-indexer` remote RPC call
    pub async fn unregister_indexer_remote_rpc(target: &Address, indexer_address: String) -> SendResult<bool> {
        let request = json!({"UnregisterIndexer": indexer_address});
        send::<bool>(&request, target, 30).await
    }
    
    /// Generated stub for `unregister-indexer` local RPC call
    pub async fn unregister_indexer_local_rpc(target: &Address, indexer_address: String) -> SendResult<bool> {
        let request = json!({"UnregisterIndexer": indexer_address});
        send::<bool>(&request, target, 30).await
    }
    
    /// Generated stub for `report-load` remote RPC call
    pub async fn report_load_remote_rpc(target: &Address, indexer_address: String, current_load: u32) -> SendResult<bool> {
        let request = json!({"ReportLoad": (indexer_address, current_load)});
        send::<bool>(&request, target, 30).await
    }
    
    /// Generated stub for `report-load` local RPC call
    pub async fn report_load_local_rpc(target: &Address, indexer_address: String, current_load: u32) -> SendResult<bool> {
        let request = json!({"ReportLoad": (indexer_address, current_load)});
        send::<bool>(&request, target, 30).await
    }
    
    /// Generated stub for `get-available-indexer` remote RPC call
    pub async fn get_available_indexer_remote_rpc(target: &Address) -> SendResult<Option<String>> {
        let request = json!({"GetAvailableIndexer" : {}});
        send::<Option<String>>(&request, target, 30).await
    }
    
    /// Generated stub for `get-available-indexer` local RPC call
    pub async fn get_available_indexer_local_rpc(target: &Address) -> SendResult<Option<String>> {
        let request = json!({"GetAvailableIndexer" : {}});
        send::<Option<String>>(&request, target, 30).await
    }
    
    /// Generated stub for `health-check` remote RPC call
    pub async fn health_check_remote_rpc(target: &Address) -> SendResult<bool> {
        let request = json!({"HealthCheck" : {}});
        send::<bool>(&request, target, 30).await
    }
    
    /// Generated stub for `health-check` local RPC call
    pub async fn health_check_local_rpc(target: &Address) -> SendResult<bool> {
        let request = json!({"HealthCheck" : {}});
        send::<bool>(&request, target, 30).await
    }
    
    /// Generated stub for `get-stats` remote RPC call
    pub async fn get_stats_remote_rpc(target: &Address) -> SendResult<Vec<(String, u32)>> {
        let request = json!({"GetStats" : {}});
        send::<Vec<(String, u32)>>(&request, target, 30).await
    }
    
    /// Generated stub for `get-stats` local RPC call
    pub async fn get_stats_local_rpc(target: &Address) -> SendResult<Vec<(String, u32)>> {
        let request = json!({"GetStats" : {}});
        send::<Vec<(String, u32)>>(&request, target, 30).await
    }
    
    /// Generated stub for `get-status` http RPC call
    pub async fn get_status_http_rpc(_target: &str) -> SendResult<LoadBalancerStatus> {
        // TODO: Implement HTTP endpoint
        SendResult::Success(LoadBalancerStatus::default())
    }
    
    
}

/// Generated RPC stubs for the indexer interface
pub mod indexer {
    use crate::*;

    /// Generated stub for `set-load-balancer` remote RPC call
    pub async fn set_load_balancer_remote_rpc(target: &Address, lb_address: String) -> SendResult<bool> {
        let request = json!({"SetLoadBalancer": lb_address});
        send::<bool>(&request, target, 30).await
    }
    
    /// Generated stub for `set-load-balancer` local RPC call
    pub async fn set_load_balancer_local_rpc(target: &Address, lb_address: String) -> SendResult<bool> {
        let request = json!({"SetLoadBalancer": lb_address});
        send::<bool>(&request, target, 30).await
    }
    
    /// Generated stub for `get-load-info` remote RPC call
    pub async fn get_load_info_remote_rpc(target: &Address) -> SendResult<IndexerLoadInfo> {
        let request = json!({"GetLoadInfo" : {}});
        send::<IndexerLoadInfo>(&request, target, 30).await
    }
    
    /// Generated stub for `get-load-info` local RPC call
    pub async fn get_load_info_local_rpc(target: &Address) -> SendResult<IndexerLoadInfo> {
        let request = json!({"GetLoadInfo" : {}});
        send::<IndexerLoadInfo>(&request, target, 30).await
    }
    
    /// Generated stub for `add-to-index` remote RPC call
    pub async fn add_to_index_remote_rpc(target: &Address, index_name: String, item: String) -> SendResult<bool> {
        let request = json!({"AddToIndex": (index_name, item)});
        send::<bool>(&request, target, 30).await
    }
    
    /// Generated stub for `remove-from-index` remote RPC call
    pub async fn remove_from_index_remote_rpc(target: &Address, index_name: String, item: String) -> SendResult<bool> {
        let request = json!({"RemoveFromIndex": (index_name, item)});
        send::<bool>(&request, target, 30).await
    }
    
    /// Generated stub for `get-index` remote RPC call
    pub async fn get_index_remote_rpc(target: &Address, index_name: String) -> SendResult<Option<Vec<String>>> {
        let request = json!({"GetIndex": index_name});
        send::<Option<Vec<String>>>(&request, target, 30).await
    }
    
    /// Generated stub for `list-indices` remote RPC call
    pub async fn list_indices_remote_rpc(target: &Address) -> SendResult<Vec<String>> {
        let request = json!({"ListIndices" : {}});
        send::<Vec<String>>(&request, target, 30).await
    }
    
    
}

/// Generated RPC stubs for the curator interface
pub mod curator {
    use crate::*;

    /// Generated stub for `add-curation` remote RPC call
    pub async fn add_curation_remote_rpc(target: &Address, title: String, content: String) -> SendResult<bool> {
        let request = json!({"AddCuration": (title, content)});
        send::<bool>(&request, target, 30).await
    }
    
    /// Generated stub for `remove-curation` remote RPC call
    pub async fn remove_curation_remote_rpc(target: &Address, title: String) -> SendResult<bool> {
        let request = json!({"RemoveCuration": title});
        send::<bool>(&request, target, 30).await
    }
    
    /// Generated stub for `get-curation` remote RPC call
    pub async fn get_curation_remote_rpc(target: &Address, title: String) -> SendResult<Option<String>> {
        let request = json!({"GetCuration": title});
        send::<Option<String>>(&request, target, 30).await
    }
    
    /// Generated stub for `temp` remote RPC call
    pub async fn temp_remote_rpc(target: &Address) -> SendResult<f32> {
        let request = json!({"Temp" : {}});
        send::<f32>(&request, target, 30).await
    }
    
    
}


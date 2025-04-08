use hyperprocess_macro::hyperprocess;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use rand::seq::SliceRandom;

// Data structures for the load balancer
#[derive(Clone, Debug, Serialize, Deserialize)]
struct IndexerInfo {
    address: String,
    capacity: u32,
    current_load: u32,
    last_heartbeat: u64,
    health_status: bool,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct LoadBalancerState {
    // Map of indexer address string to indexer info
    indexers: HashMap<String, IndexerInfo>,
    // Counts for statistics
    total_requests: u64,
    requests_since_startup: u64,
    startup_time: u64,
    last_rebalance_time: u64,
}

// Load balancer strategies
#[derive(Debug)]
#[allow(dead_code)] // Allow unused variants as they may be implemented in the future
enum DistributionStrategy {
    RoundRobin,
    LeastConnections,
    Random,
}

impl LoadBalancerState {
    fn new() -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Self {
            indexers: HashMap::new(),
            total_requests: 0,
            requests_since_startup: 0,
            startup_time: now,
            last_rebalance_time: now,
        }
    }
    
    // Get an indexer based on the strategy
    fn get_indexer(&self, strategy: DistributionStrategy) -> Option<String> {
        if self.indexers.is_empty() {
            return None;
        }
        
        match strategy {
            DistributionStrategy::RoundRobin => {
                // Simple implementation just gets a random one for now
                self.indexers.values()
                    .filter(|info| info.health_status)
                    .collect::<Vec<_>>()
                    .choose(&mut rand::thread_rng())
                    .map(|info| info.address.clone())
            },
            DistributionStrategy::LeastConnections => {
                // Find the indexer with the lowest load relative to capacity
                self.indexers.values()
                    .filter(|info| info.health_status)
                    .min_by_key(|info| (info.current_load as f32 / info.capacity as f32 * 100.0) as u32)
                    .map(|info| info.address.clone())
            },
            DistributionStrategy::Random => {
                // Choose a random healthy indexer
                self.indexers.values()
                    .filter(|info| info.health_status)
                    .collect::<Vec<_>>()
                    .choose(&mut rand::thread_rng())
                    .map(|info| info.address.clone())
            }
        }
    }
    
    // Check indexer health and update status
    #[allow(dead_code)] // This method may be used in the future
    fn check_indexer_health(&mut self) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        // Mark indexers as unhealthy if no heartbeat for 60 seconds
        for info in self.indexers.values_mut() {
            if now - info.last_heartbeat > 60 {
                info.health_status = false;
            }
        }
    }
    
    // Update load information for a specific indexer
    fn update_load(&mut self, indexer_address: &str, load: u32) {
        if let Some(info) = self.indexers.get_mut(indexer_address) {
            info.current_load = load;
            info.last_heartbeat = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            info.health_status = true;
        }
    }
}

#[hyperprocess(
    name = "Load Balancer",
    ui = Some(HttpBindingConfig::default()),
    endpoints = vec![
        Binding::Http {
            path: "/api/loadbalancer",
            config: HttpBindingConfig::new(false, false, false, None)
        }
    ],
    save_config = SaveOptions::EveryMessage,
    wit_world = "app-framework-demo-uncentered-dot-os-v0"
)]
impl LoadBalancerState {
    #[init]
    async fn initialize(&mut self) {
        println!("Load Balancer initialized");
        *self = LoadBalancerState::new();
        
        // Schedule periodic health checks
        let interval = Duration::from_secs(15);
        std::thread::spawn(move || {
            loop {
                println!("Running periodic health check");
                std::thread::sleep(interval);
            }
        });
    }
    
    // Register an indexer with the load balancer
    #[remote]
    #[local]
    fn register_indexer(&mut self, indexer_address: String, capacity: u32) -> bool {
        println!("Registering indexer: {} with capacity: {}", indexer_address, capacity);
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        let info = IndexerInfo {
            address: indexer_address.clone(),
            capacity,
            current_load: 0,
            last_heartbeat: now,
            health_status: true,
        };
        
        self.indexers.insert(indexer_address, info);
        true
    }
    
    // Unregister an indexer from the load balancer
    #[remote]
    #[local]
    fn unregister_indexer(&mut self, indexer_address: String) -> bool {
        println!("Unregistering indexer: {}", indexer_address);
        
        if self.indexers.remove(&indexer_address).is_some() {
            true
        } else {
            println!("Tried to unregister unknown indexer: {}", indexer_address);
            false
        }
    }
    
    // Report current load for an indexer
    #[remote]
    #[local]
    fn report_load(&mut self, indexer_address: String, current_load: u32) -> bool {
        if self.indexers.contains_key(&indexer_address) {
            self.update_load(&indexer_address, current_load);
            true
        } else {
            println!("Tried to update load for unknown indexer: {}", indexer_address);
            false
        }
    }
    
    // Get an available indexer from the pool based on load
    #[remote]
    #[local]
    fn get_available_indexer(&mut self) -> Option<String> {
        self.total_requests += 1;
        self.requests_since_startup += 1;
        
        // Use least connections strategy by default
        let indexer = self.get_indexer(DistributionStrategy::LeastConnections);
        
        if let Some(idx) = &indexer {
            println!("Selected indexer for request: {}", idx);
            // Optimistically update the load (will be corrected on next heartbeat)
            if let Some(info) = self.indexers.get_mut(idx) {
                info.current_load += 1;
            }
        } else {
            println!("No healthy indexers available");
        }
        
        indexer
    }
    
    // Health check for the load balancer itself
    #[remote]
    #[local]
    fn health_check(&mut self) -> bool {
        println!("Health check");
        true
    }
    
    // Get load balancer statistics
    #[remote]
    #[local]
    fn get_stats(&mut self) -> Vec<(String, u32)> {
        println!("Getting load balancer stats");
        
        self.indexers.values()
            .map(|info| (info.address.clone(), info.current_load))
            .collect()
    }
    
    // HTTP endpoint to view load balancer status
    #[http]
    fn get_status(&mut self) -> LoadBalancerStatus {
        let healthy_count = self.indexers.values()
            .filter(|info| info.health_status)
            .count();
            
        let total_count = self.indexers.len();
        let uptime_secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.startup_time;
            
        LoadBalancerStatus {
            healthy_indexers: healthy_count as u32,
            total_indexers: total_count as u32,
            total_requests: self.total_requests,
            uptime_seconds: uptime_secs,
            indexer_stats: self.indexers.values()
                .map(|info| IndexerStat {
                    address: info.address.clone(),
                    current_load: info.current_load,
                    capacity: info.capacity,
                    health_status: info.health_status,
                    load_percentage: if info.capacity > 0 {
                        (info.current_load as f32 / info.capacity as f32 * 100.0) as u32
                    } else {
                        0
                    },
                })
                .collect(),
        }
    }
}
// Status response for HTTP requests
#[derive(Serialize, Deserialize, Debug)]
struct LoadBalancerStatus {
    healthy_indexers: u32,
    total_indexers: u32,
    total_requests: u64,
    uptime_seconds: u64,
    indexer_stats: Vec<IndexerStat>,
}

#[derive(Serialize, Deserialize, Debug)]
struct IndexerStat {
    address: String,
    current_load: u32,
    capacity: u32,
    health_status: bool,
    load_percentage: u32,
}
# Load Balancer for P2P Applications

This component provides load balancing capabilities for the distributed P2P system, particularly to prevent clients from overloading indexers during high traffic.

## Features

- **Dynamic Indexer Registration**: Indexers can register/unregister themselves with the load balancer
- **Health Monitoring**: Periodic health checks ensure only healthy indexers receive traffic
- **Load-Based Distribution**: Routes requests to indexers with the lowest load relative to capacity
- **Multiple Distribution Strategies**:
  - Least Connections: Routes to indexers with the lowest load percentage
  - Round Robin: Distributes requests evenly (currently implemented as random)
  - Random: Randomly selects healthy indexers
- **Load Statistics**: Provides detailed metrics about indexer load and request distribution
- **Fault Tolerance**: Clients can fall back to direct indexer access if load balancer is unavailable

## Usage

### For Indexers

```rust
// Register with the load balancer
set_load_balancer(&load_balancer_address).await;

// The indexer automatically reports its load periodically
```

### For Clients

```rust
// Configure client to use load balancer
set_load_balancer(&load_balancer_address).await;

// Use load-balanced methods to access indexers
lb_add_to_index("my_index", "item").await;
lb_get_index("my_index").await;
lb_list_indices().await;
```

## Implementation Details

- The load balancer uses a configurable state persistence strategy (every 30 seconds by default)
- Indexers report their load metrics every 10 seconds
- Indexers are considered unhealthy if no heartbeat is received for 60 seconds
- The load balancer provides an HTTP API endpoint for status monitoring
- Each distribution strategy can be selected programmatically

## Future Improvements

- Configurable timeouts and intervals
- More sophisticated load balancing algorithms
- Geographic routing support
- Better failure detection and recovery
- Cluster support for load balancer high-availability
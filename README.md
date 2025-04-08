# P2P Application Suite with Load Balancing

A distributed peer-to-peer application suite featuring clients, indexers, curators, and a load balancer to manage high traffic loads.

## Components

- **Client**: User-facing component that makes requests to the indexers
- **Indexer**: Manages indices as key-value pairs, supporting operations to add, remove, and query items
- **Curator**: Maintains curated content with title-content mappings
- **Load Balancer**: Distributes client requests across multiple indexers based on load metrics

## How to Run

```bash
# After making changes to any component, regenerate the bindings
hyper-bindgen

# Build all components
kit b

# Run tests
kit run-tests

# Start with fake node
kit f
kit bs
```

## Key Features

- **Load Balancing**: Prevents indexers from being overloaded during high traffic
- **Health Monitoring**: Automatic health checks ensure only healthy indexers receive traffic
- **Fault Tolerance**: Clients can fall back to direct indexer access if load balancer is unavailable
- **Multiple Distribution Strategies**: Supports least connections, round robin, and random selection
- **Stateful Components**: All components maintain state that is persisted based on configurable strategies

## Architecture

The system uses a WebAssembly-based process framework with:
- RPC-style communication between components
- Async support through a custom runtime
- Serialized message passing
- Built-in state persistence

## Usage

See individual component READMEs for detailed usage:
- [Load Balancer](./load-balancer/README.md)

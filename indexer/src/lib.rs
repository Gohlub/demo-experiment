use hyperprocess_macro::hyperprocess;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Serialize, Deserialize)]
struct IndexerState {
    indices: HashMap<String, Vec<String>>, // Maps index name to list of items
}

#[hyperprocess(
    name = "Indexer",
    ui = Some(HttpBindingConfig::default()),
    endpoints = vec![
        Binding::Http {
            path: "/api",
            config: HttpBindingConfig::new(false, false, false, None),
        }, 
        Binding::Ws {
            path: "/ws",
            config: WsBindingConfig::new(false, false, false),
        }
    ],
    save_config = SaveOptions::EveryMessage,
    wit_world = "app-framework-demo-uncentered-dot-os-v0"
)]
impl IndexerState {
    #[init]
    async fn initialize(&mut self) {
        // Initialize with empty hashmap if not already present
        if self.indices.is_empty() {
            self.indices = HashMap::new();
        }
    }
    
    #[remote]
    async fn add_to_index(&mut self, index_name: String, item: String) -> bool {
        let items = self.indices.entry(index_name).or_insert_with(Vec::new);
        if !items.contains(&item) {
            items.push(item);
            true
        } else {
            false // Item already exists in the index
        }
    }

    #[remote]
    async fn remove_from_index(&mut self, index_name: String, item: String) -> bool {
        if let Some(items) = self.indices.get_mut(&index_name) {
            let original_len = items.len();
            items.retain(|i| i != &item);
            original_len != items.len() // Returns true if an item was removed
        } else {
            false // Index doesn't exist
        }
    }

    #[remote]
    async fn get_index(&self, index_name: String) -> Option<Vec<String>> {
        self.indices.get(&index_name).cloned()
    }

    #[remote]
    async fn list_indices(&self) -> Vec<String> {
        self.indices.keys().cloned().collect()
    }
}


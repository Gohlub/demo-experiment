use hyperprocess_macro::hyperprocess;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Serialize, Deserialize)]
struct CuratorState {
    curations: HashMap<String, String>, // Maps title to content
}

#[hyperprocess(
    name = "Curator",
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
impl CuratorState {
    #[init]
    async fn initialize(&mut self) {
        // Initialize with empty hashmap if not already present
        if self.curations.is_empty() {
            self.curations = HashMap::new();
        }
    }

    #[remote]
    async fn add_curation(&mut self, title: String, content: String) -> bool {
        self.curations.insert(title, content);
        true
    }

    #[remote]
    async fn remove_curation(&mut self, title: String) -> bool {
        self.curations.remove(&title).is_some()
    }

    #[remote]
    async fn get_curation(&self, title: String) -> Option<String> {
        self.curations.get(&title).cloned()
    }

    #[remote]
    async fn temp(&self) -> f32{
        1.0
    }
}


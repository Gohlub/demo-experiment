use hyperprocess_macro::hyperprocess;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
struct IndexerState {

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
    }
    
    #[remote]
    async fn temp(&self) -> f32{
        1.0
    }
}


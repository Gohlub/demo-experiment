use hyperprocess_macro::hyperprocess;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
struct ClientState {}

#[hyperprocess(
    name = "Client",
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
impl ClientState {
    #[init]
    async fn initialize(&mut self) {}

    #[remote]
    #[local]
    async fn leet(&self, test: u32) -> u32 {
        1337 * test
    }
}

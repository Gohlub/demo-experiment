use hyperprocess_macro::hyperprocess;
use hyperware::process::standard::ProcessId;
use hyperware_app_common::State;
use hyperware_process_lib::{Address, LazyLoadBlob, Request as HyperwareRequest};
use hyperware_process_lib::{http::server::WsMessageType};
use hyperware_app_common::send;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use hyperware_process_lib::kiprintln;
use hyperware::process::standard::Address as WitAddress;

#[derive(Default, Debug, Serialize, Deserialize)]
struct CuratorState {

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
    }

    #[remote]
    async fn temp(&self) -> f32{
        1.0
    }
}


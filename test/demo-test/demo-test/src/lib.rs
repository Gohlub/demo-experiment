use crate::hyperware::process::tester::{
    FailResponse, Request as TesterRequest, Response as TesterResponse, RunRequest,
};
use hyperware_process_lib::our;

use hyperware_process_lib::http::client::send_request_await_response;
use hyperware_process_lib::http::Method;
use hyperware_process_lib::logging::*;
use hyperware_process_lib::{
    await_message, call_init, print_to_terminal, println, Address, ProcessId, Request, Response,
};

use hyperware_process_lib::logging::{init_logging, Level};
use hyperware_process_lib::vfs::{create_drive, create_file, File};

mod tester_lib;
use tester_lib::*;

wit_bindgen::generate!({
    path: "target/wit",
    world: "tester-app-framework-demo-uncentered-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [PartialEq, serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

call_init!(init);
fn init(_our: Address) -> anyhow::Result<()> {
    // Even if we uncomment this it doesn't work
    // fail!("Well maybe this doesn't even work");
    print_to_terminal(0, "begin");

    loop {
        match handle_message() {
            Ok(()) => {}
            Err(e) => {
                fail!("dial_test");
            }
        };
    }
    Ok(())
}

fn handle_message() -> anyhow::Result<()> {
    let message = await_message().unwrap();
    if !message.is_request() {
        return Err(anyhow::anyhow!("not a request"));
    }

    let source = message.source();
    if our().node != source.node {
        return Err(anyhow::anyhow!(
            "rejecting foreign Message from {:?}",
            source,
        ));
    }

    let TesterRequest::Run(RunRequest {
        input_node_names: node_names,
        ..
    }) = message.body().try_into()?;

    Response::new()
        .body(TesterResponse::Run(Ok(())))
        .send()
        .unwrap();

    Ok(())
}

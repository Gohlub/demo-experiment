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

fn handle_message(our: &Address) -> anyhow::Result<()> {
    let message = await_message() else {
        fail!("Test failed due to error");
    };

    if !message.is_request() {
        unimplemented!();
    }
    let source = message.source();
    if our.node != source.node {
        return Err(anyhow::anyhow!(
            "rejecting foreign Message from {:?}",
            source,
        ));
    }
    let TesterRequest::Run(RunRequest {
        input_node_names: node_names,
        ..
    }) = message.body().try_into()?;
    print_to_terminal(0, "chat_test: a");
    assert!(node_names.len() >= 2);
    if our.node != node_names[0] {
        // we are not master node: return
        Response::new()
            .body(TesterResponse::Run(Ok(())))
            .send()
            .unwrap();
        return Ok(());
    }

    // we are master node

    let our_chat_address = Address {
        node: our.node.clone(),
        process: ProcessId::new(Some("indexer"), "app-framework-demo", "uncentered.os"),
    };
    let their_chat_address = Address {
        node: node_names[1].clone(),
        process: ProcessId::new(Some("client0"), "app-framework-demo", "uncentered.os"),
    };

    Response::new()
        .body(TesterResponse::Run(Ok(())))
        .send()
        .unwrap();

    Ok(())
}

call_init!(init);
fn init(our: Address) {
    print_to_terminal(0, "begin");

    loop {
        match handle_message(&our) {
            Ok(()) => {},
            Err(e) => {
                print_to_terminal(0, format!("chat_test: error: {e:?}").as_str());

                fail!("chat_test");
            },
        };
    }
}
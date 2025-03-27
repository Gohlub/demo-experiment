use crate::hyperware::process::tester::{
    FailResponse, Request as TesterRequest, Response as TesterResponse, RunRequest,
};
use hyperware_process_lib::our;

use hyperware_process_lib::{
    await_message, call_init, print_to_terminal, println, Address, ProcessId, Request, Response,
};

use hyperware_process_lib::vfs::{create_drive, create_file, File};
use caller_utils::*;

mod tester_lib;
use tester_lib::*;

wit_bindgen::generate!({
    path: "target/wit",
    world: "test-app-framework-demo-uncentered-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [PartialEq, serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

fn handle_message (our: &Address) -> anyhow::Result<()> {
    let message = await_message().unwrap();

    // let result = caller_utils::client::leet_remote_rpc()

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

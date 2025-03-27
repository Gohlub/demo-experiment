#[allow(unused_imports)]
use crate::hyperware::process::tester::{FailResponse, Response as TesterResponse};

#[macro_export]
macro_rules! fail {
    ($test:expr) => {
        Response::new()
            .body(TesterResponse::Run(Err(FailResponse {
                test: $test.into(),
                file: file!().into(),
                line: line!(),
                column: column!(),
            })))
            .send()
            .unwrap();
        panic!("")
    };
    ($test:expr, $file:expr, $line:expr, $column:expr) => {
        Response::new()
            .body(TesterResponse::Run(Err(FailResponse {
                test: $test.into(),
                file: $file.into(),
                line: $line,
                column: $column,
            })))
            .send()
            .unwrap();
        panic!("")
    };
}

#[macro_export]
macro_rules! async_test_suite {
    ($wit_world:expr, $($test_name:ident: async $test_body:block),* $(,)?) => {
        wit_bindgen::generate!({
            path: "target/wit",
            world: $wit_world,
            generate_unused_types: true,
            additional_derives: [PartialEq, serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
        });

        // Import only what we need within the macro scope without conflicting with outer imports
        use hyperware_process_lib::{
            await_message, call_init, print_to_terminal, Address, Response
        };
        
        $(
            async fn $test_name() -> anyhow::Result<()> {
                $test_body
            }
        )*
        
        async fn run_all_tests() -> anyhow::Result<()> {
            $(
                print_to_terminal(0, concat!("Running test: ", stringify!($test_name)));
                $test_name().await?;
                print_to_terminal(0, concat!("Test passed: ", stringify!($test_name)));
            )*
            
            print_to_terminal(0, "All tests passed!");
            Ok(())
        }

        call_init!(init);
        fn init(_our: Address) {
            print_to_terminal(0, "Starting test suite...");
            
            // Flag to track if tests have started
            let mut tests_started = false;
            
            // Main event loop
            loop {
                // Poll tasks to advance the executor
                hyperware_app_common::APP_CONTEXT.with(|ctx| {
                    ctx.borrow_mut().executor.poll_all_tasks();
                });
                
                if !tests_started {
                    // Wait for the first message to start tests
                    match await_message() {
                        Ok(_) => {
                            tests_started = true;
                            print_to_terminal(0, "Running tests...");
                            
                            // Start the test suite with the hyper! macro
                            hyperware_app_common::hyper! {
                                match run_all_tests().await {
                                    Ok(()) => {
                                        // All tests passed - send success response
                                        Response::new()
                                            .body(TesterResponse::Run(Ok(())))
                                            .send()
                                            .unwrap();
                                    },
                                    Err(e) => {
                                        // Test failed - report failure
                                        print_to_terminal(0, format!("Test suite failed: {:?}", e).as_str());
                                        crate::fail!("test_suite");
                                    }
                                }
                            }
                        },
                        Err(e) => {
                            print_to_terminal(0, format!("Failed to receive initial message: {:?}", e).as_str());
                            crate::fail!("initial_message");
                        }
                    }
                } else {
                    // Tests have started, just process additional messages
                    // to keep the event loop running and allow async tasks to progress
                    match await_message() {
                        Ok(_) => {}, // Ignore additional messages
                        Err(_) => {}, // Ignore errors too
                    }
                }
            }
        }
    };
}
use crate::hyperware::process::tester::{FailResponse, Response as TesterResponse};
mod tester_lib;
use caller_utils::client::*;
use hyperware_app_common::SendResult;
use tester_lib::*;

async_test_suite!(
    "test-app-framework-demo-uncentered-dot-os-v0",

    test_basic_math: async {
        if 2 + 2 != 4 {
            fail!("wrong result");
        }
        Ok(())
    },

    test_function_call: async {
        let address: Address = ("client.os", "client", "app-framework-demo", "uncentered.os").into();
        let result = test_remote_call(
            caller_utils::client::leet_remote_rpc(&address, 1337),
            1337 * 1337,
            "wrong leet_remote_rpc result"
        ).await?;
        print_to_terminal(0, &format!("leet_remote_rpc result: {}", result));
        Ok(())
    },

    test_just_leet: async {
        let address: Address = ("client.os", "client", "app-framework-demo", "uncentered.os").into();
        let result = test_remote_call(
            caller_utils::client::just_leet_remote_rpc(&address),
            1337,
            "wrong just_leet_remote_rpc result"
        ).await?;
        print_to_terminal(0, &format!("just_leet_remote_rpc result: {}", result));
        Ok(())
    },

    // Add more tests here as needed

);

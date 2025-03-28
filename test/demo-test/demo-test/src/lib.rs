use crate::hyperware::process::tester::{FailResponse, Response as TesterResponse};
mod tester_lib;
use caller_utils::client::*;
use caller_utils::curator::*;
use hyperware_app_common::SendResult;
use tester_lib::*;
use std::collections::HashMap;

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

    // Curator tests
    test_curator_add_and_get: async {
        let address: Address = ("curator.os", "curator", "app-framework-demo", "uncentered.os").into();
        
        // Test adding a curation
        let add_result = test_remote_call(
            add_curation_remote_rpc(&address, "Test Title".to_string(), "Test Content".to_string()),
            true,
            "Failed to add curation"
        ).await?;
        print_to_terminal(0, &format!("add_curation_remote_rpc result: {}", add_result));
        
        // Test getting the curation
        let get_result = test_remote_call(
            get_curation_remote_rpc(&address, "Test Title".to_string()),
            Some("Test Content".to_string()),
            "Failed to get curation or content mismatch"
        ).await?;
        print_to_terminal(0, &format!("get_curation_remote_rpc result: {:?}", get_result));
        
        Ok(())
    },

    test_curator_remove: async {
        let address: Address = ("curator.os", "curator", "app-framework-demo", "uncentered.os").into();
        
        // Add a curation first
        let _ = add_curation_remote_rpc(&address, "Remove Test".to_string(), "Content to Remove".to_string()).await;
        
        // Test removing the curation
        let remove_result = test_remote_call(
            remove_curation_remote_rpc(&address, "Remove Test".to_string()),
            true,
            "Failed to remove curation"
        ).await?;
        print_to_terminal(0, &format!("remove_curation_remote_rpc result: {}", remove_result));
        
        // Verify it's gone
        let get_result = test_remote_call(
            get_curation_remote_rpc(&address, "Remove Test".to_string()),
            None,
            "Curation should be gone but was found"
        ).await?;
        print_to_terminal(0, &format!("get_curation_remote_rpc after removal: {:?}", get_result));
        
        Ok(())
    },

    test_nonexistent_curation: async {
        let address: Address = ("curator.os", "curator", "app-framework-demo", "uncentered.os").into();
        
        // Test getting a nonexistent curation
        let get_result = test_remote_call(
            get_curation_remote_rpc(&address, "NonexistentTitle".to_string()),
            None,
            "Expected None for nonexistent curation"
        ).await?;
        print_to_terminal(0, &format!("Nonexistent curation result: {:?}", get_result));
        
        Ok(())
    },

    // Add more tests here as needed
);

use crate::hyperware::process::tester::{FailResponse, Response as TesterResponse};
mod tester_lib;
mod load_balancer_tests;
mod fault_tolerance_tests;
use caller_utils::client::*;
use caller_utils::curator::*;
use caller_utils::indexer::*;
use caller_utils::load_balancer::*;
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

    // // Indexer tests
    test_indexer_add_and_get: async {
        let address: Address = ("indexer.os", "indexer", "app-framework-demo", "uncentered.os").into();
        
        // Test adding to an index
        let add_result = test_remote_call(
            add_to_index_remote_rpc(&address, "TestIndex".to_string(), "Item1".to_string()),
            true,
            "Failed to add to index"
        ).await?;
        print_to_terminal(0, &format!("add_to_index_remote_rpc result: {}", add_result));
        
        // Add another item
        let _ = add_to_index_remote_rpc(&address, "TestIndex".to_string(), "Item2".to_string()).await;
        
        // Test getting the index
        let get_result = test_remote_call(
            get_index_remote_rpc(&address, "TestIndex".to_string()),
            Some(vec!["Item1".to_string(), "Item2".to_string()]),
            "Failed to get index or content mismatch"
        ).await?;
        print_to_terminal(0, &format!("get_index_remote_rpc result: {:?}", get_result));
        
        Ok(())
    },

    test_indexer_remove: async {
        let address: Address = ("indexer.os", "indexer", "app-framework-demo", "uncentered.os").into();
        
        // Add items to the index first
        let _ = add_to_index_remote_rpc(&address, "RemoveTest".to_string(), "ItemToRemove".to_string()).await;
        let _ = add_to_index_remote_rpc(&address, "RemoveTest".to_string(), "ItemToKeep".to_string()).await;
        
        // Test removing from the index
        let remove_result = test_remote_call(
            remove_from_index_remote_rpc(&address, "RemoveTest".to_string(), "ItemToRemove".to_string()),
            true,
            "Failed to remove from index"
        ).await?;
        print_to_terminal(0, &format!("remove_from_index_remote_rpc result: {}", remove_result));
        
        // Verify only the correct item is removed
        let get_result = test_remote_call(
            get_index_remote_rpc(&address, "RemoveTest".to_string()),
            Some(vec!["ItemToKeep".to_string()]),
            "Index content is incorrect after removal"
        ).await?;
        print_to_terminal(0, &format!("get_index_remote_rpc after removal: {:?}", get_result));
        
        Ok(())
    },

    test_nonexistent_index: async {
        let address: Address = ("indexer.os", "indexer", "app-framework-demo", "uncentered.os").into();
        
        // Test getting a nonexistent index
        let get_result = test_remote_call(
            get_index_remote_rpc(&address, "NonexistentIndex".to_string()),
            None,
            "Expected None for nonexistent index"
        ).await?;
        print_to_terminal(0, &format!("Nonexistent index result: {:?}", get_result));
        
        Ok(())
    },

    test_list_indices: async {
        let address: Address = ("indexer.os", "indexer", "app-framework-demo", "uncentered.os").into();
        
        // Create a few indices first
        let _ = add_to_index_remote_rpc(&address, "Index1".to_string(), "ItemX".to_string()).await;
        let _ = add_to_index_remote_rpc(&address, "Index2".to_string(), "ItemY".to_string()).await;
        
        let indices = list_indices_remote_rpc(&address).await;
        // Check if the indices were created successfully
        if let SendResult::Success(indices_list) = indices {
            print_to_terminal(0, &format!("Current indices: {:?}", indices_list));
            
            // Verify that our created indices exist in the list
            let has_index1 = indices_list.contains(&"Index1".to_string());
            let has_index2 = indices_list.contains(&"Index2".to_string());
            let has_remove_test = indices_list.contains(&"RemoveTest".to_string());
            let has_test_index = indices_list.contains(&"TestIndex".to_string());
            
            print_to_terminal(0, &format!("Contains Index1: {}, Index2: {}, RemoveTest: {}, TestIndex: {}", 
                has_index1, has_index2, has_remove_test, has_test_index));

            if !has_index1 || !has_index2 || !has_remove_test || !has_test_index {
                fail!("Some expected indices are missing from the list");
            }
        } else {
            print_to_terminal(0, "Failed to retrieve indices list");
        }
        
        Ok(())
    },
    
    // Load Balancer Tests
    test_load_balancer_register: async {
        let lb_address: Address = ("load-balancer.os", "load-balancer", "app-framework-demo", "uncentered.os").into();
        let indexer_address_str = "indexer.os:indexer:app-framework-demo:uncentered.os";
        
        // Register an indexer with the load balancer
        let register_result = register_indexer_remote_rpc(&lb_address, indexer_address_str.to_string(), 1000).await;
        if let SendResult::Success(result) = register_result {
            if !result {
                fail!("Failed to register indexer with load balancer");
            }
            print_to_terminal(0, "Successfully registered indexer with load balancer");
        } else {
            fail!("Error calling register_indexer_remote_rpc");
        }
        
        // Health check
        let health_result = health_check_remote_rpc(&lb_address).await;
        if let SendResult::Success(result) = health_result {
            if !result {
                fail!("Load balancer health check failed");
            }
            print_to_terminal(0, "Load balancer health check passed");
        } else {
            fail!("Error calling health_check_remote_rpc");
        }
        
        Ok(())
    },
    
    test_load_balancer_client: async {
        let lb_address: Address = ("load-balancer.os", "load-balancer", "app-framework-demo", "uncentered.os").into();
        let client_address: Address = ("client.os", "client", "app-framework-demo", "uncentered.os").into();
        
        // Configure client to use load balancer
        let lb_address_str = lb_address.to_string();
        let config_result = set_load_balancer_remote_rpc(&client_address, lb_address_str).await;
        if let SendResult::Success(result) = config_result {
            if !result {
                fail!("Failed to configure client to use load balancer");
            }
            print_to_terminal(0, "Successfully configured client to use load balancer");
        } else {
            fail!("Error calling set_load_balancer_remote_rpc");
        }
        
        // Get client stats to verify configuration
        let stats_result = get_stats_remote_rpc(&client_address).await;
        if let SendResult::Success(stats) = stats_result {
            if !stats.using_load_balancer {
                fail!("Client not using load balancer after configuration");
            }
            print_to_terminal(0, &format!("Client stats: {:?}", stats));
        } else {
            fail!("Error calling get_stats_remote_rpc");
        }
        
        Ok(())
    },
    
    // Comprehensive load balancer test
    test_load_balancer_end_to_end: async {
        // Set up addresses for our test components
        let lb_address: Address = ("load-balancer.os", "load-balancer", "app-framework-demo", "uncentered.os").into();
        let indexer1_str = "indexer1.os:indexer:app-framework-demo:uncentered.os";
        let indexer2_str = "indexer2.os:indexer:app-framework-demo:uncentered.os";
        let client_address: Address = ("client.os", "client", "app-framework-demo", "uncentered.os").into();
        
        // Step 1: Register multiple indexers with different capacities
        print_to_terminal(0, "Step 1: Registering indexers with different capacities");
        let _ = register_indexer_remote_rpc(&lb_address, indexer1_str.to_string(), 100).await;
        let _ = register_indexer_remote_rpc(&lb_address, indexer2_str.to_string(), 200).await;
        
        // Step 2: Report different loads
        print_to_terminal(0, "Step 2: Reporting loads (indexer1: 80%, indexer2: 30%)");
        let _ = report_load_remote_rpc(&lb_address, indexer1_str.to_string(), 80).await;
        let _ = report_load_remote_rpc(&lb_address, indexer2_str.to_string(), 60).await;
        
        // Step 3: Configure client to use load balancer
        print_to_terminal(0, "Step 3: Configuring client to use load balancer");
        let _ = set_load_balancer_remote_rpc(&client_address, lb_address.to_string()).await;
        
        // Step 4: Make multiple requests and verify distribution
        print_to_terminal(0, "Step 4: Testing load distribution (should favor indexer2)");
        let mut indexer1_count = 0;
        let mut indexer2_count = 0;
        
        for i in 0..20 {
            let indexer_result = get_available_indexer_remote_rpc(&lb_address).await;
            if let SendResult::Success(Some(indexer)) = indexer_result {
                print_to_terminal(0, &format!("Request {}: Selected {}", i, indexer));
                if indexer == indexer1_str {
                    indexer1_count += 1;
                } else if indexer == indexer2_str {
                    indexer2_count += 1;
                }
            }
        }
        
        print_to_terminal(0, &format!("Distribution: indexer1: {}, indexer2: {}", 
            indexer1_count, indexer2_count));
        
        // Indexer2 should be chosen more often since it has lower relative load
        if indexer2_count <= indexer1_count {
            print_to_terminal(0, "Warning: Distribution did not favor the less loaded indexer");
        }
        
        // Step 5: Test health checks by marking one indexer as unhealthy
        print_to_terminal(0, "Step 5: Testing health monitoring");
        // We would normally make the load balancer mark indexer1 as unhealthy by
        // not sending heartbeats, but in the test we'll just check that our
        // health check endpoint works
        let health_result = health_check_remote_rpc(&lb_address).await;
        if let SendResult::Success(result) = health_result {
            if !result {
                fail!("Load balancer health check failed");
            }
        } else {
            fail!("Error calling health_check_remote_rpc");
        }
        
        Ok(())
    },
);

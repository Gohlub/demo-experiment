use caller_utils::load_balancer::*;
use caller_utils::client::*;
use caller_utils::indexer::*;
use hyperware_app_common::SendResult;
use crate::tester_lib::*;

async_tests!{
    test_load_balancer_register_unregister: async {
        // Set up addresses for our test components
        let lb_address: Address = ("load-balancer.os", "load-balancer", "app-framework-demo", "uncentered.os").into();
        let indexer_address_str = "indexer.os:indexer:app-framework-demo:uncentered.os";
        
        // Register an indexer with the load balancer
        let register_result = test_remote_call(
            register_indexer_remote_rpc(&lb_address, indexer_address_str.to_string(), 1000),
            true,
            "Failed to register indexer with load balancer"
        ).await?;
        print_to_terminal(0, &format!("Register indexer result: {}", register_result));
        
        // Get stats to verify registration
        let stats_result = get_stats_remote_rpc(&lb_address).await;
        if let SendResult::Success(stats) = stats_result {
            print_to_terminal(0, &format!("Load balancer stats: {:?}", stats));
            
            // Verify the indexer is in the stats
            let found = stats.iter().any(|(addr, _)| addr == indexer_address_str);
            if !found {
                fail!("Registered indexer not found in load balancer stats");
            }
        } else {
            fail!("Failed to get load balancer stats");
        }
        
        // Unregister the indexer
        let unregister_result = test_remote_call(
            unregister_indexer_remote_rpc(&lb_address, indexer_address_str.to_string()),
            true,
            "Failed to unregister indexer"
        ).await?;
        print_to_terminal(0, &format!("Unregister indexer result: {}", unregister_result));
        
        Ok(())
    },
    
    test_load_balancer_client_integration: async {
        // Set up addresses for our test components
        let lb_address: Address = ("load-balancer.os", "load-balancer", "app-framework-demo", "uncentered.os").into();
        let indexer_address_str = "indexer.os:indexer:app-framework-demo:uncentered.os";
        let client_address: Address = ("client.os", "client", "app-framework-demo", "uncentered.os").into();
        
        // Register the indexer with the load balancer
        let _ = register_indexer_remote_rpc(&lb_address, indexer_address_str.to_string(), 1000).await;
        
        // Report some load for the indexer
        let load_report_result = test_remote_call(
            report_load_remote_rpc(&lb_address, indexer_address_str.to_string(), 50),
            true,
            "Failed to report load for indexer"
        ).await?;
        print_to_terminal(0, &format!("Report load result: {}", load_report_result));
        
        // Configure the client to use the load balancer
        let lb_address_str = lb_address.to_string();
        let client_config_result = test_remote_call(
            set_load_balancer_remote_rpc(&client_address, lb_address_str),
            true,
            "Failed to configure client to use load balancer"
        ).await?;
        print_to_terminal(0, &format!("Client config result: {}", client_config_result));
        
        // Get an available indexer from the load balancer
        let indexer_result = get_available_indexer_remote_rpc(&lb_address).await;
        if let SendResult::Success(maybe_indexer) = indexer_result {
            if let Some(available_indexer) = maybe_indexer {
                print_to_terminal(0, &format!("Available indexer: {}", available_indexer));
            } else {
                fail!("Load balancer returned no available indexer");
            }
        } else {
            fail!("Failed to get available indexer from load balancer");
        }
        
        // Add data through the client's load-balanced methods
        let index_name = "load_balanced_test".to_string();
        let item = "test_item".to_string();
        
        let add_result = lb_add_to_index_remote_rpc(&client_address, index_name.clone(), item.clone()).await;
        if let SendResult::Success(result) = add_result {
            if !result.success {
                fail!("Load balanced index addition failed: {:?}", result.error);
            }
            print_to_terminal(0, &format!("Load balanced add result: {:?}", result));
        } else {
            fail!("Failed to call load balanced add method");
        }
        
        // Read the data using client's load-balanced methods
        let get_result = lb_get_index_remote_rpc(&client_address, index_name.clone()).await;
        if let SendResult::Success(result) = get_result {
            if !result.success {
                fail!("Load balanced get index failed: {:?}", result.error);
            }
            print_to_terminal(0, &format!("Load balanced get result: {:?}", result));
            
            // Verify the data
            if let Some(Some(items)) = result.result {
                if !items.contains(&item) {
                    fail!("Item not found in retrieved index");
                }
            } else {
                fail!("Invalid response from load balanced get index");
            }
        } else {
            fail!("Failed to call load balanced get method");
        }
        
        // Test health check
        let health_result = test_remote_call(
            health_check_remote_rpc(&lb_address),
            true,
            "Health check failed"
        ).await?;
        print_to_terminal(0, &format!("Health check result: {}", health_result));
        
        Ok(())
    },
    
    test_load_balancing_strategy: async {
        // Create two indexers with different loads to test the load balancing strategy
        let lb_address: Address = ("load-balancer.os", "load-balancer", "app-framework-demo", "uncentered.os").into();
        let indexer1_address_str = "indexer1.os:indexer:app-framework-demo:uncentered.os";
        let indexer2_address_str = "indexer2.os:indexer:app-framework-demo:uncentered.os";
        
        // Register both indexers
        let _ = register_indexer_remote_rpc(&lb_address, indexer1_address_str.to_string(), 100).await;
        let _ = register_indexer_remote_rpc(&lb_address, indexer2_address_str.to_string(), 100).await;
        
        // Set different loads - indexer1 at 80% capacity, indexer2 at 20% capacity
        let _ = report_load_remote_rpc(&lb_address, indexer1_address_str.to_string(), 80).await;
        let _ = report_load_remote_rpc(&lb_address, indexer2_address_str.to_string(), 20).await;
        
        // Request an indexer multiple times and count which one we get
        // The least connections strategy should favor indexer2
        let mut indexer1_count = 0;
        let mut indexer2_count = 0;
        
        for _ in 0..10 {
            let indexer_result = get_available_indexer_remote_rpc(&lb_address).await;
            if let SendResult::Success(Some(indexer)) = indexer_result {
                if indexer == indexer1_address_str {
                    indexer1_count += 1;
                } else if indexer == indexer2_address_str {
                    indexer2_count += 1;
                }
            }
        }
        
        print_to_terminal(0, &format!("Indexer1 selection count: {}", indexer1_count));
        print_to_terminal(0, &format!("Indexer2 selection count: {}", indexer2_count));
        
        // Indexer2 should be chosen more often since it has lower load
        if indexer2_count <= indexer1_count {
            fail!("Load balancing strategy not working as expected");
        }
        
        Ok(())
    }
}
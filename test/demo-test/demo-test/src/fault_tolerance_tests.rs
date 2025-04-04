use caller_utils::load_balancer::*;
use caller_utils::client::*;
use caller_utils::indexer::*;
use hyperware_app_common::SendResult;
use crate::tester_lib::*;

async_tests!{
    test_load_balancer_fault_tolerance: async {
        // Set up addresses for our test components
        let lb_address: Address = ("load-balancer.os", "load-balancer", "app-framework-demo", "uncentered.os").into();
        let indexer1_str = "indexer1.os:indexer:app-framework-demo:uncentered.os";
        let indexer2_str = "indexer2.os:indexer:app-framework-demo:uncentered.os";
        let client_address: Address = ("client.os", "client", "app-framework-demo", "uncentered.os").into();
        
        // Step 1: Register two indexers
        print_to_terminal(0, "Registering two indexers with the load balancer");
        let _ = register_indexer_remote_rpc(&lb_address, indexer1_str.to_string(), 100).await;
        let _ = register_indexer_remote_rpc(&lb_address, indexer2_str.to_string(), 100).await;
        
        // Step 2: Report loads for both indexers
        print_to_terminal(0, "Reporting normal loads for both indexers");
        let _ = report_load_remote_rpc(&lb_address, indexer1_str.to_string(), 50).await;
        let _ = report_load_remote_rpc(&lb_address, indexer2_str.to_string(), 50).await;
        
        // Step 3: Verify both indexers are available
        print_to_terminal(0, "Checking that both indexers are available");
        let stats_result = get_stats_remote_rpc(&lb_address).await;
        if let SendResult::Success(stats) = stats_result {
            let has_indexer1 = stats.iter().any(|(addr, _)| addr == indexer1_str);
            let has_indexer2 = stats.iter().any(|(addr, _)| addr == indexer2_str);
            
            if !has_indexer1 || !has_indexer2 {
                fail!("Not all indexers are present in the stats");
            }
            
            print_to_terminal(0, &format!("Initial stats: {:?}", stats));
        } else {
            fail!("Failed to get load balancer stats");
        }
        
        // Step 4: Simulate indexer1 going offline by unregistering it
        print_to_terminal(0, "Simulating indexer1 going offline");
        let _ = unregister_indexer_remote_rpc(&lb_address, indexer1_str.to_string()).await;
        
        // Step 5: Make multiple requests and verify that only indexer2 is used
        print_to_terminal(0, "Making requests (should all use indexer2)");
        let mut indexer1_count = 0;
        let mut indexer2_count = 0;
        let mut other_count = 0;
        
        for i in 0..5 {
            let indexer_result = get_available_indexer_remote_rpc(&lb_address).await;
            if let SendResult::Success(Some(indexer)) = indexer_result {
                print_to_terminal(0, &format!("Request {}: Selected {}", i, indexer));
                if indexer == indexer1_str {
                    indexer1_count += 1;
                } else if indexer == indexer2_str {
                    indexer2_count += 1;
                } else {
                    other_count += 1;
                }
            }
        }
        
        // Step 6: Verify that indexer1 was never selected
        print_to_terminal(0, &format!("Distribution: indexer1 (offline): {}, indexer2: {}, other: {}", 
            indexer1_count, indexer2_count, other_count));
        
        if indexer1_count > 0 {
            fail!("Offline indexer was selected for requests");
        }
        
        if indexer2_count == 0 {
            fail!("No healthy indexer was selected for requests");
        }
        
        // Step 7: Bring indexer1 back online
        print_to_terminal(0, "Bringing indexer1 back online");
        let _ = register_indexer_remote_rpc(&lb_address, indexer1_str.to_string(), 100).await;
        let _ = report_load_remote_rpc(&lb_address, indexer1_str.to_string(), 20).await; // Lower load than indexer2
        
        // Step 8: Configure client to use direct indexer when load balancer is unavailable
        print_to_terminal(0, "Setting up client with fallback to direct indexer");
        let _ = set_load_balancer_remote_rpc(&client_address, lb_address.to_string()).await;
        let _ = set_direct_indexer_remote_rpc(&client_address, indexer1_str.to_string()).await;
        
        // In a real system, we would test the client's fallback mechanism by making 
        // the load balancer unavailable, but that's challenging in a test environment
        
        Ok(())
    }
}
use crate::hyperware::process::tester::{FailResponse, Response as TesterResponse};
mod tester_lib;
use tester_lib::*;

async_test_suite!(
    "test-app-framework-demo-uncentered-dot-os-v0",
    
    test_basic_math: async {
        assert_eq!(2 + 2, 4);
        Ok(())
    },
    
    // Add more tests here as needed
);
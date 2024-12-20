use j4fsmb_tests::ServerProcess;

#[tokio::test]
async fn test_simple_read() {
    let _handle = ServerProcess::new().waited_until_running().await;

}

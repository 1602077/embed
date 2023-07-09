use crate::helpers::run_test_app;

#[tokio::test]
async fn hello_world() {
    let app = run_test_app().await;
}

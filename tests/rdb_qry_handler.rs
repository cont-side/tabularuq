mod shared;
use shared::*;
use tabularuq::rdb_qry_handler::{error::QueryHandleError, DataRows, QueryHandler};

#[tokio::test]
async fn mock_connect_test() {
    let mut handler_ok_mock = MockQueryHandlerMock::new();
    handler_ok_mock.expect_connect().returning(|| Ok(()));
    let ok_result = handler_ok_mock.connect().await;
    assert!(ok_result.is_ok());

    let mut handler_err_mock = MockQueryHandlerMock::new();
    handler_err_mock
        .expect_connect()
        .returning(|| Err(Box::new(QueryHandleError::Unknown("Unknown".to_string()))));
    let err_result = handler_err_mock.connect().await;
    let err = err_result.unwrap_err();
    assert_eq!(
        *err.downcast::<QueryHandleError>().unwrap(),
        QueryHandleError::Unknown("Unknown".to_string())
    );
}

#[tokio::test]
async fn mock_query_test() {
    let mut handler_ok_mock = MockQueryHandlerMock::new();
    handler_ok_mock.expect_query().returning(|_, _| Ok(DataRows::new(None, Vec::new())));
    let ok_result = handler_ok_mock.query("SELECT * FROM table", None).await;
    assert!(ok_result.is_ok());

    let mut handler_err_mock = MockQueryHandlerMock::new();
    handler_err_mock
        .expect_query()
        .returning(|_, _| Err(Box::new(QueryHandleError::Unknown("Unknown".to_string()))));
    let err_result = handler_err_mock.query("SELECT * FROM table", None).await;
    let err = err_result.unwrap_err();
    assert_eq!(
        *err.downcast::<QueryHandleError>().unwrap(),
        QueryHandleError::Unknown("Unknown".to_string())
    );
}

#[tokio::test]
async fn mock_mutate_test() {
    let mut handler_ok_mock = MockQueryHandlerMock::new();
    handler_ok_mock
        .expect_mutate()
        .returning(|_, _| Box::pin(async { Ok(MockQueryResultMock::new()) }));
    let ok_result = handler_ok_mock.mutate("INSERT INTO table", None).await;
    assert!(ok_result.is_ok());

    let mut handler_err_mock = MockQueryHandlerMock::new();
    handler_err_mock.expect_mutate().returning(|_, _| {
        Box::pin(async {
            Err(Box::new(QueryHandleError::Unknown("Unknown".to_string()))
                as Box<dyn std::error::Error>)
        })
    });
    let err_result = handler_err_mock.mutate("INSERT INTO table", None).await;
    let err = err_result.unwrap_err();
    assert_eq!(
        *err.downcast::<QueryHandleError>().unwrap(),
        QueryHandleError::Unknown("Unknown".to_string())
    )
}

#[tokio::test]
async fn mock_close_test() {
    let mut handler_ok_mock = MockQueryHandlerMock::new();
    handler_ok_mock.expect_close().returning(|| Ok(()));
    let ok_result = handler_ok_mock.close().await;
    assert!(ok_result.is_ok());

    let mut handler_err_mock = MockQueryHandlerMock::new();
    handler_err_mock
        .expect_close()
        .returning(|| Err(Box::new(QueryHandleError::Unknown("Unknown".to_string()))));
    let err_result = handler_err_mock.close().await;
    let err = err_result.unwrap_err();
    assert_eq!(
        *err.downcast::<QueryHandleError>().unwrap(),
        QueryHandleError::Unknown("Unknown".to_string())
    );
}

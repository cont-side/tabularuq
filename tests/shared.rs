use mockall::mock;
use std::{future::Future, sync::Arc};
use tabularuq::rdb_qry_handler::datatype::DataType;
use tabularuq::rdb_qry_handler::*;

mock! {
    #[derive(Debug)]
    pub QueryResultMock {}

    impl QueryResult for QueryResultMock {
        fn affected_rows(&self) -> u64;
    }
}

mock! {
    pub QueryHandlerMock {}

    impl QueryHandler for QueryHandlerMock {
        type ConnectionConfig = u32;

        fn from_config(
            conn_config: u32,
        ) -> Result<Self, Box<dyn std::error::Error>>;

        async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>>;

        async fn query(
            &mut self,
            query: &str,
            bind_variables: Option<Arc<[DataType]>>,
            fetch_more: Box<dyn Fn(Option<&[String]>, Option<&DataRecord>) -> bool + Send>,
        ) -> Result<DataRows, Box<dyn std::error::Error>>;

        fn mutate(
            &mut self,
            query: &str,
            bind_variables: Option<Arc<[DataType]>>,
        ) -> impl Future<Output = Result<MockQueryResultMock, Box<dyn std::error::Error>>> + Send;

        async fn close(self) -> Result<(), Box<dyn std::error::Error>>;
    }
}

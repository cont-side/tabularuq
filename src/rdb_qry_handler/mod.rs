use datatype::{DataType, FromDataType};
use serde::Deserialize;
use std::{fs::read_to_string, future::Future, path::Path, sync::Arc};

pub mod datatype;
pub mod error;
pub mod sqlserver;

#[derive(Debug)]
pub struct DataRecord {
    cells: Vec<Option<DataType>>,
}

pub trait IntoDataRecord {
    fn into_data_record(self) -> DataRecord;
}

impl DataRecord {
    pub fn cells(&self) -> &[Option<DataType>] {
        &self.cells
    }

    pub fn cell(&self, index: usize) -> Option<&DataType> {
        self.cells.get(index).and_then(|cell| cell.as_ref())
    }

    pub fn cvalue<T>(&self, index: usize) -> Option<T>
    where
        T: FromDataType,
    {
        self.cell(index).and_then(|cell| cell.value::<T>())
    }
}

#[derive(Debug)]
pub struct DataRows {
    column_meta: Option<Vec<String>>,
    records: Vec<DataRecord>,
}

impl DataRows {
    pub fn new(column_meta: Option<Vec<String>>, records: Vec<DataRecord>) -> Self {
        DataRows { column_meta, records }
    }

    pub fn column_meta(&self) -> Option<&[String]> {
        self.column_meta.as_deref()
    }

    pub fn records(&self) -> &[DataRecord] {
        &self.records
    }
}

pub trait QueryResult {
    fn affected_rows(&self) -> u64;
}

pub trait QueryHandler {
    type ConnectionConfig;

    fn from_config(conn_config: Self::ConnectionConfig) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;

    fn connect(&mut self) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>> + Send;

    fn query(
        &mut self,
        query: &str,
        bind_variables: Option<Arc<[DataType]>>,
        fetch_more: Box<dyn Fn(Option<&[String]>, Option<&DataRecord>) -> bool + Send>,
    ) -> impl Future<Output = Result<DataRows, Box<dyn std::error::Error>>> + Send;

    fn mutate(
        &mut self,
        query: &str,
        bind_variables: Option<Arc<[DataType]>>,
    ) -> impl Future<Output = Result<impl QueryResult, Box<dyn std::error::Error>>> + Send;

    fn close(self) -> impl Future<Output = Result<(), Box<dyn std::error::Error>>> + Send;

    fn default_fetch_more() -> Box<dyn Fn(Option<&[String]>, Option<&DataRecord>) -> bool + Send> {
        Box::new(|_, _| true)
    }
}

#[derive(Deserialize)]
pub struct DataSourceInform {
    driver: String,
    sqlserver: Option<sqlserver::SqlServerConnectionConfig>,
}

impl DataSourceInform {
    pub fn new(driver: String, sqlserver: Option<sqlserver::SqlServerConnectionConfig>) -> Self {
        DataSourceInform { driver, sqlserver }
    }
}

//TODO: deprecated
#[allow(dead_code)]
pub fn qry_handler<P>(config_file: P) -> Option<impl QueryHandler>
where
    P: AsRef<Path>,
{
    let contents = read_to_string(config_file).ok()?;
    let connect_content = contents.as_str();

    let connection_config: DataSourceInform = toml::from_str(connect_content).ok()?;
    let driver = connection_config.driver.as_str();

    match driver {
        "sqlserver" => {
            let conn_config = connection_config.sqlserver?;
            let handler = sqlserver::SqlServerHandler::from_config(conn_config).ok()?;
            Some(handler)
        }
        _ => {
            println!("Not supported driver");
            None
        }
    }
}

pub fn qry_handler_from_dsi(inform: DataSourceInform) -> Option<impl QueryHandler> {
    let driver = inform.driver.as_str();

    match driver {
        "sqlserver" => {
            let conn_config = inform.sqlserver?;
            let handler = sqlserver::SqlServerHandler::from_config(conn_config).ok()?;
            Some(handler)
        }
        _ => {
            println!("Not supported driver");
            None
        }
    }
}

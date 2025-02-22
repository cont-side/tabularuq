use std::sync::Arc;

use chrono::{DateTime, Duration};
use futures::stream::TryStreamExt;
use serde::Deserialize;
use tiberius::{AuthMethod, Client, ColumnData, Config, Query, QueryItem, ResultMetadata, Row};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

use crate::rdb_qry_handler::{
    DataRecord, DataRows, DataType, IntoDataRecord, QueryHandler, QueryResult,
};

use super::error::QueryHandleError;

trait IntoMetaRecord {
    fn into_meta_rec(self) -> (Option<Vec<String>>, Option<DataRecord>);
}

impl IntoMetaRecord for QueryItem {
    fn into_meta_rec(self) -> (Option<Vec<String>>, Option<DataRecord>) {
        match self {
            QueryItem::Metadata(meta) => {
                let col_meta = SqlServerHandler::col_meta(Some(&meta));
                (col_meta, Option::None)
            }
            QueryItem::Row(row) => {
                let record = row.into_data_record();
                (Option::None, Option::Some(record))
            }
        }
    }
}

pub struct SqlServerHandler {
    conn_config: SqlServerConnectionConfig,
    client: Option<Client<Compat<TcpStream>>>,
}

impl QueryHandler for SqlServerHandler {
    type ConnectionConfig = SqlServerConnectionConfig;

    fn from_config(
        conn_config: SqlServerConnectionConfig,
    ) -> Result<SqlServerHandler, Box<dyn std::error::Error>> {
        Result::Ok(SqlServerHandler { conn_config, client: Option::None })
    }

    async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let conn_config = &self.conn_config;

        let config = SqlServerHandler::create_config(conn_config)?;
        let tcp = TcpStream::connect(config.get_addr()).await?;
        tcp.set_nodelay(true)?;
        let client = Client::connect(config, tcp.compat_write()).await?;
        self.client = Some(client);

        Result::Ok(())
    }

    async fn query(
        &mut self,
        query: &str,
        bind_variables: Option<Arc<[DataType]>>,
        fetch_more: Box<dyn Fn(Option<&[String]>, Option<&DataRecord>) -> bool + Send>,
    ) -> Result<DataRows, Box<dyn std::error::Error>> {
        let client = self
            .client
            .as_mut()
            .ok_or(QueryHandleError::NotInitialized("Client is not initialized".to_string()))?;

        let mut select = Query::new(query);
        if let Some(bind_vars) = bind_variables {
            for bind_var in bind_vars.iter() {
                SqlServerHandler::bind_query(&mut select, bind_var);
            }
        }
        let mut stream = select.query(client).await?;

        let mut column_meta = None;
        let mut records = Vec::new();
        while let Some(item) = stream.try_next().await? {
            let (col_meta, record) = item.into_meta_rec();

            let need_to_continue =
                fetch_more(col_meta.as_ref().map(|value| value.as_slice()), record.as_ref());

            if col_meta.is_some() {
                column_meta = col_meta;
            }
            if let Some(record) = record {
                records.push(record);
            }

            if !need_to_continue {
                break;
            }
        }

        Result::Ok(DataRows { column_meta, records })
    }

    async fn mutate(
        &mut self,
        query: &str,
        bind_variables: Option<&[DataType]>,
    ) -> Result<impl QueryResult, Box<dyn std::error::Error>> {
        let client = self
            .client
            .as_mut()
            .ok_or(QueryHandleError::NotInitialized("Client is not initialized".to_string()))?;

        let mut mutate = Query::new(query);
        if let Some(bind_vars) = bind_variables {
            for bind_var in bind_vars {
                SqlServerHandler::bind_query(&mut mutate, bind_var);
            }
        }

        let result = mutate.execute(client).await?;
        Result::Ok(QueryAffectedRows { affected_rows: result.total() })
    }

    async fn close(self) -> Result<(), Box<dyn std::error::Error>> {
        let client = self.client.ok_or(QueryHandleError::InvalidCall(
            "Client cannot be unwrapped. Ownership might already be moved.".to_string(),
        ))?;

        client.close().await?;
        Result::Ok(())
    }
}

#[derive(Deserialize)]
pub struct SqlServerConnectionConfig {
    ado_string: Option<String>,
    host: String,
    port: Option<u16>,
    database: String,
    username: String,
    password: String,
}

impl SqlServerHandler {
    fn create_config(
        conn_config: &SqlServerConnectionConfig,
    ) -> Result<Config, Box<dyn std::error::Error>> {
        let ado = conn_config.ado_string.as_ref();
        let host = conn_config.host.as_str();
        let port = conn_config.port.unwrap_or(1433);
        let database = conn_config.database.as_str();
        let username = conn_config.username.as_str();
        let password = conn_config.password.as_str();

        let mut config = if let Some(ado_string) = ado {
            Config::from_ado_string(ado_string.as_str())?
        } else {
            Config::new()
        };

        config.host(host);
        config.port(port);
        config.database(database);
        config.authentication(AuthMethod::sql_server(username, password));
        config.encryption(tiberius::EncryptionLevel::NotSupported);

        Result::Ok(config)
    }

    fn bind_query(query: &mut Query<'_>, bind_var: &DataType) {
        match bind_var {
            DataType::U8(val) => query.bind(val.to_owned()),
            DataType::I16(val) => query.bind(val.to_owned()),
            DataType::I32(val) => query.bind(val.to_owned()),
            DataType::I64(val) => query.bind(val.to_owned()),
            DataType::F32(val) => query.bind(val.to_owned()),
            DataType::F64(val) => query.bind(val.to_owned()),
            DataType::Bool(val) => query.bind(val.to_owned()),
            DataType::String(val) => query.bind(val.to_owned()),
            DataType::Bytes(val) => query.bind(val.to_owned()),
            DataType::DateTime(val) => query.bind(Some(val.to_owned())),

            //NOTE: following types are not supported in tiberius. So, they are converted to supported types.
            DataType::U16(val) => query.bind(val.to_owned() as i32),
            DataType::U32(val) => query.bind(val.to_owned() as i64),
            DataType::U64(val) => query.bind(val.to_owned() as i64),
            _ => {}
        }
    }

    fn col_meta(meta: Option<&ResultMetadata>) -> Option<Vec<String>> {
        let columns = if let Some(meta) = meta {
            let columns = meta.columns();
            Some(columns)
        } else {
            None
        };

        let mut col_type = Vec::new();
        for col in columns? {
            col_type.push(col.to_owned().name().to_string());
        }

        Some(col_type)
    }

    fn col_to_cell(col: ColumnData<'static>) -> Option<DataType> {
        match col {
            ColumnData::U8(val) => val.map(DataType::U8),
            ColumnData::I16(val) => val.map(DataType::I16),
            ColumnData::I32(val) => val.map(DataType::I32),
            ColumnData::I64(val) => val.map(DataType::I64),
            ColumnData::F32(val) => val.map(DataType::F32),
            ColumnData::F64(val) => val.map(DataType::F64),
            ColumnData::Bit(val) => val.map(DataType::Bool),
            ColumnData::String(val) => val.map(|v| DataType::String(v.to_string())),
            ColumnData::Guid(val) => val.map(|v| DataType::String(v.to_string())),
            ColumnData::Binary(value) => value.map(|v| DataType::Bytes(v.to_vec())),
            ColumnData::Numeric(val) => val.map(|v| DataType::I128(v.value())),

            ColumnData::DateTimeOffset(val) => val.map(|v| DataType::I16(v.offset())),
            ColumnData::DateTime(val) => val.map(|v| {
                let days = v.days();
                let seconds_frag = v.seconds_fragments();
                let criteria_date_time = DateTime::parse_from_rfc3339("1900-01-01T00:00:00Z")
                    .expect("Criteria date setting error");
                let date = criteria_date_time + Duration::days(days as i64);
                let millis = seconds_frag * 10 / 3;
                let date_time = date + Duration::milliseconds(millis as i64);
                DataType::DateTime(date_time)
            }),
            ColumnData::Time(val) => val.map(|v| DataType::U64(v.increments())),
            ColumnData::Date(val) => val.map(|v| DataType::U32(v.days())),
            _ => {
                //TODO: ColumnData::Xml, ColumnData::DateTimeOffset, ColumnData::Time, ColumnData::Date should be handled.
                None
            }
        }
    }
}

impl IntoDataRecord for Row {
    fn into_data_record(self) -> DataRecord {
        let cells = self.into_iter().map(SqlServerHandler::col_to_cell).collect();
        DataRecord { cells }
    }
}

pub struct QueryAffectedRows {
    affected_rows: u64,
}

impl QueryResult for QueryAffectedRows {
    fn affected_rows(&self) -> u64 {
        self.affected_rows
    }
}

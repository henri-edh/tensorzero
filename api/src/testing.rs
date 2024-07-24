#![cfg(test)]
use crate::config_parser::Config;
use crate::{api_util::AppStateData, clickhouse::ClickHouseConnectionInfo};
use std::sync::Arc;

pub fn get_unit_test_app_state_data(config: Config) -> AppStateData {
    let http_client = reqwest::Client::new();
    let clickhouse_connection_info = ClickHouseConnectionInfo::new("", true, Some(true));
    AppStateData {
        config: Arc::new(config),
        http_client,
        clickhouse_connection_info,
    }
}

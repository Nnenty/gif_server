use std::collections::HashMap;

use crate::tenor::Results;

use super::{constants::TENOR_RANDOM_QUERY_ADDRESS, errors::QueryError};

pub async fn search_gif(params: &HashMap<String, String>) -> Result<Results, QueryError> {
    let key_name = params.keys().next().expect("parameter should be set");
    let query_param: &str = params
        .get(key_name)
        .expect("parameter value should be set")
        .as_str();

    let resp = match reqwest::Client::builder()
        .build()
        .map_err(|err| -> QueryError { err.into() })?
        .get(TENOR_RANDOM_QUERY_ADDRESS)
        .query(&[("q", query_param), ("key", "LIVDSRZULELA"), ("limit", "6")])
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err.into()),
    };

    let resp_json = resp.text().await?;
    let tenor_results: Results = match serde_json::from_str(&resp_json) {
        Ok(value) => value,
        Err(err) => return Err(err.into()),
    };

    Ok(tenor_results)
}

use crate::tenor::Results;

use super::{constants::TENOR_RANDOM_QUERY_ADDRESS, errors::QueryError};

pub async fn random_cat_gif_query() -> Result<Results, QueryError> {
    let resp = match reqwest::Client::builder()
        .build()
        .map_err(|err| -> QueryError { err.into() })?
        .get(TENOR_RANDOM_QUERY_ADDRESS)
        .query(&[("q", "cat"), ("key", "LIVDSRZULELA"), ("limit", "1")])
        .send()
        .await
    {
        Ok(response) => response,
        Err(err) => return Err(err.into()),
    };

    let resp_json = resp.text().await.unwrap();
    let tenor_results: Results = match serde_json::from_str(&resp_json) {
        Ok(value) => value,
        Err(err) => return Err(err.into()),
    };

    Ok(tenor_results)
}

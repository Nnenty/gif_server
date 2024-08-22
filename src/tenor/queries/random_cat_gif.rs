use crate::tenor::Results;
use rand::{self, Rng};

use super::{constants::TENOR_RANDOM_QUERY_ADDRESS, errors::QueryError};

pub async fn random_cat_gif_query() -> Result<Results, QueryError> {
    let rand_query_range = rand::thread_rng().gen_range(0..14);

    let query = match rand_query_range {
        0 => "cat",
        1 => "cat+sleep",
        2 => "cat+meow",
        3 => "cat+cute",
        4 => "cool+cat",
        5 => "cat+meme",
        6 => "cat+play",
        7 => "cat+eat",
        8 => "cat+lying",
        9 => "cat+sit",
        10 => "cat+run",
        11 => "cat+licking",
        12 => "cat+luna",
        13 => "cat+drink",
        14 => "cat+unico",
        _ => "cat",
    };

    let resp = match reqwest::Client::builder()
        .build()
        .map_err(|err| -> QueryError { err.into() })?
        .get(TENOR_RANDOM_QUERY_ADDRESS)
        .query(&[("q", query), ("key", "LIVDSRZULELA"), ("limit", "1")])
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

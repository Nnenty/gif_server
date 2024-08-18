use reqwest::StatusCode;

use crate::tenor::TenorResults;

const TENOR_RANDOM_QUERY_ADDRESS: &str = "https://g.tenor.com/v1/random";

fn addr_parameters(addr: &str, query: &str, key: &str, limit: i32) -> String {
    format!("{}?q={}&key={}&limit={}", addr, query, key, limit)
}

pub async fn random_cat_gif_query() -> Result<TenorResults, (StatusCode, String)> {
    let addr = addr_parameters(TENOR_RANDOM_QUERY_ADDRESS, "cat", "LIVDSRZULELA", 1);

    let resp = match reqwest::get(addr).await {
        Ok(response) => response,
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Server error: {err}"),
            ))
        }
    };

    let resp_json = resp.text().await.unwrap();
    let tenor_results: TenorResults = match serde_json::from_str(&resp_json) {
        Ok(value) => value,
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Server error: {err}"),
            ))
        }
    };

    Ok(tenor_results)
}

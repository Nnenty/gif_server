use std::collections::HashMap;

use axum::{
    extract::Query,
    response::{Html, IntoResponse},
};
use reqwest::StatusCode;
use tracing::debug;

use crate::{
    presentation::html::get_search_gif_html, tenor::queries::search_gif as search_gif_query,
};

pub async fn search_gif(
    Query(params): Query<HashMap<String, String>>,
) -> Result<Html<String>, impl IntoResponse> {
    debug!("Client connected to `/search_gif`");

    let search_gif_html_without_params: Html<String> =
        match get_search_gif_html(None, None, None).await {
            Ok(search_gif_html) => search_gif_html,
            Err(err) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Server error: {err}"),
                ))
            }
        };
    if params.keys().next().is_none() {
        return Ok(search_gif_html_without_params);
    }
    if params
        .values()
        .next()
        .is_some_and(|param| param.as_str().cmp("").is_eq())
    {
        return Ok(search_gif_html_without_params);
    }

    let response = match search_gif_query(&params).await {
        Ok(home_html) => home_html,
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Server error: {err}"),
            ))
        }
    };

    let search_gif_html: Html<String> = match get_search_gif_html(
        Some(response.get_all_gifs_url()),
        Some(response.get_all_gifs_description()),
        // unwrap because we checked for the presence of the parameter above
        Some(params.values().next().unwrap()),
    )
    .await
    {
        Ok(search_gif_html) => search_gif_html,
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Server error: {err}"),
            ))
        }
    };

    Ok(search_gif_html)
}

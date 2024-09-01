use serde::Deserialize;

use super::media_format::Media;

#[derive(Deserialize, Debug)]
struct Result {
    content_description: String,
    media: Media,
}

#[derive(Deserialize, Debug)]
pub struct Results {
    results: Vec<Result>,
    next: String,
}

impl Results {
    pub fn get_first_gif_url(&self) -> &str {
        &self.results[0].media.0[0].gif.url
    }
    pub fn get_first_content_description(&self) -> &str {
        &self.results[0].content_description
    }
    pub fn get_all_gifs_url(&self) -> Vec<&str> {
        let mut gifs = Vec::new();

        self.results
            .iter()
            .enumerate()
            .for_each(|(i, results)| gifs.insert(i, results.media.0[0].gif.url.as_str()));

        gifs
    }
    pub fn get_all_gifs_description(&self) -> Vec<&str> {
        let mut descriptions = Vec::new();

        self.results
            .iter()
            .enumerate()
            .for_each(|(i, results)| descriptions.insert(i, results.content_description.as_str()));

        descriptions
    }
    pub fn get_next_pos(&self) -> &str {
        &self.next
    }
}

#[cfg(test)]
mod _results_tests {
    use crate::tenor::queries::random_cat_gif as random_cat_gif_query;

    #[tokio::test]
    async fn test_get_next_pos() {
        let results = random_cat_gif_query().await.unwrap();

        let next = results.get_next_pos();
        assert_eq!(results.next, next);

        let wrong_next = next.to_string() + "hehehe";
        assert_ne!(results.next, wrong_next);
    }
}

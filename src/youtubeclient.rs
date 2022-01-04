use std::process::exit;

use google_youtube3::{oauth2, YouTube, hyper, hyper_rustls, api::Video};

pub struct YoutubeClient {
    hub: YouTube,
}

impl YoutubeClient {
    pub async fn new() -> Self {
        let secret = oauth2::ApplicationSecret {
            client_id: dotenv::var("CLIENT_ID").unwrap().to_string(),
            project_id: Some(dotenv::var("PROJECT_ID").unwrap().to_string(),),
            auth_uri: dotenv::var("AUTH_URI").unwrap().to_string(),
            token_uri: dotenv::var("TOKEN_URI").unwrap().to_string(),
            auth_provider_x509_cert_url: Some(dotenv::var("AUTH_PROVIDER_CERT_X509").unwrap().to_string(),),
            client_secret: dotenv::var("CLIENT_SECRET").unwrap().to_string(),
            redirect_uris: vec!["urn:ietf:wg:oauth:2.0:oob".to_string(), dotenv::var("REDIRECT_URIS").unwrap().to_string()],
            ..Default::default()
        };
        let auth = oauth2::InstalledFlowAuthenticator::builder(
            secret.clone(),
            oauth2::InstalledFlowReturnMethod::HTTPRedirect,
        )
            .persist_tokens_to_disk("token.json")
            .build().await.unwrap();

        let hub = YouTube::new(hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()), auth.clone());

        Self {
            hub,
        }
    }

    pub async fn search_music(&self, query_name: &str, max_results: u32) -> Vec<Video> {
        let result = self.hub.search().list(&vec!["snippet".to_string()])
             .q(query_name)
             .video_category_id("10") // for music vids only
             .add_type("video") // has to be present id video category is set
             .max_results(max_results)
             .doit().await;

        if let Ok(res) = result {
            let results = res.1.items.iter().next().unwrap().to_vec();
            let ids = results.iter().map(|result| {
                result.clone().id.unwrap().video_id.unwrap().to_string()
            }).collect();
        
            let results = self.get_videos_info(ids).await;
            return results;
        } else {
            println!("Error while getting snippets: {:?}", result);
            exit(1);
        }
    }

    pub async fn get_videos_info(&self, ids: Vec<String>) -> Vec<Video> {
        let mut request = self.hub.videos().list(&vec![
            "contentDetails".to_string(),
            "snippet".to_string(),
            "statistics".to_string(),
        ]);

        for id in ids.iter() {
            request = request.add_id(id);
        }
        
        let result = request.doit().await;

        if let Ok(res) = result {
            return res.1.items.iter().next().unwrap().to_vec();
        } else {
            println!("Error: {:?}", result);
            exit(1);
        }
    }
}
use crate::{
    api::Api,
    helpers::send_req,
    request::{CraiyonRequest, CraiyonResponse},
    utils::{IMAGE_PER_REQUEST, URL_IMAGE},
};

use clap::ValueEnum;
use image::DynamicImage;
use std::{error::Error, fmt::Display, sync::Arc};
use tokio::sync::Mutex;

#[derive(Default, Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Model {
    model: ModelType,
    version: Api,
    api_token: Option<String>,
}

#[allow(dead_code)]
impl Model {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn version(mut self, ver: Api) -> Self {
        self.version = ver;
        self
    }

    pub fn api_token(mut self, api_token: Option<String>) -> Self {
        self.api_token = api_token;
        self
    }

    pub fn model_type(mut self, mod_type: ModelType) -> Self {
        self.model = mod_type;
        self
    }

    pub fn from(model: ModelType, version: Api) -> Self {
        Self {
            model,
            version,
            api_token: None,
        }
    }

    #[allow(dead_code)]
    pub async fn generate_from_prompt(
        &self,
        prompt: &str,
        num_images: usize,
    ) -> Result<Vec<DynamicImage>, Box<dyn Error>> {
        self.generate(prompt, "", num_images).await
    }

    #[allow(dead_code)]
    pub async fn generate(
        &self,
        prompt: &str,
        negative_prompt: &str,
        num_images: usize,
    ) -> Result<Vec<DynamicImage>, Box<dyn Error>> {

       let images = {

            let version = Arc::new(self.version.to_owned()); // FIXME
            let data = self.version.to_response(
                prompt.to_string(),
                negative_prompt.to_string(),
                self.model.to_string(),
            ).into();

            Self::generate_concurrent(version, data, num_images).await?
        };

        Ok(images)
    }

    #[allow(dead_code)]
    async fn generate_concurrent(
        version: Arc<Api>,
        data: Arc<CraiyonRequest>,
        num_images: usize,
    ) -> Result<Vec<DynamicImage>, Box<dyn Error>> {

        let epochs = num_images / IMAGE_PER_REQUEST;
        let mut threads = Vec::with_capacity(epochs + 1); // +1 possible remainder thread
        let image_buf = Arc::new(Mutex::new(Vec::with_capacity(num_images)));

        for _ in 0..epochs {
            let data = data.clone();
            let image_buf = image_buf.clone();
            let version = version.clone();

            threads.push(tokio::spawn(async move {
                let images = Self::generate_api_chunk(*version, data).await.unwrap();
                let mut image_buf = image_buf.lock().await;
                image_buf.extend(images.lock().await.drain(..));
            }));
        }

        let remainder = num_images % IMAGE_PER_REQUEST;

        if remainder != 0 {
            let data = data.clone();
            let image_buf = image_buf.clone();
            let version = version.clone();

            threads.push(tokio::spawn(async move {
                let images = Self::generate_exact(*version, data, remainder)
                    .await
                    .unwrap();

                let mut image_buf = image_buf.lock().await;
                image_buf.extend(images.lock().await.drain(..));
            }));
        }

        for thread in threads {
            thread.await?;
        }

        Ok(image_buf.clone().lock().await.to_vec())
    }

    async fn generate_api_chunk<T>(
        version: T,
        data: Arc<CraiyonRequest>,
    ) -> Result<Arc<Mutex<Vec<DynamicImage>>>, Box<dyn Error>>
    where
        T: AsRef<str>,
    {
        let response = send_req(version.as_ref(), &*data.clone()).await?; // This takes about ~1min
        let res: CraiyonResponse = response.json().await?;
        let image_buf = Arc::new(Mutex::new(Vec::<DynamicImage>::with_capacity(
            IMAGE_PER_REQUEST,
        )));

        let image_urls = res
            .images
            .iter()
            .map(|image| format!("{URL_IMAGE}/{image}"));

        for image_url in image_urls {
            let pixels = reqwest::get(image_url).await?.bytes().await?.to_vec();
            let image = image::load_from_memory(&pixels)?;

            image_buf.clone().lock().await.push(image);
        }

        Ok(image_buf)
    }

    async fn generate_exact<T>(
        version: T,
        data: Arc<CraiyonRequest>,
        num_images: usize,
    ) -> Result<Arc<Mutex<Vec<DynamicImage>>>, Box<dyn Error>>
    where
        T: AsRef<str>,
    {
        let response = send_req(version.as_ref(), &*data.clone()).await?;
        let res: CraiyonResponse = response.json().await?;
        let image_buf = Arc::new(Mutex::new(Vec::<DynamicImage>::with_capacity(
            IMAGE_PER_REQUEST,
        )));

        let image_urls = res
            .images
            .iter()
            .take(num_images)
            .map(|image| format!("{URL_IMAGE}/{image}"));

        for image_url in image_urls {
            let pixels = reqwest::get(image_url).await?.bytes().await?.to_vec();
            let image = image::load_from_memory(&pixels)?;

            image_buf.lock().await.push(image);
        }

        Ok(image_buf)
    }
}

/// Variants of craiyon::Model
#[allow(dead_code)]
#[derive(Debug, Default, Clone, Eq, PartialEq, PartialOrd, Ord, ValueEnum)]
pub enum ModelType {
    Art,
    Drawing,
    Photo,
    #[default]
    General,
}

impl Display for ModelType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        match self {
            ModelType::Art => f.write_str("art"),
            ModelType::Drawing => f.write_str("drawing"),
            ModelType::Photo => f.write_str("photo"),
            ModelType::General => f.write_str("none"),
        }
    }
}

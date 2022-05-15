mod error;
mod manifest;
pub mod token;

use super::registry;
use crate::http;
use crate::result;
use crate::storage;

use serde::Deserialize;
use std::fs;
use std::path;

fn registry1_url(path: &str) -> String {
    format!("https://registry-1.docker.io/v2/{}", path)
}

pub struct DockerHub {
    authorized_token: Option<token::Token>,
}

impl DockerHub {
    pub fn new(authorized_token: Option<token::Token>) -> result::Result<Self> {
        Ok(Self { authorized_token })
    }

    fn manifest(
        &self,
        repositry: &str,
        tag: &str,
        token: &token::Token,
    ) -> result::Result<bytes::Bytes> {
        let url = registry1_url(format!("{}/manifests/{}", repositry, tag).as_str());
        let client = reqwest::blocking::Client::new();
        let resp = client
            .get(url)
            .header(
                "Accept",
                "application/vnd.docker.distribution.manifest.v2+json",
            )
            .header("Authorization", token.to_string())
            .send()?;
        if !resp.status().is_success() {
            return Err(Box::new(http::HttpError {
                status_code: resp.status(),
                message: "cannot fetch manifest from docker hub registry".to_string(),
            }));
        }

        Ok(resp.bytes()?)
    }

    fn blob(
        &self,
        repository: &str,
        digest: &str,
        token: &token::Token,
    ) -> result::Result<bytes::Bytes> {
        let url = registry1_url(format!("{}/blobs/{}", repository, digest).as_str());
        let client = reqwest::blocking::Client::new();
        let resp = client
            .get(url)
            .header("Authorization", token.to_string())
            .send()?;
        if !resp.status().is_success() {
            return Err(Box::new(http::HttpError {
                status_code: resp.status(),
                message: "cannot fetch blobs from docker hub registry".to_string(),
            }));
        }

        Ok(resp.bytes()?)
    }
}

fn bearer_token(repository: &str) -> result::Result<token::Token> {
    let resp = reqwest::blocking::get(format!(
        "https://auth.docker.io/token?scope=repository:{}:pull&service=registry.docker.io",
        repository
    ))?;
    if !resp.status().is_success() {
        return Err(Box::new(http::HttpError {
            status_code: resp.status(),
            message: "cannot fetch bearer token".to_string(),
        }));
    }

    #[derive(Debug, Deserialize)]
    struct Response {
        token: String,
    }
    let resp = resp.json::<Response>()?;
    Ok(token::Token::Bearer(resp.token))
}

fn normalize_repository(repository: &str) -> result::Result<String> {
    let url = format!(
        "https://hub.docker.com/v2/repositories/library/{}",
        repository
    );
    let resp = reqwest::blocking::get(url)?;
    match resp.status() {
        reqwest::StatusCode::OK => Ok(format!("library/{}", repository)),
        reqwest::StatusCode::NOT_FOUND => Ok(repository.to_string()),
        _ => Err(Box::new(error::UnknownImageNameError {
            image_name: repository.to_string(),
        })),
    }
}

impl registry::Registry for DockerHub {
    fn download_base_image(&self, repository: &str, tag: &str) -> result::Result<path::PathBuf> {
        if repository == "scratch" {
            return Err(Box::new(error::ReservedImageError {
                event: "download image".to_string(),
                image_name: repository.to_string(),
            }));
        }
        let repository = normalize_repository(repository)?;
        let token = match &self.authorized_token {
            Some(token) => token.clone(),
            None => bearer_token(repository.as_str())?,
        };
        let manifest_storage = manifest::storage().join(&repository).join(tag);
        fs::create_dir_all(&manifest_storage)?;

        let manifest = self.manifest(repository.as_str(), tag, &token)?;
        fs::write(
            manifest_storage.join(manifest::MANIFEST_FILENAME),
            &manifest,
        )?;
        let manifest: manifest::Manifest = serde_json::from_slice(&manifest)?;

        let blob_storage = storage::blob_storage();
        fs::create_dir_all(&blob_storage)?;
        for layer in manifest.layers {
            let blob = self.blob(repository.as_str(), layer.digest.as_str(), &token)?;
            fs::write(blob_storage.join(layer.digest), blob)?;
        }
        let blob = self.blob(repository.as_str(), manifest.config.digest.as_str(), &token)?;
        fs::write(blob_storage.join(manifest.config.digest), blob)?;

        Ok(manifest_storage)
    }
}

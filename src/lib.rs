use std::process;

use anyhow::{Context, Error, Ok, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};

fn number_each_item(item: (usize, &String)) -> String {
    let mut n = format!("{}. ", item.0 + 1);
    n.push_str(item.1);

    n
}

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    /// Content to be translated.
    contents: Vec<String>,

    /// Source language code. Default: en
    #[arg(short, long)]
    src_lang: Option<String>,

    /// Target language code. Default: zh
    #[arg(short, long)]
    tgt_lang: Option<String>,

    /// Google Cloud project ID. Can be passed via `GCLOUD_PROJECT_ID`.
    ///
    /// Must have enabled the Translation API.
    #[arg(short, long)]
    project_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    source_language_code: String,
    target_language_code: String,
    contents: Vec<String>,
    mime_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslResponse {
    translations: Vec<Translation>,
}

impl TranslResponse {
    pub fn parse(&self) -> String {
        let numbered_transl: Vec<String> = self
            .translations
            .iter()
            .map(|x| x.content())
            .enumerate()
            .map(|x| number_each_item(x))
            .collect();

        numbered_transl.join("\n")
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Translation {
    translated_text: String,
}

impl Translation {
    fn content(&self) -> &String {
        &self.translated_text
    }
}

impl Body {
    pub fn from(args: Args) -> Self {
        Self {
            source_language_code: match args.src_lang {
                Some(code) => code,
                None => String::from("en"),
            },
            target_language_code: match args.tgt_lang {
                Some(code) => code,
                None => String::from("zh"),
            },
            contents: args.contents,
            mime_type: String::from("text/plain"),
        }
    }
}

fn gain_access_token() -> Result<String> {
    let output = process::Command::new("gcloud")
        .arg("auth")
        .arg("print-access-token")
        .output()
        .with_context(|| "Unable to run `gcloud auth print-access-token`.")?;

    if !output.status.success() {
        Err(Error::msg(
            "`gcloud auth print-access-token` executed with failing error code.",
        ))
    } else {
        String::from_utf8(output.stdout).with_context(|| "Unable to decode access token.")
    }
}

pub fn prepare_request(args: &Args) -> Result<reqwest::RequestBuilder> {
    let access_token = gain_access_token()?.trim().to_owned();
    let project_id = match &args.project_id {
        Some(id) => id.to_owned(),
        None => {
            std::env::var("GCLOUD_PROJECT_ID").with_context(|| "GCLOUD_PROJECT_ID not set.")?
        }
    };
    let post_url =
        format!("https://translation.googleapis.com/v3/projects/{project_id}:translateText");

    Ok(reqwest::Client::new()
        .post(post_url)
        .header("Authorization", format!("Bearer {access_token}"))
        .header("x-goog-user-project", project_id)
        .header("Content-Type", "application/json; charset=utf-8"))
}

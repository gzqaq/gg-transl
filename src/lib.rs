use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    /// Content to be translated.
    content: String,

    /// Source language code. Default: en
    #[arg(short, long)]
    src_lang: Option<String>,

    /// Target language code. Default: zh
    #[arg(short, long)]
    tgt_lang: Option<String>,

    /// Google Cloud access token. Can be passed via `GCLOUD_ACCESS_TOKEN`.
    ///
    /// Can be obtained by `gcloud auth print-access-token`
    #[arg(long)]
    access_token: Option<String>,

    /// Google Cloud project ID. Can be passed via `GCLOUD_PROJECT_ID`.
    ///
    /// Must have enabled the Translation API.
    #[arg(short, long)]
    project_id: Option<String>,
}

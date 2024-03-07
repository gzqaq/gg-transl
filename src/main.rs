use std::process;

use clap::Parser;
use gg_transl::{prepare_request, Args, Body, TranslResponse};

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let response = prepare_request(&args)
        .unwrap_or_else(|e| {
            eprintln!("\x1b[1;31mError when creating request!\x1b[0m\n{e}");
            process::exit(1);
        })
        .json(&Body::from(args))
        .send()
        .await
        .unwrap_or_else(|e| {
            eprintln!("\x1b[1;31mCommunication error!\x1b[0m\n{e}");
            process::exit(1)
        });

    let raw_res = response.text().await.unwrap_or_else(|e| {
        eprintln!("\x1b[1;31mDecode error!\x1b[0m\n{e}");
        process::exit(2)
    });
    let transl_res: TranslResponse = serde_json::from_str(&raw_res).unwrap_or_else(|_| {
        eprintln!("\x1b[1;31mNo translation returned!\x1b[0m\nRaw output: {raw_res}");
        process::exit(3)
    });

    println!("{}", transl_res.parse());
}

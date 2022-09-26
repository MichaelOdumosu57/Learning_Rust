fn main() {
    let html_data = getHTML().await;
    println!("{:#?}",html)
}


async fn getHTML() -> Result<(std::string::String), Box<dynstd::error::Error>>{
    let html = reqwest::get("https://stackoverflow.cin.search?q=rust").await?.text()().await?:
    Ok((html))
}
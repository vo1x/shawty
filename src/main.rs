use clap::Parser;
use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::page::ScreenshotParams;
use futures::StreamExt;
use std::error::Error;
use std::time::Duration;
use serde_json;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value="")]
    url: String,

    #[arg(short, long, default_value = "screenshot.png")]
    output: String,

    #[arg(long, default_value = "")]
    keep: String,

    #[arg(long, default_value = "")]
    delete: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    println!("[LOG] Arguments parsed: {:?}", args);

    println!("[LOG] Building standard browser configuration...");
    let browser_config = BrowserConfig::builder()
        .build()
        .expect("Failed to build browser config");
    
    println!("[LOG] Launching browser...");
    let (mut browser, mut handler) = Browser::launch(browser_config).await?;
    println!("[LOG] Browser launched successfully.");

    let _handle = tokio::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                break;
            }
        }
    });

    println!("[LOG] Opening new page and navigating to: {}", &args.url);
    let page = browser.new_page(&args.url).await?;
    println!("[LOG] Navigation successful. Waiting for page to settle...");
    
    tokio::time::sleep(Duration::from_secs(5)).await;

    if !args.keep.trim().is_empty() {
        let keep_selectors: Vec<&str> = args.keep.split(',').map(|s| s.trim()).collect();
        println!("[LOG] Isolating elements with selectors: {:?}", keep_selectors);
        let keep_selectors_js = serde_json::to_string(&keep_selectors)?;

        let delete_selectors: Vec<&str> = args.delete.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
        let delete_selectors_js = serde_json::to_string(&delete_selectors)?;

        let script = format!(r#"
            const selectorsToKeep = {keep_js};
            const elementsToKeep = [];
            for (const selector of selectorsToKeep) {{
                try {{
                    document.querySelectorAll(selector).forEach(el => elementsToKeep.push(el));
                }} catch (e) {{ console.error(`Failed to process keep selector: ${{selector}}`, e); }}
            }}

            if (elementsToKeep.length > 0) {{
                const newBody = document.createElement('body');
                newBody.style.margin = '0';
                newBody.style.padding = '10px';
                newBody.style.backgroundColor = getComputedStyle(document.body).backgroundColor;
                elementsToKeep.forEach(el => newBody.appendChild(el));

                const selectorsToDelete = {delete_js};
                for (const selector of selectorsToDelete) {{
                    try {{
                        newBody.querySelectorAll(selector).forEach(el => el.remove());
                    }} catch (e) {{ console.error(`Failed to process delete selector: ${{selector}}`, e); }}
                }}

                document.body = newBody;
            }}
        "#, keep_js = keep_selectors_js, delete_js = delete_selectors_js);

        page.evaluate(script).await?;
        println!("[LOG] Finished isolating and deleting elements.");
    }

    println!("[LOG] Taking full page screenshot...");
    
    let params = ScreenshotParams::builder()
        .full_page(true)
        .build();

    page.save_screenshot(params, &args.output).await?;
    println!("[LOG] Screenshot taken and saved.");

    println!("[LOG] Successfully captured screenshot and saved it as '{}'", &args.output);

    browser.close().await?;
    println!("[LOG] Browser closed.");

    Ok(())
}

use std::env;
use std::io::{self, Write};
use dotenv::dotenv;
use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("GEMINI_API_KEY").expect("⚠️ GEMINI_API_KEY must be set in your .env file");

    println!("======================================");
    println!(" 🧬 AI Regex Assistant CLI (Day 18) ");
    println!("======================================");
    println!("1. Generate Regex from English");
    println!("2. Explain an existing Regex");
    print!("\nChoose an option (1 or 2): ");
    io::stdout().flush()?;

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    let choice = choice.trim();

    let mut user_input = String::new();
    let prompt_text;

    if choice == "1" {
        print!("\n📝 Describe what you want to match: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut user_input)?;
        
        prompt_text = format!(
            "You are an expert programmer. Generate ONLY the raw regular expression for the following request. Do not include markdown formatting, backticks, or explanations. Request: {}",
            user_input.trim()
        );
    } else if choice == "2" {
        print!("\n🧩 Paste the Regex you want explained: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut user_input)?;
        
        prompt_text = format!(
            "You are an expert programmer. Explain this regular expression clearly and concisely, breaking down what each part does: {}",
            user_input.trim()
        );
    } else {
        println!("❌ Invalid choice. Please run the program again.");
        return Ok(());
    }

    println!("\n🧠 Thinking...\n");

    let client = Client::new();
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-3-flash-preview:generateContent?key={}", api_key);

    let request_body = json!({
        "contents": [{
            "parts": [{"text": prompt_text}]
        }]
    });

    let response = client.post(&url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    if response.status().is_success() {
        let res_json: serde_json::Value = response.json().await?;
        if let Some(text) = res_json["candidates"][0]["content"]["parts"][0]["text"].as_str() {
            if choice == "1" {
                println!("✅ Generated Regex:\n\n{}", text.trim());
            } else {
                println!("✅ Regex Explanation:\n\n{}", text.trim());
            }
        } else {
            println!("❌ Failed to parse the response from the API.");
        }
    } else {
        println!("❌ API Request failed with status: {:?}", response.status());
    }

    Ok(())
}
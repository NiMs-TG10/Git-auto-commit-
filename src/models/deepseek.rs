use isahc::{prelude::*, Request};
use serde_json::{json, Value};
use std::time::Duration;
use yansi::Paint;

use crate::utils::checks::Check;
use crate::utils::config::{get_api_key, load_model_from_pref};
use crate::utils::diff::get_diff;

pub fn deepseek() -> String {
    //checks if env exists
    dotenvy::dotenv().ok();
    let api_url = "https://api.deepseek.com/chat/completions";
    let api_key = get_api_key("deepseek");

    Check::api_key_present(&api_key);
    Check::api_url_present(&api_url);

    let prompt = include_str!("../../assets/prompt.txt");
    let full_diff = get_diff();

    Check::is_prompt_empty(prompt);

    let uri = format!("{}?key={}", api_url, api_key);

    let model = load_model_from_pref(Some("deepseek"));

    let req_body = json!({
      "model": model,
      "stream_options": null,
      "max_tokens" : 200,
      "messages": [
        {"role": "system", "content": prompt},
        {"role": "user", "content": full_diff}
      ],
      "stream": false
    });
    // Why format! is Necessary
    // Rust is very strict about types. "Bearer " is a string slice (&str), which is a static, borrowed string. api_key is likely an owned String. You can't just add them together with a + operator, especially when building a header string.

    // The format! macro is the standard Rust way to create a new String by interpolating (inserting) values into a template. It's safe and efficient because it creates a new String on the heap with the exact capacity needed.
    let response = Request::post(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .timeout(Duration::from_secs(20))
        .body(req_body.to_string())
        .unwrap()
        .send();

    match response {
        Ok(mut res) => match res.text() {
            Ok(res) => {
                let v: Value = serde_json::from_str(&res).unwrap();
                let commit_msg = &v["choices"][0]["message"]["content"];

                let final_msg = commit_msg.to_string();
                let clear_msg = final_msg.trim().trim_matches(|c| c == '"' || c == '\n');
                return clear_msg.to_string();
            }
            Err(e) => {
                println!("{}", e.red());
                return "".to_string();
            }
        },
        Err(e) => {
            println!("{}", e.red());
            return "".to_string();
        }
    }
}

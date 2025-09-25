use yansi::Paint;
pub struct Check {}

impl Check {
    pub fn api_key_present(value: &str) {
        if value.is_empty() {
            println!(
                "{}",
                "either export the key in terminal or define them in .env"
            );
            std::process::exit(1)
            // exit(0) = success , 1 != success
        }
    }

    pub fn api_url_present(value: &str) {
        if value.is_empty() {
            println!(
                "{}",
                "either export the key in terminal or define them in .env"
            );
            std::process::exit(1)
        }
    }

    pub fn is_prompt_empty(value: &str) {
        if value.is_empty() {
            println!("{}", "no prompt found".red());
            std::process::exit(1)
        }
    }

    pub fn is_diff_empty(value: &str) {
        if value.is_empty() {
            println!("{}", "are the stages changed ?".red());
            println!("{}", "try `git add <file_name>`".red());
            std::process::exit(1)
        }
    }
    // serde_json::from_str(&res).unwrap() already implements below "method/functionality" . Hence No need

    // pub fn is_response_empty(value: &str) {
    //     if value.is_empty() {
    //         println!("{}", " no response, might be a server error".red());
    //         std::process::exit(1)
    //     }
    // }

    pub fn is_model_name_empty(value: &str) {
        if value.is_empty() {
            println!("{}", "model_name not found".red());
            std::process::exit(1)
        }
    }
}

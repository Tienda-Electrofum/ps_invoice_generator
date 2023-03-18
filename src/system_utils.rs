use std::env;
pub struct SystemUtils{}

impl SystemUtils {
    pub fn get_environment_variables() -> (String, String){
        let api_url;
        let api_key;
        let _ = match env::var_os("PS_API_URL") {
            Some(api_url_var) => {
                api_url = String::from(api_url_var.to_str().unwrap());
                println!("\"PS_API_URL\" is: {}", api_url);
            }
            None => {
                println!("The variable \"PS_API_URL\"doesn't exist in the system");
                std::process::exit(0);
            }
        };
    
        let _ = match env::var_os("PS_API_KEY") {
            Some(api_url_var) => {
                api_key = String::from(api_url_var.to_str().unwrap());
                println!("\"PS_API_URL\" is: {}", api_key);
            }
            None => {
                println!("The variable \"PS_API_KEY\"doesn't exist in the system");
                std::process::exit(0)
            }
        };
        return (api_url, api_key);
    }
}
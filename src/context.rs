use leptos::logging::error;
use std::env;

use crate::ai_interface::AIInterface;

#[derive(Clone, Debug)]
pub struct DramaStudyToolAppContext {
    pub ai_interface: AIInterface,
}

impl DramaStudyToolAppContext {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        let _ai_api_key = env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
            error!("OPENAI_API_KEY not set");
            String::new()
        });

        let _ai_completions_endpoint_base = env::var("OPENAI_API_URL").unwrap_or_else(|_| {
            error!("OPENAI_API_URL not set");
            String::new()
        });

        let _ai_deployment_id = env::var("OPENAI_API_DEPLOYMENT").unwrap_or_else(|_| {
            error!("OPENAI_API_DEPLOYMENT not set");
            String::new()
        });

        let _ai_api_version = env::var("OPENAI_API_VERSION").unwrap_or_else(|_| {
            error!("OPENAI_API_VERSION not set");
            String::new()
        });

        Self {
            ai_interface: AIInterface::new(),
        }
    }
}

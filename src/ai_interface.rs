use leptos::logging::*;
use az_openai_rs::*;
use anyhow::{anyhow, Result};
use serde_json::json;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VocabularyInfo {
    pub word: String,
    pub translation: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GrammarPointInfo {
    pub name: String,
    pub relevant_text: String,
    pub description: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubtitleTranslationInfo {
    pub vocabulary: Vec<VocabularyInfo>,
    pub translation: String,
    pub grammar_points: Vec<GrammarPointInfo>,
}

#[derive(Clone, Debug)]
pub struct AIInterface;

impl AIInterface {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn translate(text: String) -> Result<SubtitleTranslationInfo> {
        let json_example_string = json!({
            "vocabulary": [
                {
                    "word": "사전",
                    "translation": "dictionary"
                },
                {
                    "word": "못",
                    "translation": "not"
                },
                {
                    "word": "찾아",
                    "translation": "find"
                }
            ],
            "translation": "I can't find the dictionary",
            "grammar_points": [
                {
                    "name": "Object Marker",
                    "relevant_text": "사전을",
                    "description": "The suffix \"을\" is the object marker, indicating that \"dictionary\" is the object of the verb."
                },
                {
                    "name": "Negative Verb Form",
                    "relevant_text": "못 찾아",
                    "description": "\"못\" is used to indicate inability or impossibility, similar to \"cannot\" in English. \"찾아\" is the verb stem of \"찾다,\" which means \"to find.\" Together, \"못 찾아\" means \"cannot find.\""
                }
            ]
        }).to_string();

        let mut messages = Vec::new();
        messages.push((
            "system",
            "You're an agent who helps provide information to language learners who are inputting a specific subtitle of a show they are watching. All of your answers and output will only be in JSON with no prose.",
        ));
        messages.push(("user", "사전을 못 찾아"));
        messages.push(("assistant", &json_example_string));
        messages.push(("user", &text));

        let response = match completions::chat()
            .messages(messages)
            .max_tokens(2000)
            .send().await {
                Ok(response) => response,
                Err(e) => return Err(anyhow!("Error: {:?}", e)),
            };

        let response_text = match response.choices.get(0) {
            Some(choice) => choice.message.content.clone(),
            None => return Err(anyhow!("No response text found")),
        };

        // Convert response_text to SubtitleTranslationInfo
        let subtitle_translation_info: SubtitleTranslationInfo = match serde_json::from_str(&response_text) {
            Ok(info) => info,
            Err(e) => return Err(anyhow!("Error deserializing: {:?}", e)),
        };

        Ok(subtitle_translation_info)
    }
}

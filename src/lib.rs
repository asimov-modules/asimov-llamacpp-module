// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]

use asimov_module::{prelude::*, tracing};
use core::error::Error;
use serde_json::{json, Value};

#[derive(Clone, Debug, bon::Builder)]
#[builder(on(String, into))]
pub struct Options {
    /// Base URL of the llama.cpp server (e.g. http://localhost:8080)
    pub endpoint: String,

    /// Model identifier (e.g. "llama3")
    pub model: String,
}

/// Generate a response using llama.cpp’s OpenAI-compatible endpoint.
/// Example: POST /v1/chat/completions
pub fn generate(input: impl AsRef<str>, options: &Options) -> Result<Vec<String>, Box<dyn Error>> {
    const UA: &str = "asimov-llamacpp-module";
    const CT_JSON: &str = "application/json";

    let req = json!({
        "model": options.model,
        "messages": [{ "role": "user", "content": input.as_ref() }],
        "stream": false
    });

    let agent = ureq::Agent::config_builder()
        .http_status_as_error(false)
        .user_agent(UA)
        .build()
        .new_agent();

    let url = format!("{}/v1/chat/completions", options.endpoint);
    let mut resp = agent
        .post(&url)
        .header("content-type", CT_JSON)
        .send_json(&req)
        .inspect_err(|e| tracing::error!("HTTP request failed: {e}"))?;

    let status = resp.status();
    let body: Value = resp
        .body_mut()
        .read_json()
        .inspect_err(|e| tracing::error!("unable to read HTTP response body: {e}"))?;
    tracing::debug!(status = %status, body = ?body, "llama.cpp response");

    if status.as_u16() / 100 != 2 {
        if let Some(msg) = body["error"]["message"].as_str() {
            return Err(msg.into());
        }
        return Err(format!("HTTP {}", status).into());
    }

    let mut out = Vec::new();
    if let Some(choices) = body["choices"].as_array() {
        for c in choices {
            if let Some(text) = c["message"]["content"].as_str() {
                out.push(text.to_string());
            }
        }
    }

    Ok(out)
}

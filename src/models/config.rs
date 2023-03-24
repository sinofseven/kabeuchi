use serde::{Deserialize, Serialize};

pub const DEFAULT_MODEL: &str = "gpt-3.5-turbo";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Profile {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_profile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_history: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    // pub n: Option<u32>,
    // pub stream: Option<bool>,
    // pub stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    // logit_bias: ??
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl Profile {
    fn merge(&mut self, profile: &Profile) {
        if self.api_key.is_none() && profile.api_key.is_some() {
            self.api_key = profile.api_key.clone();
        }

        if self.use_history.is_none() && profile.use_history.is_some() {
            self.use_history = profile.use_history;
        }

        if self.model.is_none() && profile.temperature.is_some() {
            self.model = profile.model.clone();
        }

        if self.temperature.is_none() && profile.temperature.is_some() {
            self.temperature = profile.temperature;
        }

        if self.top_p.is_none() && profile.top_p.is_some() {
            self.top_p = profile.top_p;
        }

        if self.max_tokens.is_none() && profile.max_tokens.is_some() {
            self.max_tokens = profile.max_tokens;
        }

        if self.presence_penalty.is_none() && profile.presence_penalty.is_some() {
            self.presence_penalty = profile.presence_penalty;
        }

        if self.frequency_penalty.is_none() && profile.frequency_penalty.is_some() {
            self.frequency_penalty = profile.frequency_penalty;
        }

        if self.user.is_none() && profile.user.is_some() {
            self.user = profile.user.clone();
        }
    }

    pub fn resolve(&self, list: &[Profile]) -> Result<Profile, String> {
        let mut result = self.clone();

        if self.api_key.is_none() && self.source_profile.is_none() {
            return Err("no exists, token or source_profile".to_string());
        }

        if self.source_profile.is_some() {
            let mut current_source_profile = self.source_profile.clone();

            while current_source_profile.is_some() {
                let source_profile = &current_source_profile.clone().unwrap();
                let profile = list.iter().find(|p| p.name.eq(source_profile));

                if let Some(profile) = profile {
                    current_source_profile = profile.source_profile.clone();
                    result.merge(profile);
                } else {
                    current_source_profile = None;
                }
            }

            if result.api_key.is_none() {
                return Err("no token after resolving source profile".to_string());
            }
        }

        Ok(result)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub profile: Vec<Profile>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            profile: Vec::new(),
        }
    }

    pub fn load() -> Result<Option<Config>, String> {
        let path = crate::path::get_path_config_file()?;
        if !path.exists() {
            return Ok(None);
        }
        let text = crate::fs::load_text(&path)?;
        toml::from_str(&text).map_err(|e| {
            format!(
                "failed to deserialize config file: path={}, err={}",
                path.display(),
                e
            )
        })
    }

    pub fn save(&self) -> Result<(), String> {
        let path = crate::path::get_path_config_file()?;
        let text = toml::to_string_pretty(&self)
            .map_err(|e| format!("failed to serialize config: {e}"))?;
        crate::fs::save_text(&path, &text)
    }

    pub fn get_profile(
        &self,
        profile_name: &str,
        is_resolve: bool,
    ) -> Result<Option<Profile>, String> {
        for c_profile in &self.profile {
            if c_profile.name == profile_name {
                if is_resolve {
                    let r_profile = c_profile.resolve(&self.profile)?;
                    return Ok(Some(r_profile));
                } else {
                    return Ok(Some(c_profile.clone()));
                }
            }
        }

        Ok(None)
    }

    pub fn upsert_profile(&mut self, profile: Profile) {
        let index = self.profile.iter().position(|p| p.name == profile.name);
        if let Some(index) = index {
            self.profile[index] = profile;
        } else {
            self.profile.push(profile)
        }
    }
}

use crate::metadata::{VaryCondition, VaryInfo};
use std::collections::HashMap;

/// Vary 值结构
#[derive(Debug, Clone)]
pub struct VaryValues {
    pub language: Option<String>,
    pub encoding: Option<String>,
    pub user_agent: Option<String>,
    pub custom: HashMap<String, String>,
}

impl VaryValues {
    pub fn new() -> Self {
        Self {
            language: None,
            encoding: None,
            user_agent: None,
            custom: HashMap::new(),
        }
    }
}

impl Default for VaryValues {
    fn default() -> Self {
        Self::new()
    }
}

/// 生成变种键
pub fn generate_variant_key(
    base_key: &str,
    vary_conditions: &[VaryCondition],
    vary_values: &VaryValues,
) -> String {
    let mut parts = vec![base_key.to_string()];
    
    for condition in vary_conditions {
        let value = match condition {
            VaryCondition::AcceptLanguage => vary_values.language.as_ref(),
            VaryCondition::AcceptEncoding => vary_values.encoding.as_ref(),
            VaryCondition::UserAgent => vary_values.user_agent.as_ref(),
            VaryCondition::Custom(key) => vary_values.custom.get(key),
        };
        
        if let Some(v) = value {
            parts.push(format!("{}:{}", condition_to_string(condition), v));
        }
    }
    
    parts.join("|")
}

/// 从 VaryInfo 生成完整的缓存键
pub fn build_cache_key(base_key: &str, vary_info: &VaryInfo) -> String {
    format!("{}:{}", base_key, vary_info.variant_key)
}

fn condition_to_string(condition: &VaryCondition) -> &str {
    match condition {
        VaryCondition::AcceptLanguage => "lang",
        VaryCondition::AcceptEncoding => "enc",
        VaryCondition::UserAgent => "ua",
        VaryCondition::Custom(key) => key,
    }
}

/// 从 HTTP 请求头构建 VaryValues
pub fn build_vary_values_from_headers(headers: &HashMap<String, String>) -> VaryValues {
    VaryValues {
        language: headers.get("accept-language").cloned(),
        encoding: headers.get("accept-encoding").cloned(),
        user_agent: headers.get("user-agent").cloned(),
        custom: headers
            .iter()
            .filter(|(k, _)| k.starts_with("x-vary-"))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect(),
    }
}


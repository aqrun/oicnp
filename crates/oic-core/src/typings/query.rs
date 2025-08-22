use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct RequestContext {
    pub uri: String,
    pub path: String,
    pub path_params: String,
    pub method: String,
    pub data: String,
}

pub enum QueryParam {
    Id(Option<i64>),
    String(Option<String>),
}

// TODO: 实现一个 ValidQueryParam derive
impl QueryParam {
    pub fn has_value(&self) -> bool {
        match self {
            Self::Id(x) => {
                if let Some(x) = x {
                    return x > &0;
                }

                false
            },
            Self::String(x) => {
                if let Some(x) = x {
                    if !x.is_empty() {
                        return true;
                    }
                }

                false
            }
        }
    }
}
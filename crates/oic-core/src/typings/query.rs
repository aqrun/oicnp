

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
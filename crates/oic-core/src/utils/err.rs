use validator::{ValidationErrors, ValidationErrorsKind};
use anyhow::{Result, anyhow};

///
/// 获取第1条错误
/// 
pub fn catch_err(validate_res: Result<(), ValidationErrors>) -> Result<()> {
    if let Err(errs) = validate_res {
        let errs_map = errs.errors();
        
        let mut code = String::from("");
        let mut msg = String::from("");
    
        for (key, value) in errs_map.iter() {
            // 当前处理的字段名
            let field = format!("{key}");
            
            match value {
                ValidationErrorsKind::Field(f) => {
                    if let Some(f) = f.get(0) {
                        code = format!("{}", f.code.clone());
    
                        if let Some(m) = f.message.clone() {
                            msg = format!("{}", m);
                        }
                    }
                },
                _ => {
    
                },
            };

            // 获取任意错误信息错误code就停止
            // 属性指定的错误信息
            if !msg.is_empty() {
                return Err(anyhow!("{}", msg.as_str()));
            }

            return Err(anyhow!("{} {}", field, code));
        }
    }

    Ok(())
}
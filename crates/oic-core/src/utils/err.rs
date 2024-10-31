use validator::{ValidationErrors, ValidationErrorsKind};
use anyhow::{Result, anyhow};

///
/// 获取第1条错误
/// 
pub fn catch_err(validate_res: Result<(), ValidationErrors>) -> Result<()> {
    if let Err(errs) = validate_res {
        let errs_map = errs.errors();
    
        let mut field = String::from("");
        let mut code = String::from("");
        let mut msg = String::from("");
    
        let mut index = 0;
        for (key, value) in errs_map.iter() {
            if index > 0 {
                break;
            }
    
            field = format!("{key}");
            
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
    
            index += 1;
        }
    
        let res_msg = format!("{field} {code} {msg}");

        return Err(anyhow!(res_msg));
    }

    Ok(())
}
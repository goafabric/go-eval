use crate::controller::dto::callee::Callee;

#[derive(Debug, Default)]
pub struct CalleeLogic;

impl CalleeLogic {
    pub fn say_my_name(&self, name: &str) -> Callee {
        Callee {
            id: Some("0".to_string()),
            message: format!("Your name is: {name}"),
        }
    }

    pub fn say_my_other_name(&self, name: &str) -> Callee {
        Callee {
            id: Some("0".to_string()),
            message: format!("Your other name is: {name}"),
        }
    }

    pub fn save(&self, callee: Callee) -> Callee {
        Callee {
            id: Some("0".to_string()),
            message: format!("Storing your message: {}", callee.message),
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OneCase {
    pub feature_name: String,
    pub case_id: String,
    pub case_title: String,
    pub preconditions: String,
    pub steps: String,
    pub postcondition: String,
    pub desired_result: String,
    pub test_methods: String,
    pub use_case_type: String,
    pub can_be_automated: String,
    pub tag: String,
    pub author: String,
    pub product_requirement_id: String,
    pub online_question_id: String,
    pub test_experience_id: String,
    pub use_case_level: String,
    pub notes: String,
}

impl OneCase {
    pub fn field_names() -> Vec<String> {
        let mut field_names = Vec::<String>::new();
        field_names.push("feature_name".to_string());
        field_names.push("case_id".to_string());
        field_names.push("case_title".to_string());
        field_names.push("preconditions".to_string());
        field_names.push("steps".to_string());
        field_names.push("postcondition".to_string());
        field_names.push("desired_result".to_string());
        field_names.push("test_methods".to_string());
        field_names.push("use_case_type".to_string());
        field_names.push("can_be_automated".to_string());
        field_names.push("tag".to_string());
        field_names.push("author".to_string());
        field_names.push("product_requirement_id".to_string());
        field_names.push("online_question_id".to_string());
        field_names.push("test_experience_id".to_string());
        field_names.push("use_case_level".to_string());
        field_names.push("notes".to_string());
        field_names
    }
}

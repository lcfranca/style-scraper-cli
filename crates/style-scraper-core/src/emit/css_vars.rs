use crate::model::tokens::DesignTokens;

pub fn emit_css_vars(_tokens: &DesignTokens) -> String {
    ":root {\n}\n".to_string()
}

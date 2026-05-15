use crate::model::tokens::DesignTokens;

pub fn emit_tailwind_config(_tokens: &DesignTokens) -> String {
    "export default { theme: { extend: {} } };\n".to_string()
}

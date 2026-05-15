use crate::model::output::StyleScraperOutput;

pub fn emit_agent_json(output: &StyleScraperOutput, pretty: bool) -> anyhow::Result<String> {
    if pretty {
        Ok(serde_json::to_string_pretty(output)?)
    } else {
        Ok(serde_json::to_string(output)?)
    }
}

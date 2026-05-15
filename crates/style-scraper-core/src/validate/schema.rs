use crate::model::raw::RawFacts;

pub fn validate_raw_facts(raw: &RawFacts) -> anyhow::Result<()> {
    anyhow::ensure!(
        raw.schema_version == "raw-facts.v1",
        "unsupported RawFacts schema_version: {}",
        raw.schema_version
    );
    anyhow::ensure!(!raw.url.trim().is_empty(), "RawFacts url is required");
    anyhow::ensure!(
        raw.viewport.width > 0 && raw.viewport.height > 0,
        "RawFacts viewport must have positive width and height"
    );
    anyhow::ensure!(
        !raw.pages.is_empty(),
        "RawFacts must contain at least one page"
    );

    for page in &raw.pages {
        anyhow::ensure!(!page.url.trim().is_empty(), "page url is required");
        for style in &page.cssom.computed_styles {
            anyhow::ensure!(
                page.dom.nodes.iter().any(|node| node.id == style.node_id),
                "computed style references missing node_id {}",
                style.node_id
            );
        }
        for layout_box in &page.layout.boxes {
            anyhow::ensure!(
                page.dom
                    .nodes
                    .iter()
                    .any(|node| node.id == layout_box.node_id),
                "layout box references missing node_id {}",
                layout_box.node_id
            );
        }
    }

    Ok(())
}

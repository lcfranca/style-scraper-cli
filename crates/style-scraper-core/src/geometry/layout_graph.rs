use crate::model::raw::RawFacts;

pub fn node_count(raw: &RawFacts) -> usize {
    raw.pages.iter().map(|page| page.dom.nodes.len()).sum()
}

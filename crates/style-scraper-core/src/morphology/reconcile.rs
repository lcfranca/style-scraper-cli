use crate::model::morphology::{GestaltCluster, Morphology};
use crate::model::raw::RawFacts;
use crate::model::tokens::DesignTokens;
use crate::morphology::{atomic, molecule, organism};

pub fn infer_morphology(
    raw: &RawFacts,
    tokens: &DesignTokens,
    clusters: &[GestaltCluster],
) -> Morphology {
    let atoms = atomic::infer_atoms(raw, tokens);
    let molecules = molecule::infer_molecules(raw, &atoms, clusters);
    let organisms = organism::infer_organisms(raw);
    let pages = organism::page_groups(raw);

    Morphology {
        atoms,
        molecules,
        organisms,
        templates: Vec::new(),
        pages,
        gestalt_clusters: clusters.to_vec(),
    }
}

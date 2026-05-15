use crate::geometry::rect::{center_y, horizontal_gap, vertical_overlap_ratio};
use crate::model::morphology::GestaltCluster;
use crate::model::raw::{RawFacts, RawLayoutBox};

pub fn cluster_layout(raw: &RawFacts) -> Vec<GestaltCluster> {
    let mut clusters = Vec::new();
    for page in &raw.pages {
        let mut boxes: Vec<&RawLayoutBox> = page
            .layout
            .boxes
            .iter()
            .filter(|layout_box| {
                layout_box.visible && layout_box.width > 0.0 && layout_box.height > 0.0
            })
            .collect();
        boxes.sort_by(|left, right| {
            left.y
                .total_cmp(&right.y)
                .then_with(|| left.x.total_cmp(&right.x))
                .then_with(|| left.node_id.cmp(&right.node_id))
        });

        let mut current: Vec<&RawLayoutBox> = Vec::new();
        for layout_box in boxes {
            if current.is_empty() {
                current.push(layout_box);
                continue;
            }
            let last = current[current.len() - 1];
            let aligned = (center_y(last) - center_y(layout_box)).abs() <= 8.0
                || vertical_overlap_ratio(last, layout_box) >= 0.65;
            let close = horizontal_gap(last, layout_box) <= 32.0;
            if aligned && close {
                current.push(layout_box);
            } else {
                push_cluster(&mut clusters, &current);
                current = vec![layout_box];
            }
        }
        push_cluster(&mut clusters, &current);
    }
    clusters
}

fn push_cluster(clusters: &mut Vec<GestaltCluster>, boxes: &[&RawLayoutBox]) {
    if boxes.len() < 2 {
        return;
    }
    let id = format!("cluster:gestalt:{:03}", clusters.len() + 1);
    let node_ids = boxes
        .iter()
        .map(|layout_box| layout_box.node_id.clone())
        .collect();
    let layout_box_ids = boxes
        .iter()
        .map(|layout_box| layout_box.id.clone())
        .collect();
    clusters.push(GestaltCluster {
        id,
        node_ids,
        layout_box_ids,
        proximity_score: 0.72,
        alignment_score: 0.76,
        common_region_score: 0.58,
    });
}

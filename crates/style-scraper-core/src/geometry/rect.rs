use crate::model::raw::RawLayoutBox;

pub fn vertical_overlap_ratio(left: &RawLayoutBox, right: &RawLayoutBox) -> f64 {
    let top = left.y.max(right.y);
    let bottom = (left.y + left.height).min(right.y + right.height);
    let overlap = (bottom - top).max(0.0);
    let min_height = left.height.min(right.height).max(1.0);
    (overlap / min_height).clamp(0.0, 1.0)
}

pub fn horizontal_gap(left: &RawLayoutBox, right: &RawLayoutBox) -> f64 {
    if left.x <= right.x {
        (right.x - (left.x + left.width)).max(0.0)
    } else {
        (left.x - (right.x + right.width)).max(0.0)
    }
}

pub fn center_y(layout_box: &RawLayoutBox) -> f64 {
    layout_box.y + layout_box.height / 2.0
}

use crate::model::raw::RawLayoutBox;

pub fn visible_boxes(boxes: &[RawLayoutBox]) -> impl Iterator<Item = &RawLayoutBox> {
    boxes.iter().filter(|layout_box| layout_box.visible)
}

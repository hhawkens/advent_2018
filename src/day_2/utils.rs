use super::data;

pub fn get_box_ids() -> Vec<String> {
    let box_ids_text = data::BOX_IDS.to_string();
    let box_ids: Vec<String> = box_ids_text.lines().map(|l| l.to_string()).collect();

    box_ids
}

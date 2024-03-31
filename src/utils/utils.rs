

pub fn hide_id_number(id_number: String) -> String {
  let len = id_number.len();
  let mut id = id_number.clone();
  id.truncate(6);
  let (_, end) = id_number.split_at(len - 4);
  id + "********" + end
}
use std::collections::BTreeMap;
use std::collections::HashSet;

#[derive(Clone, Serialize, Deserialize)]
pub struct Index {
  pub key: String,
  pub data: BTreeMap<String, HashSet<String>>,
}

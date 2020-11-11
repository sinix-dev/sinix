use serde::Serialize;

#[derive(Serialize)]
pub struct Reply {
  pub data: String,
}

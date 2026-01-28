use crate::dao::Db;

#[derive(Clone)]
pub struct Context {
  pub db: Db,
}

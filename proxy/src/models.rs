use std::sync::atomic::AtomicBool;

pub struct Backend {
  pub addr: String,
  pub is_alive: AtomicBool
}
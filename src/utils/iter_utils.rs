pub trait IterUtils<T> {
  fn fuse_to_vec(self, fuser: impl Fn(&T, &T) -> Option<T>) -> Vec<T>;
}

impl<T, I: Iterator<Item = T>> IterUtils<T> for I {
  fn fuse_to_vec(self, try_fuse: impl Fn(&T, &T) -> Option<T>) -> Vec<T> {
    let mut fused_items = Vec::new();
    for item in self {
      if fused_items.len() == 0 {
        fused_items.push(item);
      } else {
        if let Some(fused) = try_fuse(fused_items.last().unwrap(), &item) {
          fused_items.pop();
          fused_items.push(fused);
        } else {
          fused_items.push(item);
        }
      }
    }

    fused_items
  }
}

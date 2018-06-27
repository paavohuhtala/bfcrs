pub trait VecUtils<T> {
  fn find_within_indices(
    &self,
    needle: &T,
    indices: impl IntoIterator<Item = usize>,
  ) -> Option<usize>;
  fn find_index(&self, needle: &T, starting_offset: usize) -> Option<usize>;
  fn find_index_backwards(&self, needle: &T, starting_offset: usize) -> Option<usize>;
}

impl<T: PartialEq> VecUtils<T> for Vec<T> {
  fn find_within_indices(
    &self,
    needle: &T,
    indices: impl IntoIterator<Item = usize>,
  ) -> Option<usize> {
    for i in indices {
      match self.get(i) {
        None => {
          return None;
        }
        Some(x) if x == needle => {
          return Some(i);
        }
        Some(_) => {}
      }
    }

    return None;
  }

  fn find_index(&self, needle: &T, starting_offset: usize) -> Option<usize> {
    self.find_within_indices(needle, starting_offset..self.len())
  }

  fn find_index_backwards(&self, needle: &T, starting_offset: usize) -> Option<usize> {
    self.find_within_indices(needle, (0..starting_offset).rev())
  }
}

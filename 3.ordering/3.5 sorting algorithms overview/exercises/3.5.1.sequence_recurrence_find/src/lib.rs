#![allow(dead_code)]

fn check<T>(seq: &[T]) -> Vec<T>
where
T: PartialEq + Clone,
{
  if seq.len() < 2 {
    return Vec::with_capacity(0);
  }
  
  return check_focus(seq);
}

fn check_focus<T>(seq: &[T]) -> Vec<T>
where
T: PartialEq + Clone,
{
  let mut reps = Vec::new();
  
  let mut iter = seq.iter();
  let mut t = iter.next().unwrap();
  let mut should_push = true;
  
  // T: Θ(n-1) = Θ(n)
  // S: Ο(n/2) = Ο(n)
  while let Some(x) = iter.next() {
    if t == x {
      if should_push {
        reps.push(t.clone()); // imagine flat array logic
        should_push = false;
      }
    } else {
      t = x;
      should_push = true;
    }
  }
  
  reps
}

#[cfg(test)]
mod tests_of_units {
  use super::check_focus;
  
  #[test]
  fn has_recurrences() {
    let seq = [1, 3, 4, 7, 7, 8];
    
    assert_eq!([7], check_focus(&seq).leak());
  }
  
  #[test]
  fn has_recurrences2() {
    let a = "a";
    
    let seq = [a, a, "b"];
    
    assert_eq!([a], check_focus(&seq).leak());
  }
  
  #[test]
  fn has_recurrences3() {
    let seq = [1, 3, 4, 7, 7, 8, 9, 9, 9, 9, 10];
    
    assert_eq!([7, 9], check_focus(&seq).leak());
  }
  
  #[test]
  fn has_recurrences4() {
    let a = "a";
    let c = "c";
    
    let seq = [a, a, "b", c, c, c, c, "d"];
    
    assert_eq!([a, c], check_focus(&seq).leak());
  }
  
  #[test]
  fn no_recurrences() {
    let seq = [1, 3, 4, 7, 8];
    
    assert_eq!([0; 0], check_focus(&seq).leak());
  }
  
  #[test]
  fn no_recurrences2() {
    let seq = ["a", "b", "y", "z"];
    assert_eq!([""; 0], check_focus(&seq).leak());
  }
}

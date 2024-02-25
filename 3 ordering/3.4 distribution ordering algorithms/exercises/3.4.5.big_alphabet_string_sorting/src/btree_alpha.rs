use std::collections::BTreeMap;
use std::str;

type AlphaBuckets<'a> = BTreeMap<usize, Vec<&'a [u8]>>;

#[allow(dead_code)]

fn sort(strs: &mut [&str]) {
  if strs.len() < 2 {
    return;
  }
  
  sort_focus(strs);
}

fn sort_focus(strs: &mut [&str]) {
  
  let len_bucs_len = strs.iter().map(|x| x.len()).max().unwrap();
    
  let mut len_bucs = Vec::<Vec<&[u8]>>::with_capacity(len_bucs_len);
    
  for _ in 0..len_bucs_len {
    len_bucs.push(Vec::new())
  }
    
  for s in strs.iter() {
    len_bucs.get_mut(s.len() - 1).unwrap().push(s.as_bytes());
  }
  
  // EXPECTATIONS:
  // 
  // let expect that our "ASCII alphabet" is quite long
  // thousand and thousand of letters available
  // but still they must upheld property that their codepoint
  // fits into usize (index type)
  //
  // this is true for any UTF-8 code point (4 bytes per one at max)
  // but not for user perceived character (grapheme clusters)
  // that are technically unlimited in length
  // (see http://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries)
  // (check https://glitchtextgenerator.com/)
  //
  // let play out that this alphabet has only grapheme clusters equal
  // to code point, for simplicity  
  
  let mut alpha_bucs = AlphaBuckets::new();
  
  let strs_len = strs.len();
    
  let mut output = Vec::<&[u8]>::with_capacity(strs_len);
    
  let mut curr_ix = len_bucs_len - 1;
  loop {
    
    
    for s in &mut len_bucs[curr_ix] {
      to_alpha_bucs(&mut alpha_bucs, s, curr_ix);
    }
    
    for o in output.iter_mut() {
      to_alpha_bucs(&mut alpha_bucs, o, curr_ix);
    }
    
    output.clear();
        
    for k in alpha_bucs.keys() {
      let buc = alpha_bucs.get(k).unwrap();
      
      if buc.len() == 0 {
        continue;
      }
      
      for s in buc.iter() {
        output.push(s);
      }
    }
    
    alpha_bucs.clear();
    
    if curr_ix == 0 {
      break;
    }
    
    curr_ix -= 1;
  }
    
  for i in 0..strs_len {
    strs[i] = str::from_utf8(output[i]).unwrap();
  }
}

fn to_alpha_bucs<'a, 'b>(alpha_bucs: &mut AlphaBuckets<'a>, s: &'a [u8], len_ix: usize) {
  
  let entry = alpha_bucs.entry(s[len_ix] as usize - 32);  
  let buc = entry.or_insert_with(|| Vec::<&[u8]>::new());  
  buc.push(s);
}

#[cfg(test)]
mod tests_of_units {
  use super::*;
  
  #[test]
  fn basic_test() {
    let mut strs = ["bb", "aa", "a", "b"];
    
    let mut criterion = strs.clone();
    criterion.sort();
    
    sort(&mut strs);
    assert_eq!(criterion, strs);
  }
  
  #[test]
  fn load_test() {
    let mut strs = ["dfeau", "aqab", "aaca", "aqaa", "bbbz", "box", "y", "x"];
    
    let mut criterion = strs.clone();
    criterion.sort();
    
    sort(&mut strs);
    assert_eq!(criterion, strs);
  }
  
  #[test]
  #[should_panic(expected = "attempt to subtract with overflow")]
  fn no_supp_for_empty_str() {
    // to add support for empty strings is trivial, so keep it omitted
    let mut test = ["", ""];
    sort(&mut test);
  }
  
  #[test]
  #[should_panic(expected = "attempt to subtract with overflow")]
  fn unsupp_elems_test1() {
    let mut test = [str::from_utf8(&[31]).unwrap(), "aaa"];
    sort(&mut test);
  }
}

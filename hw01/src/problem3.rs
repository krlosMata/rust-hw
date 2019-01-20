/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
  let mut help_vec: Vec<u32> = vec![0;n as usize];
  let mut final_vec: Vec<u32> = Vec::new();
    // Loops from 2 to n
    for i in 2..n {
      if help_vec[i as usize] == 0 {
        if i.pow(2) > 20 {
          help_vec[i as usize] = 1;
          let mut mult = 2*i;
          while mult < n {
            help_vec[mult as usize] = 1;
            mult += i;
          }
        } else {
          break;
        }
      }
    }
  for i in 2..n {
    if help_vec[i as usize] == 0 {
      final_vec.push(i);    
    }
  }
  final_vec
}
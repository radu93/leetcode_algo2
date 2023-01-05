
struct Solution;

impl Solution {
    fn collapse(s: &mut Vec<u8>) -> i32 {
        let mut current: usize = 0;
        let mut forward: usize = 0;

        while forward < s.len() {
            if s[forward] == '#' as u8 {
                current = if current == 0 { 0 } else { current - 1 }
            } else {
                s[current] = s[forward];
                current += 1;
            }
            forward += 1;
        }
        current as i32
    }

    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut a = s.into_bytes();
        let mut b = t.into_bytes();
    
        let la = Solution::collapse(&mut a) as usize;
        let lb = Solution::collapse(&mut b) as usize;

        &a[..la] == &b[..lb]

    }
}

fn main() {
    println!("{:?}", Solution::backspace_compare(String::from("###a############c"), String::from("v")));
}

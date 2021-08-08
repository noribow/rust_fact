struct BigNum {
    len : u32,
    flag : u32,
    m : Vec<u32>,
}

impl BigNum {
    pub fn new() -> BigNum {
        let mut p : Vec<u32> = Vec::with_capacity(256);
        p.resize(256, 0);
        p[0] = 0; 
        BigNum{ len:1, flag:0, m:p}
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_test() {
        let a = BigNum::new();
        assert_eq!(a.len, 1);
        assert_eq!(a.flag, 0);
        assert_eq!(a.m.capacity(), 256);
        assert_eq!(a.m.len(), 256);
        assert_eq!(a.m[0], 0)
    }
}
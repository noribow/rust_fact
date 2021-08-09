struct BigNum {
    len : u32,
    sign : i32,
    m : Vec<u32>,
}

impl BigNum {
    pub fn new() -> BigNum {
        let mut p : Vec<u32> = Vec::with_capacity(256);
        p.resize(256, 0);
        p[0] = 0; 
        BigNum{ len:1, sign:0, m:p}
    }
    pub fn set( &mut self, _a:i32) {
        self.len = 1;
        if _a > 0 {
            self.sign = 1;
            self.m[0] = _a.clone() as u32;
        } else if _a < 0 {
            self.sign = -1;
            self.m[0] = _a.abs() as u32;
        }else {
            self.sign = 0;
            self.m[0] = 0
        }
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
        assert_eq!(a.sign, 0);
        assert_eq!(a.m.capacity(), 256);
        assert_eq!(a.m.len(), 256);
        assert_eq!(a.m[0], 0)
    }
}
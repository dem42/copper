pub fn insertion_sort<T: PartialOrd>(list: &mut [T]) {
    let n = list.len();
    for i in 0..n {
        let mut j = i;        
        while j > 0 && list[j - 1] > list[j] {
            list.swap(j - 1, j);
            j -= 1;
        }
    }
}

pub fn gen_murmur3_f32(x: u32, y: u32, seed: u32) -> f32 {
    let hash = murmur3(x, y, seed);
    const MAX: u32 = 0xffffffff;
    hash as f32 / MAX as f32
}

pub fn murmur3(x: u32, y: u32, seed: u32) -> u32 {
    let mut hash = seed;
    const C1: u32 = 0xcc9e2d51;
    const C2: u32 = 0x1b873593;
    const M: u32 = 5;
    const N: u32 = 0xe6546b64; 
    const R1: usize = 15;
    const R2: usize = 13;
    // murmur takes each 4 byte block packs the bytes into an 32 bit integer
    // then it applies the following steps on it (multiplication, rotation, xoring)
    let mut k1 = x;
    k1 = k1.wrapping_mul(C1);
    k1 = rotl(k1, R1);
    k1 = k1.wrapping_mul(C2);
    hash = hash ^ k1;
    hash = rotl(hash, R2);
    hash = (hash.wrapping_mul(M)).wrapping_add(N);

    // since we want to hash two integers we unroll the murmur loop
    let mut k1 = y;
    k1 = k1.wrapping_mul(C1);
    k1 = rotl(k1, R1);
    k1 = k1.wrapping_mul(C2);
    hash = hash ^ k1;
    hash = rotl(hash, R2);
    hash = (hash.wrapping_mul(M)).wrapping_add(N);

    hash = hash ^ 8;
    hash = hash ^ (hash >> 16);
    hash = hash.wrapping_mul(0x85ebca6b);
    hash = hash ^ (hash >> 13);
    hash = hash.wrapping_mul(0xc2b2ae35);
    hash = hash ^ (hash >> 16);
    hash
}

fn rotl(x: u32, r: usize) -> u32 {
    (x << r) | (x >> (32 - r))
}

#[cfg(test)]
#[macro_use]
pub mod test_utils {

    pub mod test_constants {
        pub const EPS_PRECISE: f32 = 1e-6;
        pub const EPS_MEDIUM: f32 = 1e-5;
        pub const EPS_BAD: f32 = 1e-2;
    }

    macro_rules! assert_f32_eq {
        ($left:expr, $right:expr, $eps:expr) => (assert!(($left - $right).abs() < $eps, format!("Left: {}, Right: {}.", $left, $right)););
        ($left:expr, $right:expr, $eps:expr, $msg:expr) => (assert!(($left - $right).abs() < $eps, format!("{}. Left: {}, Right: {}.", $msg, $left, $right));)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_insertion_sort_1() {
        let mut l = vec![3, 2, 5, 1];
        insertion_sort(&mut l);
        assert_eq!(l[0], 1);
        assert_eq!(l[1], 2);
        assert_eq!(l[2], 3);
        assert_eq!(l[3], 5);
    }

    #[test]
    fn test_insertion_sort_2() {
        let mut l = vec![1];
        insertion_sort(&mut l);
        assert_eq!(l[0], 1);

        let mut l = vec![1.2, 100.3, 20.2];
        insertion_sort(&mut l);
        assert_eq!(l[0], 1.2);
        assert_eq!(l[1], 20.2);
        assert_eq!(l[2], 100.3);
    }

    #[test]
    fn test_murmur_hash() {
        // dcba bits as u32
        let a = 1633837924;
        let hash = murmur3(a, a, 123);
        // you can check online murmurhash3 
        // for example hashing dcba dcba
        assert_eq!(hash, 3442415257);
    }

    #[test]
    fn test_murmur_gen_f32() {
        let a = 1633837924;
        let f32_hash = gen_murmur3_f32(a, a, 123);
        assert_eq!(f32_hash, 3442415257f32 / 4294967295f32);
    }
}
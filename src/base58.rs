pub const BASE58: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
pub const BASE58_LEN: u64 = 58;

// convert a u64 number to a base58 string, guaranteed to be 11 characters long
pub fn from_u64(mut num: u64) -> String {
    let mut result = String::new();

    // ceil(log58(2^64)) = 11
    for _ in 0..11 {
        let remainder = num % BASE58_LEN;
        num = num / BASE58_LEN;
        result.push(BASE58.chars().nth(remainder as usize).unwrap());
    }

    result.chars().rev().collect()
}

// convert a base58 string to a u64 number
pub fn to_u64<T: AsRef<str>>(base58: T) -> u64 {
    base58.as_ref().chars().fold(0, |acc, c| {
        let index = BASE58.find(c).unwrap() as u64;
        acc * BASE58_LEN + index
    })
}

// returns the successor of a base58 string
pub fn next<T: AsRef<str>>(base58: T) -> String {
    from_u64(to_u64(base58) + 1)
}

use std::iter::zip;

pub fn xor_bytes(x: Vec<u8>, y: Vec<u8>) -> Vec<u8> {
    let zipped = zip(x, y);
    zipped.map(|(a, b)| a ^ b).collect::<Vec<u8>>()
}

pub fn xor_bytes_single(x: Vec<u8>, y: u8) -> Vec<u8> {
    x.iter().map(|a| a ^ y).collect::<Vec<u8>>()
}

pub fn repeating_key_xor(input: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    let cycle: Vec<u8> = key.iter().cycle().take(input.len()).cloned().collect();
    xor_bytes(input, cycle)
}

pub fn xor_bytes2<T, S, U>(x: T, y: S) -> U
where
    T: AsRef<[u8]>,
    S: AsRef<[u8]>,
    U: From<Vec<u8>>,
{
    let x_bytes = x.as_ref();
    let y_bytes = y.as_ref();

    let zipped = x_bytes.iter().zip(y_bytes.iter());
    let result = zipped.map(|(&a, &b)| a ^ b).collect::<Vec<u8>>();

    U::from(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comparision_xor_bytes() {
        let a = vec![23, 232, 9, 3, 19, 2, 30, 49, 8, 29];
        let b = vec![143, 54, 9, 13, 9, 1, 4, 49, 8, 244];

        let old = xor_bytes(a.clone(), b.clone());
        let new: Vec<u8> = xor_bytes2(&a, &b);

        assert_eq!(old, new)
    }
}

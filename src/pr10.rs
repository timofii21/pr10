fn rotate(s: &str, n: isize) -> String {
    let len = s.len() as isize;
    if len == 0 {
        return s.to_string();
    }

    let shift = ((n % len) + len) % len;
    let split_index = len as usize - shift as usize;

    format!("{}{}", &s[split_index..], &s[..split_index])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "abcdefgh";
        let shifts = [
            (0,  "abcdefgh"),
            (8,  "abcdefgh"),
            (-8, "abcdefgh"),
            (1,  "habcdefg"),
            (2,  "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10,"cdefghab"),
        ];

        shifts
            .iter()
            .for_each(|(n, exp)|
                assert_eq!(
                    rotate(s, *n),
                    exp.to_string()
                )
            );
    }
}

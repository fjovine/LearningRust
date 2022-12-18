#[allow(dead_code)]

// Trasforma il carattere decimale passato nel numero corrispondente
pub fn ch_to_i(num: char) -> u8 {
    let result = num as u8 - b'0' as u8;
    assert!(result <= 9, "Carattere non numerico");
    result
}

// Trasforma il numero passato nel carattere corrispondente
pub fn i_to_ch(num: u8) -> char {
    assert!(num <= 9, "Richiesto carattere non numerico");
    (num + b'0' as u8) as char
}

/// Raddoppia la cifra passata considerandola in BCD.
//  * `bcd` - numero decimale da raddoppiare
//  * `resto` - resto che viene usato per raddoppiare e
//              aggiornato col valore da usare per il byte sucessivo
pub fn double_bcd(bcd: u8, resto: &mut bool) -> u8 {
    fn double_digit(digit: u8) -> u8 {
        if digit < 5 {
            digit * 2
        } else {
            digit * 2 + 6
        }
    }
    let lsd = double_digit(bcd & 0x0F) + if *resto { 1 } else { 0 };
    *resto = (lsd & 0xF0) != 0;
    let msd = double_digit((bcd & 0xF0) >> 4) + if *resto { 1 } else { 0 };
    *resto = (msd & 0xF0) != 0;
    let result = (msd << 4) + (lsd & 0xF);
    result
}

// somma le coppie di cifre BCD passate considerando il resto
// e ritorna la coppia di cifre, aggiornando il resto.
pub fn sum_bcd(a: u8, b: u8, resto: &mut bool) -> u8 {
    fn sum_nibble(na: u8, nb: u8, resto: &mut bool) -> u8 {
        let mut sum = na + nb + if *resto { 1 } else { 0 };
        if sum >= 10 {
            *resto = true;
            sum % 10  
        } else {
            *resto = false;
            sum
        }
    }

    let lsd = sum_nibble(a & 0xF, b & 0xF, resto);
    let msd = sum_nibble((a & 0xF0) >> 4, (b & 0xF0) >> 4, resto);
    let result = (msd << 4) + (lsd & 0xF);
    result
}

pub fn assert_strings_are_equal(a:&str, b:&str) {
    let eq = a.eq(b);
    if !eq {
        println!("found [{}] expected [{}]", a, b);
        panic!("Strings are not equal");
    }
}

pub fn assert_strings_are_equal_ignoring_case(a:&str, b:&str) {
    let mut eq = a.len() == b.len();
    if eq {
        let a = a.to_lowercase();
        let b = b.to_lowercase();
        eq = a.eq(&b);
    }
    if !eq {
        println!("found [{}] expected [{}]", a, b);
        panic!("Strings are not equal");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_case_doble_bcd(to_double: u8, expected_value: u8, expected_resto: bool) {
        let mut resto = false;
        let doppio = double_bcd(to_double, &mut resto);
        assert_eq!(doppio, expected_value);
        assert_eq!(resto, expected_resto);
    }

    fn check_case_sum_bcd(a: u8, b: u8, expected_value: u8, expected_resto: bool) {
        let mut resto = false;
        let somma = sum_bcd(a, b, &mut resto);
        assert_eq!(somma, expected_value);
        assert_eq!(resto, expected_resto);
    }

    #[test]
    fn double_bcd_works_well() {
        check_case_doble_bcd(0x1, 0x2, false);
        check_case_doble_bcd(0x11, 0x22, false);
        check_case_doble_bcd(0x28, 0x56, false);
        check_case_doble_bcd(0x91, 0x82, true);
    }

    #[test]
    fn sum_bcd_works_well() {
        check_case_sum_bcd(0x1, 0x2, 0x3, false);
        check_case_sum_bcd(0x10, 0x20, 0x30, false);
        check_case_sum_bcd(0x11, 0x22, 0x33, false);
        check_case_sum_bcd(0x9, 0x9, 0x18, false);
        check_case_sum_bcd(0x19, 0x19, 0x38, false);
        check_case_sum_bcd(0x99, 0x1, 0x00, true);
        check_case_sum_bcd(0x99, 0x99, 0x98, true);
    }
}

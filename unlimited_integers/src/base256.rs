#[allow(dead_code)]
use crate::bcd;
use std::cmp;
use std::u64;

/// Interi con un arbitrario numero di cifre significative.
/// Sono memorizzati in parole a 64 bit in complemento a due.
/// Il vettore cife contiene le singole parole che compongono
/// il numero in precisione arbitraria.
/// Le parole componenti sono memorizzate a partire dalla meno significativa
/// Quindi cifre[0] è la parola meno significativa cifre[cifre.len()-1]

pub struct Base256 {
    cifre: Vec<u64>,
}

impl Base256 {
    fn get_cifra(&self, i: usize) -> u64 {
        match self.cifre.get(i) {
            Some(n) => *n as u64,
            None => 0,
        }
    }

    fn set_cifra(self: &mut Self, i: usize, n: u64) {
        if i < self.cifre.len() {
            self.cifre[i] = n;
        } else {
            // TO DO considerare il caso di indirizzo oltre la dimensione
            self.cifre.push(n);
        }
    }

    fn char_div(num: char, resto: &mut u8) -> char {
        let numeratore = (num as u8 - (b'0' as u8)) + (*resto) * 10u8;
        let risultato = numeratore / 2;
        *resto = numeratore % 2;
        (risultato + b'0') as char
    }

    fn divide_by_two(s: &str) -> (u8, String) {
        let mut result = String::new();
        let mut resto: u8 = 0;
        let mut is_first = true;
        for c in s.chars() {
            assert!(
                "0123456789".contains(c),
                "Il numero {} contiene una cifra sbagliata",
                s
            );
            if is_first {
                if c >= '2' {
                    result.push(Base256::char_div(c, &mut resto));
                } else {
                    resto = 1;
                }
                is_first = false;
                continue;
            }
            result.push(Base256::char_div(c, &mut resto));
        }
        (resto, result)
    }

    /// Takst the ones complement of the number, i.e. inverts each bit
    fn ones_complement(&mut self) {
        for i in 0..self.cifre.len() {
            self.cifre[i] = 0xFFFF_FFFF_FFFF_FFFF ^ self.cifre[i];
        }
    }

    // takes the two's complement of the number
    fn twos_complement(&mut self) {
        let mut carry = false;
        for i in 0..self.cifre.len() {
            let mut partial = 0xFFFF_FFFF_FFFF_FFFFu128 ^ self.cifre[i] as u128;
            if (i == 0) || carry {
                partial += 1;
            }

            self.cifre[i] = (0xFFFF_FFFF_FFFF_FFFF & partial) as u64;
            carry = (partial >> 64) > 0;
        }
    }

    /// Returns the least significant u64 digit
    pub fn lsd(&self) -> u64 {
        self.cifre[0]
    }

    /// Retunrs the most significant u64 digit
    pub fn msd(&self) -> u64 {
        assert!(self.cifre.len()>0);
        self.cifre[self.cifre.len() - 1]
    }

    pub fn is_negative(&self) -> bool {
        (self.msd() & 0x8000_0000_0000_0000) != 0
    }

    pub fn new(s: &str) -> Self {
        fn convert_and_push(result: &mut Base256, s: &str) {
            println!("convert_and_push {}",s);
            match u64::from_str_radix(&s, 16) {
                Ok(n) => result.cifre.push(n),
                Err(_) => panic!("The {} number is not hex", s),
            }
        }
        let mut result = Base256 { cifre: Vec::new() };
        let mut cifra_corrente = 0u64;

        let mut numero = String::new();

        let mut is_negative = false;

        if s.starts_with("0x") || s.starts_with("0X") {
            // hexadecimal format
            let mut i = 0;
            numero = String::from(&s[2..s.len()]);
            //println!("numero : {}", numero);
            let mut digits = String::new();
            for c in numero.chars().rev() {
                println!("C:{}", c);
                digits.insert(0, c);
                i += 1;
                if i >= 16 {
                    convert_and_push(&mut result, &digits);
                    digits.clear();
                    i = 0;
                }
            }
            if i > 0 {
                convert_and_push(&mut result, &digits);
            }

            return result;
        }
        if s.starts_with('-') {
            is_negative = true;
            numero = String::from(&s[1..s.len()]);
        } else {
            numero = String::from(s);
        }

        let mut bit_corrente = 0;
        println!("numero {}", numero);
        while numero.len() > 0 {
            let (resto, risultato) = Base256::divide_by_two(&numero);
            if bit_corrente>=64 {
                result.cifre.push(cifra_corrente);
                bit_corrente = 0;
                cifra_corrente = 0;
            }
            if resto > 0 {
                cifra_corrente |= 1 << bit_corrente;
            }
            numero = risultato;
            bit_corrente += 1;
        }
        println!("Qui");
        if bit_corrente > 0 {
            println!("Qui 01");
            result.cifre.push(cifra_corrente);
        }
        println!("Qui 1");
        if is_negative {
            result.twos_complement();
        }
        println!("Qui 2");

        result
    }

    pub fn new_from(other: &Base256) -> Self {
        Base256 {
            cifre: other.cifre.clone()
        }
    }

    /// Sums other to self S = O + S
    pub fn sum(self: &mut Self, other: Base256) {
        let mut partial_sum: u128;
        let mut resto = 0u128;
        let max = cmp::max(self.cifre.len(), other.cifre.len());
        for i in 0..max {
            partial_sum = self.get_cifra(i) as u128 + other.get_cifra(i) as u128 + resto;
            self.set_cifra(i, (partial_sum & 0xFFFFFFFFFFFFFFFF) as u64);
            resto = partial_sum >> 64;
        }
    }

    /// Subtracts other from self S = S - 0
    pub fn sub(self : &mut Self, other: Base256) {
        let mut other = Base256::new_from(&other);
        other.twos_complement();
        println!("oher ");
        other.debug());
        self.sum(other);
    }

    pub fn debug(self) {
        let mut i = 0;
        for cifra in self.cifre.iter() {
            println!("{} {:#016x}", i, cifra);
            i += 1;
        }
    }

    // Ritorna una stringa che è la rappresentazione decimale
    // di self.
    pub fn to_string(&self) -> String {
        fn convert(to_convert : &Base256, is_negative: bool) -> String {
            let mut result = bcd::BCD::new("0");
            let mut potenza_di_due = bcd::BCD::new("1");
            for i in 0..to_convert.cifre.len() {
                let cifra = to_convert.cifre[i];
                for j in 0..64 {
                    if cifra & (1 << j) != 0 {
                        result.sum(&potenza_di_due);
                    }
                    potenza_di_due.double();
                }
            }
            if is_negative {
                let mut result = result.to_string();
                result.insert(0, '-');
                result
            } else {
                result.to_string()
            }
        }
        
        if self.is_negative() {
            let mut c = Base256::new_from(self);
            c.twos_complement();
            convert(&c, true)
        } else {
            convert(self,false)
        }        
    }

    // Ritorna una stringa che è la rappresentazione esadecimale di self
    pub fn to_string_hex(&self) -> String {
        let mut result = String::new();
        for i in 0..self.cifre.len() {
            if i < self.cifre.len()-1 {
                let val = &format!("{:#016x}", self.cifre[i]);
                result.insert_str(0, &val[2..val.len()]);
            } else {
                result.insert_str(0, &format!("{:#x}", self.cifre[i]));               
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::utils;

    fn twos_complement_check_case(a:&str, expected : &str) {
        let mut dut = super::Base256::new(a);
        dut.twos_complement();
        utils::assert_strings_are_equal_ignoring_case(&dut.to_string_hex(), expected);
    }

    #[test]
    fn twos_complement_works_well() {
        twos_complement_check_case("1234", "0xFFFFFFFFFFFFFB2E");
    }
        
    fn ones_complement_check_case(a: &str, expected: &str) {
        let mut dut = super::Base256::new(a);
        dut.ones_complement();
        utils::assert_strings_are_equal_ignoring_case(&dut.to_string_hex(), expected);
    }

    #[test]
    fn ones_complement_works_well() {
        ones_complement_check_case("0xAAAAAAAAAAAAAAAA","0x5555555555555555");
        ones_complement_check_case("0x1AAAAAAAAAAAAAAAA","0xFFFFFFFFFFFFFFFE5555555555555555");   
    }
    
    fn new_check_case(a: &str, expected_sign: bool, expected_result: &str) {
        let a = super::Base256::new(a);
        println!("Is negative {}", a.is_negative());
        assert_eq!(a.is_negative(), expected_sign);
        println!("found {} expected {}", a.to_string(), expected_result);
        assert_eq!(a.to_string().eq(expected_result), true);
    }

    #[test]
    fn new_works_well() {
        new_check_case("1000", false, "1000");
/*        new_check_case("0x8", false, "8");
        new_check_case("0xA", false, "10");
        new_check_case("0x100", false, "256");
        new_check_case("0x10", false, "16");
        new_check_case(
            "0x1234567890123456789012345678901234567890",
            false,
            "103929005307130220006098923584552504982110632080",
        );
        new_check_case("-1000", true, "-1000");
*/
    }
    fn sum_check_case(a: &str, b: &str, expected_result: &str) {
        let mut a = super::Base256::new(a);
        let b = super::Base256::new(b);
        println!("a {}", a.to_string());
        println!("b {}", b.to_string());
        a.sum(b);
        println!("a {}", a.to_string());
        assert_eq!(a.to_string().eq(expected_result), true);
    }

//$$$    #[test]
//$$$    fn sum_works_well() {
//$$$        sum_check_case("1", "1", "2");
//$$$        sum_check_case("1000", "1", "1001");
//$$$        sum_check_case("1", "1000", "1001");
//$$$        sum_check_case("1", "-1000", "-999");
//$$$        sum_check_case("-1", "-1000", "-1001");
//$$$    }

    fn sub_check_case(a:&str, b:&str, expected_result: &str) {
        let mut a = super::Base256::new(a);
        let b = super::Base256::new(b);
        println!("a");
        a.debug();
        //println!("b {}", b.to_string());
        a.sub(b);
        //println!("a {}", a.to_string());
        assert_eq!(a.to_string().eq(expected_result), true);        
    }
    
    #[test]
    fn sub_works_well() {
//        sub_check_case("10","1","9");
//        sub_check_case("100","1","99");
//        sub_check_case("10000","1","9999");
//        sub_check_case("100000000","1","99999999");
//        sub_check_case("10000000000000000","1","9999999999999999");
        sub_check_case("100000000000000000000000000000000","1","99999999999999999999999999999999");
//        sub_check_case("1000000000000000000000000000000000000000","1", "3");
    }
}
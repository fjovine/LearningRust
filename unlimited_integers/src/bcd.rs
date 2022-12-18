use crate::utils;

pub struct BCD {
    // true se il numero è negativo
    is_negative: bool,

    // Memorizza il valore assoluto del numero partendo
    // dal byte meno significiativo
    // Quindi il numero decimale 123456789
    // sarà memorizzato nel vettore bcd come
    // | indirizzo | valore |
    // | ----:|:----:|
    // | 0 | 0x89 |
    // | 1 | 0x76 |
    // | 2 | 0x54 |
    // | 3 | 0x32 |
    // | 4 | 0x01 |
    //
    // Questo si può verificare con il seguente snippet
    //   let bcd = bcd::BCD::new("123456789");
    //   bcd.debug();
    //
    pub bcd: Vec<u8>,
}

impl BCD {
    // ritorna la n.esima coppia di cifre singificative (n=0 sono unità e decine in BCD)
    // se n è maggiore del numero di cifre BCD, allora ritorna 0
    fn get_cifra(&self, n: usize) -> u8 {
        if n < self.bcd.len() {
            self.bcd[n]
        } else {
            0
        }
    }

    pub fn len(&self) -> usize {
        self.bcd.len()
    }

    pub fn new(s: &str) -> Self {
        let mut result = BCD {
            is_negative: false,
            bcd: Vec::new(),
        };

        let mut bcd: u8 = 0;
        let mut shift = 0;

        for c in s.chars().rev() {
            if result.is_negative {
                panic!("La stringa {} non ha un formato valido", s);
            }

            if c == '-' {
                result.is_negative = true;
                break;
            }

            bcd |= utils::ch_to_i(c) << shift;
            if shift == 0 {
                shift = 4;
            } else {
                shift = 0;
                result.bcd.push(bcd);
                bcd = 0;
            }
        }
        if shift == 4 {
            result.bcd.push(bcd);
        }
        result
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        let mut i = self.bcd.len() - 1;
        let mut is_first = true;
        loop {
            let c = self.bcd[i];
            if !is_first || (is_first && (c & 0xF0 != 0)) {
                result.push(utils::i_to_ch((c & 0xF0) >> 4));
            }
            is_first = false;
            result.push(utils::i_to_ch(c & 0xF));
            if i == 0 {
                break;
            }

            i -= 1;
        }
        result
    }

    pub fn debug(&self) {
        for i in self.bcd.iter() {
            println!("{:#02x}", i)
        }
    }

    /// Raddoppia il numero BCD passato.
    pub fn double(&mut self) {
        let mut resto = false;
        for i in 0..self.bcd.len() {
            self.bcd[i] = utils::double_bcd(self.bcd[i], &mut resto);
        }
        if resto {
            self.bcd.push(0x1);
        }
    }

    // Somma i due BCD passati.
    // se self è il numero A e other è il numero B
    // postcondizione di questa funzione è che self = A + B
    pub fn sum(&mut self, other: &BCD) {
        let max_len = std::cmp::max(self.len(), other.len());
        let mut resto = false;
        for i in 0..max_len {
            let res = utils::sum_bcd(self.get_cifra(i), other.get_cifra(i), &mut resto);
            if i >= self.len() {
                self.bcd.push(res);
            } else {
                self.bcd[i] = res;
            }
        }
        if resto {
            self.bcd.push(0x1);
        }
    }
}

#[cfg(test)]
mod tests {
    fn get_cifra_check_case(s: &str, i: usize, expected: u8) {
        let dut = super::BCD::new(s);
        assert_eq!(dut.get_cifra(i), expected);
    }

    #[test]
    fn get_cifra_works_well() {
        get_cifra_check_case("12345", 0, 0x45);
        get_cifra_check_case("12345", 1, 0x23);
        get_cifra_check_case("12345", 2, 0x01);
    }

    fn sum_check_case(a: &str, b: &str, expected: &str) {
        let mut a = super::BCD::new(a);
        let b = super::BCD::new(b);
        a.sum(&b);
        assert_eq!(a.to_string().eq(expected), true);
    }

    #[test]
    fn sum_works_well() {
        sum_check_case("1","1", "2");
        sum_check_case("1","9", "10");
        sum_check_case("9","1", "10");
        sum_check_case("199999","1", "200000");
        sum_check_case("1", "199999", "200000");
    }
}

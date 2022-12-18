// mod base256;

mod bcd;
mod utils;
mod base256;

fn main() {
    let bcd = bcd::BCD::new("12464988461534879846132123456789");
    println!("{}", bcd.to_string());
    /*
    let mut bcd = bcd::BCD::new("1");
    for i in 1..32 {
        bcd.double();
        println!("{} {}", i, bcd.to_string());
    }
    */
    /*
    let mut bcd = bcd::BCD::new("123456789");
    bcd.debug();
    println!("{}", bcd.to_string());
    bcd.double();
    println!("{}", bcd.to_string());
    */

    /*
      let b = base256::Base256::new("100");
      //b.debug();
      let mut c = base256::Base256::new ("10000");
      //c.debug();
      c.sum(b);
      c.debug();
    */
    /*
      while numero.len() > 0 {
        let (resto, risultato) = Base256::divide_by_two(&numero);
        numero = risultato;
        println!("resto {} risultato {}", resto, numero);
      }
    */
    /*
      for n in '0'..='9' {
        let mut resto : u8 = 0;
        let risultato = char_div(n, &mut resto);
        println!("{}/2 = {} resto {}", n, risultato, resto);
      }
    */
}

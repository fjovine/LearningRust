pub struct StringReader {
    current_index : usize,
    content : Vec<char>,
}

impl StringReader {
    pub fn new(s: &str) -> Self {
        let result = StringReader {
            current_index : 0,
            content : s.chars().collect::<Vec<_>>(),
        };

        result
    }

    pub fn print(&self) {
        println!("{:?}",self.content);
    }

    pub fn next(&mut self) -> Option<char> {
        while self.current_index < self.content.len() {
            let result = self.content[self.current_index];
            self.current_index += 1;
            if ! result.is_whitespace() {
                return Some(result);
            }
        }
        None
    }

    // Check if the next character in the stream reader is the passed one.
    // If it is, then returns, otherwise it panics
    pub fn accept(&mut self, ch:char) {
        match self.next() {
            Some(c) => assert_eq!(c, ch),
            None => panic!("Required {} but string ended", ch)
        }
    }

    pub fn get_next_quoted_string(&mut self) -> String {
        let mut result = String::new();
        loop {
            match self.next() {
                Some(c) => 
                    if c != '"' {
                        result.push(c);
                    } else {
                        break;
                    },
                None => break,
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn basic_test() {
        let sr = super::StringReader::new("(\"A\",(\"B\",0,0),(\"C\",(\"D\",0,0),(\"E\",(\"F\",0,0),(\"G\",0,0)))))");
        sr.print();
    }

    #[test]
    fn next_works_well() {
        let mut sr = super::StringReader::new("0123   45 67    89     ");
        let mut n = 0;
        loop {
            match sr.next() {
                Some(c) => { 
                    assert_eq!(c, (n + b'0') as char);
                },
                None => break,
            }
            n += 1;
        }
    }

    #[test]
    fn get_next_quoted_string_works_well() {
        let mut sr = super::StringReader::new("asfdas \"this_is_a_string\" asdfasd");
        loop {
            match sr.next() {
                Some(c) => 
                    if c == '"' {
                        let s = sr.get_next_quoted_string();
                        assert_eq!(s, "this_is_a_string");
                        break;
                    },
                None => break,
            }
        }
    }
}
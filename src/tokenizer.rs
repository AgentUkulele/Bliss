use regex::Regex;

pub struct Lexer {
    source: Vec<String>,
    output: Vec<Token>,
    pos: usize,
}
#[derive(Debug)]
#[allow(non_camel_case_types, dead_code)]
pub enum Token {
    OPEN_MATCH,
    CLOSE_MATCH,
    RANGE(usize, usize),
    COUNT(usize),
    IDENTIFIER(String),
    ERROR,
    EOF,
}

impl Lexer {
    pub fn new (file: String) -> Lexer {
        let src: Vec<String> = file.split(";")
            .into_iter()
            .map(String::from)
            .collect();

        Lexer {
            source: src,
            output: Vec::new(),
            pos: 0
        }
    }

    pub fn init(&mut self) {
        while self.pos < self.source.len() {

        let entries: Vec<String> = self.source[self.pos].to_string()
            .split(" ")
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>();

        self.pos += 1;

        let range_regex = Regex::new(r"\[\d+-\d+\]").unwrap();
        let count_regex = Regex::new(r"\[\d+\]").unwrap();
        let id_regex = Regex::new(r#"".+""#).unwrap();

        for entry in entries {
            if count_regex.is_match(&entry) {
                self.output.push(Token::COUNT(entry.chars().nth(1).unwrap().to_string().parse::<usize>().unwrap()));
                continue;
            } else if range_regex.is_match(&entry) {
                let chars = entry.split("").collect::<Vec<&str>>();
                self.output.push(Token::RANGE(chars[2].parse::<usize>().unwrap(), chars[4].parse::<usize>().unwrap()));
                continue;
            } else if id_regex.is_match(&entry) {
                self.output.push(Token::IDENTIFIER(entry));
                continue;
            }
            match entry.as_ref() {
                "{" =>  { self.output.push(Token::OPEN_MATCH); }
                "}" => { self.output.push(Token::CLOSE_MATCH); }

                _ => {}
            }
        }
    }
        self.output.push(Token::EOF);
    }

    pub fn get_output(&self) -> &Vec<Token> {
        &self.output
    }
    
}



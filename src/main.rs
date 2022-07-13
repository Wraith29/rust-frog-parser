use std::{collections::HashMap, fs, env, process::exit};

enum StrOrInt {
    Str(String),
    Int(i64),
}

impl ToString for StrOrInt {
    fn to_string(&self) -> String {
        match self {
            StrOrInt::Str(str) => str.clone(),
            StrOrInt::Int(n) => n.to_string(),
        }
    }
}

struct LineParser {
    line: String,
}

impl LineParser {
    fn new(line: &str) -> LineParser {
        LineParser {
            line: String::from(line),
        }
    }

    fn parse(&mut self, variables: HashMap<String, StrOrInt>) -> Option<String> {
        let line = if self.line.contains("{|") && self.line.contains("|}") {
            let mut new = String::new();
            let start_idx = self.line.find("{|").unwrap();
            let end_idx = self.line.find("|}").unwrap();
            if start_idx > 0 {
                new.push_str(&self.line[0..start_idx]);
            }

            let key = &self.line[start_idx + 2..end_idx];
            if variables.contains_key(key) {
                new.push_str(variables.get(key).unwrap().to_string().as_str())
            } else {
                new.push_str(key);
            }

            if end_idx < self.line.len() {
                new.push_str(&self.line[end_idx + 2..self.line.len()]);
            }
            new
        } else {
            self.line.clone()
        };

        Some(line.trim().into())
    }
}

struct Parser {
    template: String,
}

impl Parser {
    fn new(template: String) -> Parser {
        Parser { template }
    }

    fn parse(&mut self) -> Option<String> {
        let lines: Vec<&str> = self.template.split('\n').collect();
        let lines: Vec<_> = lines
            .iter()
            .map(|line| {
                let mut vars: HashMap<String, StrOrInt> = HashMap::new();

                vars.insert("my_var".into(), StrOrInt::Str("Hi".into()));
                vars.insert("other_var".into(), StrOrInt::Int(38));
                LineParser::new(line).parse(vars).unwrap()
            })
            .collect();

        Some(lines.join("\n"))
    }
}

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();

    if args.len() <= 0 {
        exit(1);
    }

    let filename = &args[0];

    let templ = fs::read_to_string(filename).unwrap();
    let mut parser = Parser::new(templ);
    let out = parser.parse().unwrap();

    println!("{out}")
}

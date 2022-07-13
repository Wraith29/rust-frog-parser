use std::{collections::HashMap, env, fs, process::exit};

pub enum VarType {
    Str(String),
    Int(i64),
    Arr(Vec<VarType>),
}

impl ToString for VarType {
    fn to_string(&self) -> String {
        match self {
            VarType::Str(str) => str.clone(),
            VarType::Int(n) => n.to_string(),
            VarType::Arr(items) => items.iter().map(|item| item.to_string()).collect(),
        }
    }
}

pub struct LineParser {
    line: String,
}

impl LineParser {
    pub fn new(line: &str) -> LineParser {
        LineParser {
            line: String::from(line),
        }
    }

    pub fn for_loop_get_before(&self) -> Option<String> {
        let for_loop_del_end_idx = self.line.find("!}").unwrap();
        let start_insert_idx = self.line.find("{|").unwrap();
        let before = &self.line[for_loop_del_end_idx + 2 .. start_insert_idx];
        Some(before.into())
    }   

    pub fn for_loop_get_after(&self) -> Option<String> {
        let end_idx = self.line.find("|}").unwrap();
        let end_for_end_idx = self.line.find("{!}").unwrap();
        let after = &self.line[end_idx + 2 .. end_for_end_idx];
        Some(after.into())
    }

    pub fn parse_str_loop(&self, str: &str) -> Option<String> {
        let before = self.for_loop_get_before().unwrap();
        let after = self.for_loop_get_after().unwrap();
        let out: String = str.chars().map(|c| format!("{before}{c}{after}\n")).collect();
        Some(out)
    }

    pub fn parse_int_loop(&self, int: &i64) -> Option<String> {
        let before = self.for_loop_get_before().unwrap();
        let after = self.for_loop_get_after().unwrap();
        let out: String = (0 .. *int).map(|i| format!("{before}{i}{after}\n")).collect();
        Some(out)
    }

    pub fn parse_arr_loop(&self, arr: &[VarType]) -> Option<String> {
        let before = self.for_loop_get_before().unwrap();
        let after = self.for_loop_get_after().unwrap();
        let out: String = arr.iter().map(|item| format!("{before}{}{after}\n", item.to_string())).collect();
        Some(out)
    }

    pub fn parse(&mut self, variables: HashMap<String, VarType>) -> Option<String> {
        if self.line.contains("{!") {
            let split_line: Vec<_> = self.line.split(' ').collect();
            let var_name_vec: Vec<_> = split_line[3].split('!').collect();
            let var_name = var_name_vec[0];

            if !variables.contains_key(var_name) {
                return None;
            }

            return match variables.get(var_name).unwrap() {
                VarType::Str(str) => self.parse_str_loop(str),
                VarType::Int(int) => self.parse_int_loop(int),
                VarType::Arr(arr) => self.parse_arr_loop(arr)
            }
        }

        let line = if self.line.contains("{|") && self.line.contains("|}") {
            self.parse_string(variables)
        } else {
            self.line.clone()
        };

        Some(line.trim().into())
    }

    fn parse_string(&mut self, variables: HashMap<String, VarType>) -> String {
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
    }
}

pub struct Parser {
    template: String,
}

impl Parser {
    pub fn new(template: String) -> Parser {
        Parser { template }
    }

    pub fn parse(&mut self) -> Option<String> {
        let lines: Vec<&str> = self.template.split('\n').collect();
        let mut in_loop = false;
        let mut loop_str = String::new();
        let lines: Vec<_> = lines
            .iter()
            .map(|&line| {
                let mut vars: HashMap<String, VarType> = HashMap::new();
                vars.insert("my_var".into(), VarType::Str("Hi".into()));
                vars.insert("other_var".into(), VarType::Int(38));
                vars.insert(
                    "str_var".into(),
                    VarType::Str("my_str".into())
                );
                vars.insert(
                    "int_var".into(),
                    VarType::Int(4)
                );
                vars.insert(
                    "arr_var".into(),
                    VarType::Arr(vec![
                        VarType::Str("Hello,".into()),
                        VarType::Str("World!".into()),
                    ])
                );

                if !line.contains("{!}") && in_loop {
                    loop_str.push_str(line.trim());
                    return String::new();
                }

                if line.contains("{!") && !in_loop {
                    in_loop = true;
                    loop_str.push_str(line.trim());
                    return String::new();
                }

                if line.contains("{!}") && in_loop {
                    loop_str.push_str(line.trim());
                    in_loop = false;
                    let out = LineParser::new(loop_str.as_str()).parse(vars).unwrap();
                    loop_str = String::new();
                    out
                } else {
                    LineParser::new(line).parse(vars).unwrap()
                }
            })
            .collect();

        Some(lines.join("\n"))
    }
}

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();

    if args.is_empty() {
        exit(1);
    }

    let filename = &args[0];

    let templ = fs::read_to_string(filename).unwrap();
    let mut parser = Parser::new(templ);
    let out = parser.parse().unwrap();

    let outfilename = format!("{filename}.after.html");
    _ = fs::write(outfilename, out);
}

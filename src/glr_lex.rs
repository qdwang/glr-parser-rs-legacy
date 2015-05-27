#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate regex;


use std::collections::HashSet;

use glr_grammar;
use glr_grammar::Atom as Atom;
use glr_grammar::GrammarItem as GrammarItem;
use std::sync::Arc;

use self::regex::Regex;

#[derive(Debug,Clone,Hash,PartialEq,Eq,PartialOrd,Ord)]
pub struct Lex {
    pub atom: Arc<Atom>,
    pub value: Option<Arc<String>>
}

fn escape_re_string(raw: String) -> String {
    let re = Regex::new(r"(?P<c>[\\\.\+\*\?\(\)\|\[\]\{\}\^\$])").unwrap();
    re.replace_all(&raw, "\\$c")
}
fn gen_re(lex_re_string: String, grammar_strings: Vec<String>) -> (Vec<String>, String) {
    let mut re_string: String = String::new();
    let mut tokens: Vec<String> = Vec::new();
    let test_w = Regex::new(r"^\w+$").unwrap();

    let mut added_grammar: HashSet<String> = HashSet::new();
    for item in grammar_strings.iter() {
        if added_grammar.contains(item) {continue}

        if re_string.len() > 0 { re_string.push('|'); }
        re_string.push_str(item);
        if test_w.is_match(item) {
            re_string.push_str("\\b");
        }
        // re_string.push_str(&("(?:".to_string() + item + ")"));

        // tokens.push(item.clone());
        added_grammar.insert(item.clone());
    }
    
    for line in lex_re_string.split("\n") {
        if line.trim().len() == 0 {continue}
        let mut reg: String = String::new();
        let mut index = 0u16;
        for item in line.split("=") {
            if index == 0 {
                tokens.push(item.trim().to_string());
            } else {
                if index > 1 {reg.push_str("=")}
                reg.push_str(item.trim());
            }
            index += 1;
        }
        re_string.push_str(&("|(".to_string() + &reg + ")"));
    }
    
    // println!("{:?}", re_string);
    (tokens, re_string)
}

pub fn gen_lex(program_raw: String, raw_lex_string: String, raw_grammar_string: String) -> (Vec<Arc<Lex>>, Vec<String>){
    let mut ret: Vec<Arc<Lex>> = Vec::new();
    let mut grammar_strings: Vec<String> = Vec::new();
    let re = Regex::new("'[^']+'").unwrap();
    for cap in re.captures_iter(&raw_grammar_string) {
        for val in cap.iter() {
            grammar_strings.push(escape_re_string(val.unwrap().to_string().replace("'", "")));
        }
    }
    let (tokens, re_string) = gen_re(raw_lex_string, grammar_strings);
    
    match Regex::new(&re_string) {
        Err(e) => {panic!("Lex Creating Error...")},
        Ok(ret_re) => {
            for cap in ret_re.captures_iter(&program_raw) {
                let mut index = 0u16;
                let mut val: String = String::new();
                for name in cap.iter() {
                    if index == 0 {index += 1; continue;}
                    if let Some(x) = name {
                        val = x.to_string();
                        break;
                    }
                    index += 1;
                }

                if index as usize == cap.len() {
                    ret.push(Arc::new(Lex {atom: Arc::new(Atom::Terminal(Arc::new(cap.at(0).unwrap().to_string()))), value: None}));
                } else if let Some(token) = tokens.get(index as usize - 1) {
                    ret.push(Arc::new(Lex {atom: Arc::new(Atom::Terminal(Arc::new(token.clone()))), value: Some(Arc::new(val))}));
                }
            }
        }
    }


    (ret, tokens)
}
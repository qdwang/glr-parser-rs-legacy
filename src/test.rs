#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_must_use)]

extern crate regex;

use std::io::prelude::*;
use std::collections::VecDeque;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fs::File;
use std::io;
use std::sync::Arc;
use self::regex::Regex;
use std::path::Path;

pub mod glr_lex;
pub mod glr_grammar;
pub mod glr;

fn inspect_tree(nodes: &mut VecDeque<Box<glr::SyntaxTreeNode>>, indent: &str){
    for item in nodes {
        let node = item;
        match node.value.clone() {
            Some(x) => {
                println!("{0}{1} -> {2}", indent, node.symbol, x)
            }, 
            None => {
                println!("{0}{1}", indent, node.symbol)
            }
        }
        
        match node.children.as_mut() {
            Some(x) => {let mut _x = x; inspect_tree(&mut _x, &(indent.to_string() + "    "))},
            None => {}
        }
    }

}

fn gen_grammar(path: &'static str, terminal_tokens: Vec<String>) -> Vec<Box<glr_grammar::GrammarItem>> {
    let mut v: Vec<u8> = Vec::new();
    match File::open(Path::new(path)) {
        Err(e) => {println!("open err {:?}", e); vec![]} ,
        Ok(mut raw) => match raw.read_to_end(&mut v) {
            Err(e) => {println!("read err {:?}", e); vec![]} ,
            Ok(len) => match String::from_utf8(v) {
                Err(e) => vec![],
                Ok(content) => {glr_grammar::grammar_gen(&content, Rc::new(terminal_tokens)) }
            } 
        }
    }
}

fn test_list_pair() {
    println!("{:?}", "press any key to test list pair parsing");
    let mut read_line = String::new(); 
    io::stdin().read_line(&mut read_line);

    let grammar_raw = "
        Goal = List 
        List = Pair+
        Pair = \'(\' Pair? \')\' 
    ";

    let grammar = glr_grammar::grammar_gen(grammar_raw, Rc::new(vec![]));
    let grammar_hashmap = glr::create_grammars_hashmap(grammar);

    let initial_item = Arc::new(glr::LRItem {
        stacktop: 0,
        terminal: Arc::new(glr_grammar::Atom::Terminal(Arc::new("<EOFMARKER>".to_string()))),
        symbol: Arc::new(glr_grammar::Atom::Symbol(Arc::new("Goal".to_string()))),
        production: vec![Arc::new(glr_grammar::Atom::Symbol(Arc::new("List".to_string())))]
    });

    let (action, goto) = glr::create_table(&grammar_hashmap, initial_item);

    let mut orders = vec![];
    for _ in 0..2 {
        orders.push(Arc::new(glr_lex::Lex {value: Some(Arc::new("lp1".to_string())), atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("(".to_string())))}));
        orders.push(Arc::new(glr_lex::Lex {value: Some(Arc::new("lp2".to_string())), atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("(".to_string())))}));
        orders.push(Arc::new(glr_lex::Lex {value: Some(Arc::new("rp1".to_string())), atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new(")".to_string())))}));
        orders.push(Arc::new(glr_lex::Lex {value: Some(Arc::new("rp2".to_string())), atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new(")".to_string())))}));
        orders.push(Arc::new(glr_lex::Lex {value: Some(Arc::new("lp3".to_string())), atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("(".to_string())))}));
        orders.push(Arc::new(glr_lex::Lex {value: Some(Arc::new("rp3".to_string())), atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new(")".to_string())))}));   
    }
    orders.push(Arc::new(glr_lex::Lex {value: Some(Arc::new("endmark".to_string())), atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("<EOFMARKER>".to_string())))}));

    let mut trees = glr::parse(orders, action, goto);

    for tree in trees {
        let mut m_tree = tree;
        inspect_tree(&mut m_tree, ""); 
        println!("Parse Tree {:?}", "---------------------------------");
    }

}

fn test_non_deterministic(){
    println!("{:?}", "press any key to test non deterministic parsing 1");
    let mut read_line = String::new(); 
    io::stdin().read_line(&mut read_line);

    let grammar = gen_grammar("test_data/non-deterministic.g", vec![]);
    let grammar_hashmap = glr::create_grammars_hashmap(grammar);

    let initial_item = Arc::new(glr::LRItem {
        stacktop: 0,
        terminal: Arc::new(glr_grammar::Atom::Terminal(Arc::new("<EOFMARKER>".to_string()))),
        symbol: Arc::new(glr_grammar::Atom::Symbol(Arc::new("Goal".to_string()))),
        production: vec![
            Arc::new(glr_grammar::Atom::Symbol(Arc::new("S".to_string())))
        ]
    });

    let (action, goto) = glr::create_table(&grammar_hashmap, initial_item);

    let mut orders = vec![];
    orders.push(Arc::new(glr_lex::Lex {value: None, atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("a".to_string())))}));
    orders.push(Arc::new(glr_lex::Lex {value: None, atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("a".to_string())))}));
    orders.push(Arc::new(glr_lex::Lex {value: None, atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("a".to_string())))}));
    orders.push(Arc::new(glr_lex::Lex {value: None, atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("a".to_string())))}));
    orders.push(Arc::new(glr_lex::Lex {value: None, atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("a".to_string())))}));
    orders.push(Arc::new(glr_lex::Lex {value: None, atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("y".to_string())))}));

    orders.push(Arc::new(glr_lex::Lex {value: None, atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("<EOFMARKER>".to_string())))}));
    let mut trees = glr::parse(orders, action, goto);

    for tree in trees {
        let mut m_tree = tree;
        inspect_tree(&mut m_tree, ""); 
        println!("Parse Tree {:?}", "---------------------------------");
    }

}

fn test_non_deterministic2(){
    println!("{:?}", "press any key to test non deterministic parsing 2");
    let mut read_line = String::new(); 
    io::stdin().read_line(&mut read_line);

    let grammar = gen_grammar("test_data/non-deterministic2.g", vec![]);
    println!("{:?}", grammar.clone());
    let grammar_hashmap = glr::create_grammars_hashmap(grammar);

    let initial_item = Arc::new(glr::LRItem {
        stacktop: 0,
        terminal: Arc::new(glr_grammar::Atom::Terminal(Arc::new("<EOFMARKER>".to_string()))),
        symbol: Arc::new(glr_grammar::Atom::Symbol(Arc::new("S".to_string()))),
        production: vec![
            Arc::new(glr_grammar::Atom::Symbol(Arc::new("E".to_string())))
        ]
    });

    let (action, goto) = glr::create_table(&grammar_hashmap, initial_item);

    let mut orders = vec![];
    orders.push(Arc::new(glr_lex::Lex {value: None, atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("d".to_string())))}));
    orders.push(Arc::new(glr_lex::Lex {value: None, atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("+".to_string())))}));
    orders.push(Arc::new(glr_lex::Lex {value: None, atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("d".to_string())))}));
    orders.push(Arc::new(glr_lex::Lex {value: None, atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("+".to_string())))}));
    orders.push(Arc::new(glr_lex::Lex {value: None, atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("d".to_string())))}));

    orders.push(Arc::new(glr_lex::Lex {value: None, atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("<EOFMARKER>".to_string())))}));
    let mut trees = glr::parse(orders, action, goto);

    for tree in trees {
        let mut m_tree = tree;
        inspect_tree(&mut m_tree, ""); 
        println!("Parse Tree {:?}", "---------------------------------");
    }
}

fn test_if_else(){
    println!("{:?}", "press any key to test if else parsing");
    let mut read_line = String::new(); 
    io::stdin().read_line(&mut read_line);

    let grammar = gen_grammar("test_data/if_else.g", vec![]);
    println!("{:?}", grammar.clone());
    let grammar_hashmap = glr::create_grammars_hashmap(grammar);

    let initial_item = Arc::new(glr::LRItem {
        stacktop: 0,
        terminal: Arc::new(glr_grammar::Atom::Terminal(Arc::new("<EOFMARKER>".to_string()))),
        symbol: Arc::new(glr_grammar::Atom::Symbol(Arc::new("Goal".to_string()))),
        production: vec![
            Arc::new(glr_grammar::Atom::Symbol(Arc::new("Stmt".to_string())))
        ]
    });

    let (action, goto) = glr::create_table(&grammar_hashmap, initial_item);

    let mut orders = vec![];
    orders.push(Arc::new(glr_lex::Lex {value: None, atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("if".to_string())))}));
    orders.push(Arc::new(glr_lex::Lex {value: None, atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("expr".to_string())))}));
    orders.push(Arc::new(glr_lex::Lex {value: None, atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("then".to_string())))}));
        orders.push(Arc::new(glr_lex::Lex {value: None, atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("if".to_string())))}));
        orders.push(Arc::new(glr_lex::Lex {value: None, atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("expr".to_string())))}));
        orders.push(Arc::new(glr_lex::Lex {value: None, atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("then".to_string())))}));
        orders.push(Arc::new(glr_lex::Lex {value: None, atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("S".to_string())))}));
        orders.push(Arc::new(glr_lex::Lex {value: None, atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("else".to_string())))}));
        orders.push(Arc::new(glr_lex::Lex {value: None, atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("S".to_string())))}));

    orders.push(Arc::new(glr_lex::Lex {value: None, atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("<EOFMARKER>".to_string())))}));
    let mut trees = glr::parse(orders, action, goto);

    for tree in trees {
        let mut m_tree = tree;
        inspect_tree(&mut m_tree, ""); 
        println!("Parse Tree {:?}", "---------------------------------");
    }
}

fn test_termianl_reduce(){
    println!("{:?}", "press any key to test terminal reduce parsing");
    let mut read_line = String::new(); 
    io::stdin().read_line(&mut read_line);

    let grammar = gen_grammar("test_data/terminal_reduce_test.g", vec![]);
    let grammar_hashmap = glr::create_grammars_hashmap(grammar);

    let initial_item = Arc::new(glr::LRItem {
        stacktop: 0,
        terminal: Arc::new(glr_grammar::Atom::Terminal(Arc::new("<EOFMARKER>".to_string()))),
        symbol: Arc::new(glr_grammar::Atom::Symbol(Arc::new("Goal".to_string()))),
        production: vec![
            Arc::new(glr_grammar::Atom::Symbol(Arc::new("S".to_string())))
        ]
    });

    let (action, goto) = glr::create_table(&grammar_hashmap, initial_item);

    let mut orders = vec![];
    orders.push(Arc::new(glr_lex::Lex {value: None, atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("{".to_string())))}));
    orders.push(Arc::new(glr_lex::Lex {value: None, atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("a".to_string())))}));
    orders.push(Arc::new(glr_lex::Lex {value: None, atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new(":".to_string())))}));
        orders.push(Arc::new(glr_lex::Lex {value: None, atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("a".to_string())))}));
        orders.push(Arc::new(glr_lex::Lex {value: None, atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("}".to_string())))}));

    orders.push(Arc::new(glr_lex::Lex {value: None, atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("<EOFMARKER>".to_string())))}));
    let mut trees = glr::parse(orders, action, goto);

    for tree in trees {
        let mut m_tree = tree;
        inspect_tree(&mut m_tree, ""); 
        println!("Parse Tree {:?}", "---------------------------------");
    }
}

fn test_python(){
    println!("{:?}", "press any key to test python parsing (this will take long time to generate parser table)");
    let mut read_line = String::new(); 
    io::stdin().read_line(&mut read_line);

    let mut v: Vec<u8> = Vec::new();
    let mut python_g: String = String::new();
    let mut python_lex: String = String::new();
    let mut python_p: String = String::new();
    match File::open(Path::new("test_data/python.g")) {
        Err(e) => {},
        Ok(mut raw) => match raw.read_to_end(&mut v) {
            Err(e) => {},
            Ok(len) => match String::from_utf8(v) {
                Err(e) => {},
                Ok(content) => {python_g = content}
            } 
        }
    }
    v = Vec::new();
    match File::open(Path::new("test_data/python.lex")) {
        Err(e) => {},
        Ok(mut raw) => match raw.read_to_end(&mut v) {
            Err(e) => {},
            Ok(len) => match String::from_utf8(v) {
                Err(e) => {},
                Ok(content) => {python_lex = content}
            } 
        }
    }
    v = Vec::new();
    match File::open(Path::new("test_data/python.p")) {
        Err(e) => {},
        Ok(mut raw) => match raw.read_to_end(&mut v) {
            Err(e) => {},
            Ok(len) => match String::from_utf8(v) {
                Err(e) => {},
                Ok(content) => {python_p = content}
            } 
        }
    }

    let (mut lex, terminal_tokens) = glr_lex::gen_lex(python_p, python_lex, python_g);
    lex.push(Arc::new(glr_lex::Lex {value: None, atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("<EOFMARKER>".to_string())))}));
    println!("Lex List: {:?}", lex);

    let grammar = gen_grammar("test_data/python.g", terminal_tokens);

    let grammar_hashmap = glr::create_grammars_hashmap(grammar);

    let initial_item = Arc::new(glr::LRItem {
        stacktop: 0,
        terminal: Arc::new(glr_grammar::Atom::Terminal(Arc::new("<EOFMARKER>".to_string()))),
        symbol: Arc::new(glr_grammar::Atom::Symbol(Arc::new("program".to_string()))),
        production: vec![
            Arc::new(glr_grammar::Atom::Symbol(Arc::new("file_input".to_string())))
        ]
    });

    println!("Generating Action, Goto tables, please wait...(In real world, you can serialize the parsing table to binary file)");
    let (action, goto) = glr::create_table(&grammar_hashmap, initial_item);

    let mut trees = glr::parse(lex, action, goto);

    for tree in trees {
        let mut m_tree = tree;
        inspect_tree(&mut m_tree, ""); 
        println!("Parse Tree {:?}", "---------------------------------");
    }

}


fn test_json(){
    println!("{:?}", "press any key to test json parsing");
    let mut read_line = String::new(); 
    io::stdin().read_line(&mut read_line);

    let mut v: Vec<u8> = Vec::new();
    let mut json_g: String = String::new();
    let mut json_lex: String = String::new();
    let mut json_p: String = String::new();
    match File::open(Path::new("test_data/json.g")) {
        Err(e) => {},
        Ok(mut raw) => match raw.read_to_end(&mut v) {
            Err(e) => {},
            Ok(len) => match String::from_utf8(v) {
                Err(e) => {},
                Ok(content) => {json_g = content}
            } 
        }
    }
    v = Vec::new();
    match File::open(Path::new("test_data/json.lex")) {
        Err(e) => {},
        Ok(mut raw) => match raw.read_to_end(&mut v) {
            Err(e) => {},
            Ok(len) => match String::from_utf8(v) {
                Err(e) => {},
                Ok(content) => {json_lex = content}
            } 
        }
    }
    v = Vec::new();
    match File::open(Path::new("test_data/json.p")) {
        Err(e) => {},
        Ok(mut raw) => match raw.read_to_end(&mut v) {
            Err(e) => {},
            Ok(len) => match String::from_utf8(v) {
                Err(e) => {},
                Ok(content) => {json_p = content}
            } 
        }
    }

    let (mut lex, terminal_tokens) = glr_lex::gen_lex(json_p, json_lex, json_g);
    lex.push(Arc::new(glr_lex::Lex {value: None, atom: Arc::new(glr_grammar::Atom::Terminal(Arc::new("<EOFMARKER>".to_string())))}));

    let grammar = gen_grammar("test_data/json.g", terminal_tokens);

    let grammar_hashmap = glr::create_grammars_hashmap(grammar);

    let initial_item = Arc::new(glr::LRItem {
        stacktop: 0,
        terminal: Arc::new(glr_grammar::Atom::Terminal(Arc::new("<EOFMARKER>".to_string()))),
        symbol: Arc::new(glr_grammar::Atom::Symbol(Arc::new("Goal".to_string()))),
        production: vec![
            Arc::new(glr_grammar::Atom::Symbol(Arc::new("json".to_string())))
        ]
    });

    let (action, goto) = glr::create_table(&grammar_hashmap, initial_item);
    println!("Lex List: {:?}", lex);

    let mut trees = glr::parse(lex, action, goto);

    for tree in trees {
        let mut m_tree = tree;
        inspect_tree(&mut m_tree, ""); 
        println!("Parse Tree {:?}", "---------------------------------");
    }
}


#[test]
fn verbose_test(){
    test_list_pair();
    test_termianl_reduce();
    test_json();
    test_if_else();
    test_non_deterministic();
    test_non_deterministic2();
}
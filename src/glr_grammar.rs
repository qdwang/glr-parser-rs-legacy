#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

use std::rc::{Rc, Weak};
use std::sync::Arc;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

#[derive(Debug,Clone,Hash,PartialEq,Eq,PartialOrd,Ord)]
pub enum Atom {
    Symbol(Arc<String>),
    Terminal(Arc<String>)
}


#[derive(Debug,Clone,Hash,PartialEq,Eq,PartialOrd,Ord)]
pub struct GrammarItem {
    pub symbol: Arc<Atom>,
    pub production: Vec<Arc<Atom>>
}

pub fn get_atom_string(atom: Arc<Atom>) -> String {
    match *atom {Atom::Symbol(ref x) => x.to_string(), Atom::Terminal(ref x) => x.to_string()}
}
pub fn grammar_gen(_grammar_str: &str, terminal_tokens: Rc<Vec<String>>) -> Vec<Box<GrammarItem>>{
    let grammar_str = _grammar_str.to_string() + "\n";
    let mut ret: Vec<Box<GrammarItem>> = vec![];

    let mut grammar_notes: HashMap<Arc<Atom>, Vec<Vec<Arc<Atom>>>> = HashMap::new();
    
    #[derive(Debug,Clone)]
    enum Mode {
        InGroup, InString, Collect, Expect   
    }
    let mut word: String = String::new();
    let mut mode: Mode = Mode::Expect;
    let mut pl_num = 0u8;

    fn plain_last_grammar(ret: &mut Vec<Box<GrammarItem>>, cache: &HashMap<Arc<Atom>, Vec<Vec<Arc<Atom>>>>) {
        // println!("ret - {:?}", ret);
        fn get_from_cache(orig: &Arc<Atom>, entry: &Vec<Vec<Arc<Atom>>>, cache: &HashMap<Arc<Atom>, Vec<Vec<Arc<Atom>>>>) -> Vec<Vec<Arc<Atom>>> {
            println!("get_from_cache start {:?}", entry);

            let mut ret = vec![];
            for entry_item in entry.iter() {
                let mut new_entry: Vec<Vec<Arc<Atom>>> = vec![vec![]];
                
                for atom in entry_item.iter() {
                    if orig != atom {
                        match cache.get(atom) {
                            Some(x) => {
                                let mut _new_entry: Vec<Vec<Arc<Atom>>> = vec![];
                                for new_entry_item in new_entry.iter() {
                                    for _x in x.iter() {
                                        let mut _entry: Vec<Arc<Atom>> = new_entry_item.clone();
                                        _entry.extend(_x.clone());
                                        _new_entry.push(_entry);
                                    }
                                }
                                new_entry = _new_entry;
                            },
                            None => {
                                for index in 0..new_entry.len() {
                                    new_entry.get_mut(index).unwrap().push(atom.clone());
                                }
                            }
                        }   
                    } else {
                        for index in 0..new_entry.len() {
                            new_entry.get_mut(index).unwrap().push(atom.clone());
                        }
                    }
                }

                ret.extend(new_entry);
            }

            println!("get_from_cache {:?}", ret);
            ret
        }
        if ret.len() > 0 {
                        println!("inner - {:?}", 1);
            match ret.pop() {
                None => {},
                Some(mut x) => {
                    let mut _x = x.clone();
                    _x.production = vec![];
                    let mut _xs = vec![_x.clone()];

                    for item in x.production.iter() {
                        println!("item - {:?}", item);
                        match cache.get(item) {
                            Some(entry) => {
                                let mut new_xs = vec![];
                                // not using all plain grammar
                                for entry_item in get_from_cache(item, entry, cache).iter() { 
                                // for entry_item in entry.iter() {
                                    for _x in _xs.iter() {
                                        let mut _new_x = _x.clone();
                                        _new_x.production.extend(entry_item.clone());
                                        new_xs.push(_new_x);
                                    }
                                }
                                _xs = new_xs;
                            },
                            None => {
                                let mut new_xs = vec![];
                                for _x in _xs.iter() {
                                    let mut __x = _x.clone();
                                    __x.production.push(item.clone());
                                    new_xs.push(__x);
                                }
                                _xs = new_xs;
                            }
                        }
                    }

                    for item in _xs.iter() {
                        ret.push(item.clone());
                    }

                }
            }
        }
    }
    for _char in grammar_str.chars() {
        // println!("<-{:?}", (_char, word.clone()));
        match (mode.clone(), _char) {
            (Mode::InGroup, '(') => {
                word.push('(');
                pl_num += 1;
            },
            (Mode::InGroup, ')') => {
                pl_num -= 1;
                
                if pl_num != 0 {word.push(')')}
                else {
                    match ret.pop() {
                        None => {},
                        Some(mut x) => {
                            let inner_symbol = "".to_string() + &get_atom_string(x.clone().symbol) + &x.production.len().to_string() + "~";
                            let inner_gs = grammar_gen(&(inner_symbol.clone() + " = " + &word), terminal_tokens.clone());

                            for item in inner_gs.iter() {
                                match grammar_notes.entry(item.clone().symbol) {
                                    Occupied(_entry) => {
                                        _entry.into_mut().push(item.clone().production);
                                    },
                                    Vacant(_entry) => {
                                        _entry.insert(vec![item.clone().production]);
                                    }
                                }
                            }
                            ret.extend(inner_gs);
                            ret.push(x);
                            word = inner_symbol;
                            mode = Mode::Collect;
                        }
                    }
                }


            },



            (Mode::InString, '\'') => {
                mode = Mode::Collect;
                if word.len() > 0 {
                    match ret.pop() {
                        None => {},
                        Some(mut x) => {
                            x.production.push(Arc::new(Atom::Terminal(Arc::new(word))));
                            ret.push(x);
                        }
                    }
                    word = String::new();
                }
            },


            (Mode::Collect, ' ') | (Mode::Collect, '\r') | (Mode::Collect, '\n') | (Mode::Collect, '\t') => {
                if word.len() > 0 {
                    match ret.pop() {
                        None => {},
                        Some(mut x) => {
                            let push_atom = if terminal_tokens.contains(&word) {
                                Arc::new(Atom::Terminal(Arc::new(word)))
                            } else {
                                Arc::new(Atom::Symbol(Arc::new(word)))
                            };
                            x.production.push(push_atom);
                            // match group_cache.get(&Arc::new(Atom::Symbol(Arc::new(word.clone())))) {
                            //     Some(entry) => {
                            //         for item in entry.iter(){
                            //             x.production.extend(entry.clone());
                            //         }
                            //     },
                            //     None => {
                            //     }
                            // }
                            
                            ret.push(x);
                        }
                    }
                    word = String::new();
                }

                match _char {
                    '\r' | '\n' => {
                        mode = Mode::Expect;
                    },
                    _ => {}
                }
            },
            (Mode::Collect, '(') => {
                pl_num += 1;
                mode = Mode::InGroup;
            },
            (Mode::Collect, '|') | (Mode::Expect, '|') => {
                if word.len() > 0 { // same as above ' ' matching
                    match ret.pop() {
                        None => {},
                        Some(mut x) => {
                            let push_atom = if terminal_tokens.contains(&word) {
                                Arc::new(Atom::Terminal(Arc::new(word)))
                            } else {
                                Arc::new(Atom::Symbol(Arc::new(word)))
                            };
                            x.production.push(push_atom);
                            
                            ret.push(x);
                        }
                    }
                    word = String::new();
                }

                match ret.pop() {
                    None => {},
                    Some(mut x) => {
                        ret.push(x.clone());
                        x.production = vec![];
                        ret.push(x);
                    }
                }
                mode = Mode::Collect;
            },
            (Mode::Collect, '\'') => {
                mode = Mode::InString;
            },
            (Mode::Collect, '+') | (Mode::Collect, '*') | (Mode::Collect, '?') => {
                match ret.pop() {
                    None => {},
                    Some(mut x) => {
                        let new_x_symbol = Arc::new(Atom::Symbol(Arc::new("~".to_string() + &x.production.len().to_string() + &get_atom_string(x.clone().symbol))));

                        let word_push: Arc<Atom>;
                        if word.len() > 0 {
                            let push_atom = if terminal_tokens.contains(&word) {
                                Arc::new(Atom::Terminal(Arc::new(word)))
                            } else {
                                Arc::new(Atom::Symbol(Arc::new(word)))
                            };
                           word_push = push_atom;
                        } else {
                            match x.production.pop() {
                                Some(x) => {
                                    word_push = x;
                                },
                                None => {
                                    word_push = Arc::new(Atom::Terminal(Arc::new("".to_string())));
                                }
                            }
                        }

                        ret.push(Box::new(GrammarItem {symbol: new_x_symbol.clone(), production: vec![word_push.clone()]}));
                        match grammar_notes.entry(new_x_symbol.clone()) {
                            Occupied(_entry) => {
                                _entry.into_mut().push(vec![word_push.clone()]);
                            },
                            Vacant(_entry) => {
                                _entry.insert(vec![vec![word_push.clone()]]);
                            }
                        }
                        if _char == '?' || _char == '*' {
                            ret.push(Box::new(GrammarItem {symbol: new_x_symbol.clone(), production: vec![]}));
                            match grammar_notes.entry(new_x_symbol.clone()) {
                                Occupied(_entry) => {
                                    _entry.into_mut().push(vec![]);
                                },
                                Vacant(_entry) => {
                                    _entry.insert(vec![vec![]]);
                                }
                            }
                        } 

                        if _char == '+' || _char == '*' {
                            ret.push(Box::new(GrammarItem {symbol: new_x_symbol.clone(), production: vec![new_x_symbol.clone(), word_push.clone()]}));
                            match grammar_notes.entry(new_x_symbol.clone()) {
                                Occupied(_entry) => {
                                    _entry.into_mut().push(vec![new_x_symbol.clone(), word_push]);
                                },
                                Vacant(_entry) => {
                                    _entry.insert(vec![vec![new_x_symbol.clone(), word_push]]);
                                }
                            }
                        }
                   

                        x.production.push(new_x_symbol.clone());
                        ret.push(x);
                        word = String::new();
                        // let mut new_x = x.clone();
                        // let new_symbol_atom = Arc::new(Atom::Symbol(Arc::new("~".to_string() + &new_x.clone().production.len().to_string() + &get_atom_string(new_x.clone().symbol))));
                        // new_x.symbol = new_symbol_atom.clone();

                        // let word_push: Arc<Atom>;
                        // if word.len() > 0 {
                        //    word_push = Arc::new(Atom::Symbol(Arc::new(word)));
                        // } else {
                        //     match x.production.pop() {
                        //         Some(x) => {
                        //             new_x.production.pop();
                        //             word_push = x;
                        //         },
                        //         None => {
                        //             word_push = Arc::new(Atom::Terminal(Arc::new("".to_string())));
                        //         }
                        //     }
                        // }

                        // if _char != '+' {ret.push(new_x.clone())}
                        // new_x.production.push(word_push.clone());
                        // ret.push(new_x);

                        // if _char != '?' {
                        //     ret.push(Box::new(GrammarItem {symbol: new_symbol_atom.clone(), production: vec![new_symbol_atom.clone(), word_push]}));
                        // }

                        // x.production = vec![new_symbol_atom];
                        // ret.push(x);

                        // word = String::new();
                    }
                }
        
            },


            (Mode::Expect, '=') => {
                // println!("->{:?}", word);
                // plain_last_grammar(&mut ret, &grammar_notes);
                ret.push(Box::new(GrammarItem {symbol: Arc::new(Atom::Symbol(Arc::new(word))), production: vec![]}));
                word = String::new();
                mode = Mode::Collect;
            },
            (Mode::Expect, ' ') | (Mode::Expect, '\r') | (Mode::Expect, '\n') | (Mode::Expect, '\t') => {
            },

            (m, x) => {
                // println!("{:?}", m);
                word.push(x);
            }
        }
    }

    // plain_last_grammar(&mut ret, &grammar_notes);

    ret
}

fn sequitur(input: Vec<Arc<Atom>>) -> Vec<GrammarItem>{
    #[derive(Debug,Clone,Hash,PartialEq,Eq,PartialOrd,Ord)]
    struct AtomPair(Arc<Atom>, Arc<Atom>);

    fn gen_grammars(prod: Vec<Arc<Atom>>, counter: &mut HashMap<AtomPair, u16>, n_count: &mut Box<u16>) -> (Vec<Arc<Atom>>, Vec<GrammarItem>) {
        let mut new_prod: Vec<Arc<Atom>> = vec![];
        let mut _ret: Vec<GrammarItem> = vec![];
        let mut counter = counter;

        if prod.len() < 2 {
            return (prod, _ret)
        }

        for index in 0..prod.len() - 1 {
            if let (Some(p1), Some(p2)) = (prod.get(index), prod.get(index + 1)) {
                match counter.entry(AtomPair(p1.clone(), p2.clone())) {
                    Occupied(entry) => {
                        if p1 == p2 {continue}

                        let mut count = entry.into_mut();
                        **n_count += 1;
                        *count = **n_count;
                        println!("{:?}", (**n_count, p1.clone(), p2.clone()));
                        _ret.push(GrammarItem {
                            symbol: Arc::new(Atom::Symbol(Arc::new("N".to_string() + &n_count.to_string()))),
                            production: vec![p1.clone(), p2.clone()]
                        });
                    },
                    Vacant(entry) => {
                        entry.insert(1);
                    }
                }
            }
        }

        // for item in counter.iter() {
        //     let symbol_str = match *item.symbol {Atom::Symbol(ref x) => x, Atom::Terminal(ref x) => x};
        //     for index in 0..item.production.len() - 1 {
        //         if let (Some(p1), Some(p2)) = (item.production.get(index), item.production.get(index + 1)) {
        //             match counter.entry(AtomPair(p1.clone(), p2.clone())) {
        //                 Occupied(entry) => {
        //                     println!("o item {:?}", item);
        //                     let mut count = entry.into_mut();
        //                     if let Ok(x) = symbol_str.parse::<u16>() {
        //                         *count = x;
        //                     }
        //                 },
        //                 Vacant(entry) => {
        //                     println!("v item {:?}", item);
        //                     if let Ok(x) = symbol_str.parse::<u16>() {
        //                         entry.insert(x);
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // }

        let mut _pass = false;
        let mut counter_remover: Vec<AtomPair> = vec![];
        for index in 0..prod.len() - 1 {
            if _pass {_pass = false; continue}

            if let (Some(p1), Some(p2)) = (prod.get(index), prod.get(index + 1)) {
                match counter.entry(AtomPair(p1.clone(), p2.clone())) {
                    Occupied(entry) => {
                        let count = entry.get();
                        if *count <= 1 {
                            new_prod.push(p1.clone());
                            if index == prod.len() - 2 {
                                new_prod.push(p2.clone());
                            }
                            counter_remover.push(AtomPair(p1.clone(), p2.clone()));
                        } else {
                            new_prod.push(Arc::new(Atom::Symbol(Arc::new("N".to_string() + &*count.to_string()))));
                            _pass = true;
                        }

                        
                    },
                    Vacant(entry) => {}
                }
            }
        }
        for item in counter_remover.iter() {
            counter.remove(&item);
        }

        // println!("counter {:?}", counter);

        if new_prod == prod {
            (new_prod, _ret)
        } else {
            let (_new_prod, _new_ret) = gen_grammars(new_prod, &mut counter, n_count);
            _ret.extend(_new_ret);
            (_new_prod, _ret)
            // (new_prod, _ret)
        }
    }

    let mut n_count = Box::new(1u16);
    let mut ret: Vec<GrammarItem> = vec![];
    let mut counter: HashMap<AtomPair, u16> = HashMap::new();

    for input_item in input.iter() {
        // let input_item = Arc::new(Atom::Terminal(Arc::new(_char.clone().to_string())));

        let mut prod: Vec<Arc<Atom>> = vec![];
        if ret.len() == 0 {
            prod = vec![input_item.clone()];
            ret.push(GrammarItem {symbol: Arc::new(Atom::Symbol(Arc::new("N1".to_string()))), 
                production: vec![input_item.clone()]});
        } else if let Some(n1) = ret.get_mut(0) {
            n1.production.push(input_item.clone());
            prod = n1.production.clone();
        }

        let (new_prod, new_grammars) = gen_grammars(prod, &mut counter, &mut n_count);
        if let Some(n1) = ret.get_mut(0) {
            n1.production = new_prod;
        }
        ret.extend(new_grammars);
    }

    ret.dedup();
    ret
}
fn main(){
    let test_str = "
        Goal = List 
        List = Pair+
        Pair = \'(\' Pair? \')\' 
    ";

    let test_str2 = "
        A = a c? d* (e (f \'g\')* h)+ i
    ";

    let test_str3 = "
        Goal = Stmt
        Stmt = \'if\' \'expr\' \'then\' Stmt (\'else\' Stmt)?
        Stmt = \'S\'
    ";

    let test_str4 = "
G = file_input

file_input =  simple_stmt 

simple_stmt = expr_stmt ( ';' expr_stmt )* ';'? 'NEWLINE'

expr_stmt = atom ('augassign'('yield_expr'|'testlist')|('='('yield_expr'|atom))*)

atom= 'NAME' | 'SHORT_STRING'

    ";
    // TODO: () * + ?
    let result = grammar_gen(test_str4, Rc::new(vec![]));
    println!("{:?}", result);
    for item in result.iter() {
        println!("{:?}", (item.clone().symbol, item.clone().production));
    }
    // let mut sequitur_input: Vec<Arc<Atom>> = vec![];
    // sequitur_input.push(Arc::new(Atom::Terminal(Arc::new("if".to_string()))));
    // sequitur_input.push(Arc::new(Atom::Terminal(Arc::new("expr".to_string()))));
    // sequitur_input.push(Arc::new(Atom::Terminal(Arc::new("then".to_string()))));
    // sequitur_input.push(Arc::new(Atom::Terminal(Arc::new("if".to_string()))));
    // sequitur_input.push(Arc::new(Atom::Terminal(Arc::new("expr".to_string()))));
    // sequitur_input.push(Arc::new(Atom::Terminal(Arc::new("then".to_string()))));
    // sequitur_input.push(Arc::new(Atom::Terminal(Arc::new("S".to_string()))));
    // sequitur_input.push(Arc::new(Atom::Terminal(Arc::new("else".to_string()))));
    // sequitur_input.push(Arc::new(Atom::Terminal(Arc::new("S".to_string()))));

 
    // let sequitur_result = sequitur(sequitur_input);
    // println!("{:?}", sequitur_result);
    // for item in result {
    //     let symbol = match item.symbol {Atom::Symbol(x) => x, Atom::Terminal(x) => "".to_string()};
    //     println!("{:?}", symbol);
    //     for p in item.production {
    //         let p_symbol = match p {Atom::Symbol(x) => ("S", x), Atom::Terminal(x) => ("T", x)};
    //         println!("    {:?}", p_symbol);
    //     }
    // }

    // A~ = b c
    // A
    // ~A = a A~ | ~A A~
    // A = ~A e
}
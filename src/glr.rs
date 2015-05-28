#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]


use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

use std::collections::VecDeque;
use std::rc::{Rc, Weak};
use std::sync::Arc;
use std::cell::RefCell;
use std::mem;

use glr_grammar;
use glr_lex;
use glr_grammar::Atom as Atom;
use glr_lex::Lex as Lex;
use glr_grammar::GrammarItem as GrammarItem;

// #[derive(Debug,Clone,Hash,PartialEq,Eq,PartialOrd,Ord)]
// pub enum Atom {
//     Symbol(Arc<String>),
//     Terminal(Arc<String>)
// }

// #[derive(Clone,Debug)]
// pub struct GrammarItem {
//     pub symbol: Arc<Atom>,
//     pub production: Vec<Arc<Atom>>
// }

#[derive(Clone,Hash,PartialEq,Eq,Ord,PartialOrd,Debug)]
pub struct LRItem {
    pub stacktop: u8,
    pub terminal: Arc<Atom>,
    pub symbol: Arc<Atom>,
    pub production: Vec<Arc<Atom>>
}



pub fn create_grammars_hashmap(grammars: Vec<Box<GrammarItem>>) -> HashMap<String, Vec<Arc<GrammarItem>>>{
    let mut grammars_hashmap: HashMap<String, Vec<Arc<GrammarItem>>> = HashMap::new();

    for item in grammars {
        let symbol_name = glr_grammar::get_atom_string(item.clone().symbol);

        if !grammars_hashmap.contains_key(&symbol_name) {
            grammars_hashmap.insert(symbol_name.clone(), vec![]);
        }
        match grammars_hashmap.entry(symbol_name) {
            Occupied(entry) => {
                entry.into_mut().push(Arc::new(*item));
            },
            _ => {}
        }
    }

    grammars_hashmap
}



fn get_first(grammars_hashmap: &HashMap<String, Vec<Arc<GrammarItem>>>, symbol: Arc<Atom>) -> Vec<Arc<Atom>> {
    let mut ret: Vec<Arc<Atom>> = vec![];
    let mut symbols: Vec<Arc<Atom>> = vec![symbol.clone()];
    let mut symbols_expanded: HashSet<Arc<Atom>> = HashSet::new();
    loop {
        let current_symbol = match symbols.pop() {Some(x) => x, None => break};

        match *current_symbol {
            Atom::Symbol(ref x) => {
                if let Some(grammars) = grammars_hashmap.get(&**x) {
                    for each_grammar in grammars {
                        let mut offsets: Vec<u8> = find_solid_indexes(&grammars_hashmap, &each_grammar.production, 0, vec![0u8]);
                        let offsets_len = offsets.len();
                        offsets.push(offsets_len as u8);

                        for offset in offsets {
                            if let Some(__symbol) = each_grammar.production.get(offset as usize) {
                                let _symbol = __symbol.clone();
                                
                                if !symbols_expanded.contains(&_symbol) {
                                    symbols.push(_symbol.clone());
                                    symbols_expanded.insert(_symbol);
                                }
                            }
                        }
                    }
                }
            },
            Atom::Terminal(ref x) => {
                ret.push(current_symbol.clone());
            }
        }
    }

    // println!("first {:?}", (symbol, ret.clone()));
    ret
    // match *symbol {
    //     Atom::Symbol(ref x) => {
    //         let mut ret: Vec<&Atom> = vec![];
    //         match grammars_hashmap.get(x) {
    //             Some(grammars) => {

    //                 for each_grammar in grammars {
    //                     match each_grammar.production.get(0) {
    //                         Some(_symbol) => {
    //                             ret.push_all(&get_first(grammars_hashmap, _symbol));

    //                         },
    //                         None => {}
    //                     }

    //                 }

    //             },
    //             None => {}
    //         }
      
    //         ret
    //     },
    //     Atom::Terminal(ref x) => {
    //         vec![symbol]
    //     }
    // }
}

// fn contains_lr_item(list: &Vec<LRItem>, x: &LRItem) -> bool {
//     for item in list.iter() {
//         if item.stacktop != x.stacktop { continue }

//         if item.terminal != x.terminal { continue }

//         if item.symbol != x.symbol { continue }

//         if !equal_atom_vec(&item.production, &x.production) { continue }

//         return true;
//     }
//     false
// }
// fn contains_lr_item(list: Vec<Arc<LRItem>>, x: Arc<LRItem>) -> bool {
//     for item in list.iter() {
//         if item.stacktop != x.stacktop { continue }

//         if item.terminal != x.terminal { continue }

//         if item.symbol != x.symbol { continue }

//         if !equal_atom_vec(&item.production, &x.production) { continue }

//         return true;
//     }
//     false
// }

fn get_real_productions(grammars_hashmap: &HashMap<String, Vec<Arc<GrammarItem>>>, 
                        production: &Vec<Arc<Atom>>) -> Vec<Vec<Arc<Atom>>> {
    let mut ret: Vec<Vec<Arc<Atom>>> = vec![vec![]];
    for item in production.iter() {
        let mut split = false;
        match **item {
            Atom::Symbol(ref x) => {
                if let Some(atom_str) = x.find('~') {
                    if let Some(grammar_items) = grammars_hashmap.get(&**x) {
                        for item in grammar_items {
                            if item.production.len() == 0 {
                                split = true;
                                break;
                            }
                        }
                    }
                }
            },
            Atom::Terminal(ref x) => {}
        }

        if !split {
            for each_ret in &mut ret {
                each_ret.push(item.clone());
            }
        } else {
            let mut ret_extend: Vec<Vec<Arc<Atom>>> = vec![];
            for each_ret in ret.iter() {
                ret_extend.push(each_ret.clone());
            }  

            for each_ret in &mut ret {
                each_ret.push(item.clone());
            }

            ret.extend(ret_extend);
        }
    }
    ret
}
fn find_solid_indexes(grammars_hashmap: &HashMap<String, Vec<Arc<GrammarItem>>>, 
                        production: &Vec<Arc<Atom>>, 
                        start_index: u8,
                        initial_offset: Vec<u8>) -> Vec<u8>{

    let initial_len: u8 = match initial_offset.get(0) {Some(x) => *x, None => 0};
    let mut offsets: Vec<u8> = initial_offset;

    loop {
        let mut break_loop = true;
        if let Some(atom) = production.get((start_index + offsets.len() as u8) as usize) {
            match **atom {
                Atom::Symbol(ref x) => {
                    if let Some(grammar_items) = grammars_hashmap.get(&**x) {
                        for item in grammar_items {
                            if item.production.len() == 0 {
                                let len = offsets.len();
                                offsets.push(len as u8 + initial_len);
                                break_loop = false;
                                break;
                            }
                        }
                    }
                }, 
                Atom::Terminal(_) => {}
            }

        }
        if break_loop {break}
    }

    return offsets;
}

fn get_closure(grammars_hashmap: &HashMap<String, Vec<Arc<GrammarItem>>>, 
                initial_items: Vec<Arc<LRItem>>, 
                mut firsts: &mut HashMap<Arc<String>, Vec<Arc<Atom>>>) -> Vec<Arc<LRItem>> {
    let mut ret: Vec<Arc<LRItem>> = Vec::new();
    let mut ret_cache: HashSet<Arc<LRItem>> = HashSet::new();

        // println!("initial_items {:?}", initial_items.clone());
    let mut working_items = initial_items;

    loop {
        let mut loop_item = match working_items.pop() {Some(x) => x, None => break};

        if !ret_cache.contains(&loop_item) {
            let new_lr_item = Arc::new(LRItem {
                stacktop: loop_item.stacktop,
                terminal: loop_item.terminal.clone(),
                symbol: loop_item.symbol.clone(),
                production: loop_item.production.clone()
            });
            ret.push(new_lr_item.clone());
            ret_cache.insert(new_lr_item);
        }
        // watch_lr_item(&loop_item, "loop item");

        let mut terminals: Vec<Arc<Atom>> = vec![];
        let mut offsets: Vec<u8> = find_solid_indexes(&grammars_hashmap, &loop_item.production, loop_item.stacktop, vec![1u8]);
        
        for stacktop_offset in offsets {
            terminals.extend(match loop_item.production.get((loop_item.stacktop + stacktop_offset) as usize) {
                Some(atom) => {
                    let atom_string = match **atom {Atom::Symbol(ref x) => x, Atom::Terminal(ref x) => x};

                    match firsts.entry(atom_string.clone()) {
                        Occupied(entry) => {
                            let mut _ret: Vec<Arc<Atom>> = vec![];
                            for item in entry.into_mut().iter() {
                                _ret.push(item.clone());
                            }
                            _ret
                        },
                        Vacant(entry) => {
                            let new_first = get_first(grammars_hashmap, atom.clone());
                            let mut atoms: Vec<Arc<Atom>> = Vec::new();
                            for item in new_first.iter() {
                                atoms.push(item.clone());
                            }
                            entry.insert(atoms);
                            new_first
                        }
                    }

                },
                None => vec![loop_item.terminal.clone()]
            });
        }


        let expand_symbol = match loop_item.production.get(loop_item.stacktop as usize) { Some(atom) => { atom }, None => continue };
        let symbol_name = glr_grammar::get_atom_string(expand_symbol.clone());

        match grammars_hashmap.get(&symbol_name) {
           Some(grammer_items) => {
                for each_grammar in grammer_items.iter() {
                    for each_production in get_real_productions(&grammars_hashmap, &each_grammar.production).iter() {
                        for terminal_atom in terminals.iter() {
                            let lr_item = Arc::new(LRItem {
                                stacktop: 0,
                                terminal: terminal_atom.clone(),
                                symbol: each_grammar.symbol.clone(),
                                production: each_production.clone()
                            });
                            if !ret_cache.contains(&lr_item) {
                                working_items.push(lr_item);
                            }
                        }
                    }
                }
            },
            None => {}  
        }
 
    }

    ret
}

// fn clone_atom(x: &Atom) -> Atom {
//     match *x {
//         Atom::Symbol(ref x) => Atom::Symbol(x.clone()),
//         Atom::Terminal(ref x) => Atom::Terminal(x.clone())
//     }
// }

// fn equal_atom_vec(iters1: &Vec<Atom>, iters2: &Vec<Atom>) -> bool {
//     if iters1.len() != iters2.len() { return false; }
//     for index in 0..iters1.len(){
//         if iters1[index] != iters2[index] {
//             return false;
//         }
//     }
//     true
// }
// fn equal_cc(cc1: &Vec<LRItem>, cc2: &Vec<LRItem>) -> bool {
//     if cc1.len() != cc2.len() { return false }

//     for item in cc2.iter() {
//         if !contains_lr_item(cc1, item) {
//             return false;
//         }
//     }

//     true
// }

fn goto(grammars_hashmap: &HashMap<String, Vec<Arc<GrammarItem>>>, 
        cc: &Vec<Arc<LRItem>>, 
        goto_atom: Arc<Atom>,
        firsts: &mut HashMap<Arc<String>, Vec<Arc<Atom>>>,
        goto_cache: &mut HashMap<Vec<Arc<LRItem>>, Vec<Arc<LRItem>>>) -> Vec<Arc<LRItem>>{
    let mut ret: Vec<Arc<LRItem>> = Vec::new();
    // let goto_pattern = match goto_atom { Atom::Symbol(x) => ("S", x), Atom::Terminal(x) => ("T", x)};

    for item in cc.iter() {
        match item.production.get(item.stacktop as usize) {
            None => {},
            Some(x) => {
                // let x_pattern = match *x { Atom::Symbol(x) => ("S", x), Atom::Terminal(x) => ("T", x)};
                if *x == goto_atom {
                    ret.push(Arc::new(LRItem {
                        stacktop: item.stacktop + 1u8,
                        symbol: item.symbol.clone(),
                        terminal: item.terminal.clone(),
                        production: item.production.clone()
                    }));
                }
            }
        }
    }

    // watch_lr_list(&ret, "before closure");
    match goto_cache.entry(ret.clone()) {
        Occupied(entry) => {
            entry.get().clone()
        },
        Vacant(entry) => {
            let mut firsts = firsts;
            ret = get_closure(&grammars_hashmap, ret, &mut firsts);
            ret.sort_by(|a, b| b.cmp(a));
            entry.insert(ret.clone());
            ret     
        }
    }


}

// fn convert_hashset_to_vec(set: &mut HashSet<Arc<LRItem>>) -> Vec<Arc<LRItem>> {
//     let mut ret: BTreeMap<Arc<LRItem>, u8> = BTreeMap::new();

//     for item in set.drain() {
//         ret.insert(item, 0);
//     }

//     ret.into_iter().map(|(k, v)| k).collect()
// }
#[derive(Debug,Clone,Hash,PartialEq,Eq,PartialOrd,Ord)]
pub enum Action { Reduce, Shift }
#[derive(Debug,Clone,Hash,PartialEq,Eq,PartialOrd,Ord)]
pub struct TableItem {
    state: u32,
    lr_item: Option<Arc<GrammarItem>>,
    action: Arc<Action>
}
pub fn create_table(grammars_hashmap: &HashMap<String, Vec<Arc<GrammarItem>>>, initial_item: Arc<LRItem>) -> (Vec<HashMap<String, HashSet<Arc<TableItem>>>>, Vec<HashMap<String, u32>>) {
    let mut firsts: HashMap<Arc<String>, Vec<Arc<Atom>>> = HashMap::new();
    let mut goto_cache: HashMap<Vec<Arc<LRItem>>, Vec<Arc<LRItem>>> = HashMap::new();

    let mut action_table: Vec<HashMap<String, HashSet<Arc<TableItem>>>> = Vec::new();
    let mut goto_table: Vec<HashMap<String, u32>> = Vec::new();
    
    let mut cc0 = get_closure(&grammars_hashmap, vec![initial_item], &mut firsts);
    cc0.sort_by(|a, b| b.cmp(a));

    let mut working_ccs: Vec<Vec<Arc<LRItem>>> = Vec::new();
    let mut working_ccs_hashmap: HashMap<Vec<Arc<LRItem>>, u32> = HashMap::new();


    working_ccs.push(cc0.clone());
    working_ccs_hashmap.insert(cc0, 0);

    let mut cc_index = 0u32;
    let mut cc_to_append: Vec<Vec<Arc<LRItem>>> = Vec::new();
    loop {
        let mut _append_count = 0u32;
        for item in cc_to_append.iter() {
            working_ccs.push(item.clone());
            working_ccs_hashmap.insert(item.clone(), working_ccs.len() as u32 - 1);
        }
        cc_to_append.clear();

        let mut loop_cc = match working_ccs.get(cc_index as usize) { Some(x) => x, None => break };

        let mut new_action: HashMap<String, HashSet<Arc<TableItem>>> = HashMap::new();
        let mut new_goto: HashMap<String, u32> = HashMap::new();
        
        let mut next_cache: HashSet<Arc<Atom>> = HashSet::new();

        // println!("{:?}", (loop_cc.len(), working_ccs.len() - cc_index as usize));
        for item in loop_cc.iter() {
            let _state: u32 = working_ccs.len() as u32 + cc_to_append.len() as u32;
            match item.production.get(item.stacktop as usize) {
                None => { // Reduce arm
                    match *item.terminal {
                        Atom::Symbol(ref x) => {},
                        Atom::Terminal(ref x) => {
                            let new_action_item = Arc::new(TableItem {state: 0, action: Arc::new(Action::Reduce), lr_item: Some(Arc::new(GrammarItem {
                                symbol: item.symbol.clone(),
                                production: item.production.clone()
                            }))});

                            match new_action.entry((&x).to_string()) {
                                Occupied(entry) => {
                                    if item.production.len() > 0 {
                                        let mut m_entry = entry.into_mut();
                                        if !m_entry.contains(&new_action_item) {
                                            m_entry.insert(new_action_item);
                                        }
                                    }
                                },
                                Vacant(entry) => {
                                    let mut _new_set = HashSet::new();
                                    _new_set.insert(new_action_item);
                                    entry.insert(_new_set);
                                }
                            }
                        }
                    }
                },
                Some(x) => { // Shift arm
                    if next_cache.contains(x) {
                        continue
                    } else {
                        next_cache.insert(x.clone());
                    }

                    // let mut to_new_cc_list: Vec<Arc<Atom>> = vec![x.clone()];

                    // let mut firsts: Vec<Arc<Atom>> = vec![];
                    // if let Atom::Symbol(ref x_str) = **x {
                    //     match x_str.find('~') {
                    //         Some(_) => {
                    //             firsts = get_first(&grammars_hashmap, x.clone());
                    //         },
                    //         None => {}
                    //     } 
                    // }
                   

                    // for x in to_new_cc_list.iter() {

                        // println!("firsts {:?}", (x.clone(), get_first(&grammars_hashmap, x.clone())));
                        let mut new_cc = goto(&grammars_hashmap, &loop_cc, x.clone(), &mut firsts, &mut goto_cache);

                        if let Some(index) = working_ccs_hashmap.get(&new_cc) {
                            match **x {
                                Atom::Symbol(ref x) => {
                                    new_goto.insert((&x).to_string(), *index);

                                    // for first in firsts.iter(){
                                    //     if let Atom::Terminal(ref x_str) = **first {
                                    //         println!("cached symbol first {:?}", (&x_str).to_string());
                                    //         let new_action_item = Arc::new(TableItem {state: *index, action: Arc::new(action_table::Shift), lr_item: None});

                                    //         match new_action.entry((&x_str).to_string()) {
                                    //             Occupied(entry) => {
                                    //                 // entry.into_mut().insert(new_action_item);
                                    //             },
                                    //             Vacant(entry) => {
                                    //                 let mut _new_set = HashSet::new();
                                    //                 _new_set.insert(new_action_item);
                                    //                 entry.insert(_new_set);
                                    //             }
                                    //         }
                                    //     }
                                    // }
                                },
                                Atom::Terminal(ref x) => {
                                    let new_action_item = Arc::new(TableItem {state: *index, action: Arc::new(Action::Shift), lr_item: None});

                                    match new_action.entry((&x).to_string()) {
                                        Occupied(entry) => {
                                            let mut m_entry = entry.into_mut();
                                            if !m_entry.contains(&new_action_item) {
                                                m_entry.insert(new_action_item);
                                            }
                                        },
                                        Vacant(entry) => {
                                            let mut _new_set = HashSet::new();
                                            _new_set.insert(new_action_item);
                                            entry.insert(_new_set);
                                        }
                                    }
                                }
                            }
                        } else {
                            cc_to_append.push(new_cc);

                            match **x {
                                Atom::Symbol(ref x) => {
                                    new_goto.insert((&x).to_string(), _state);


                                    // for first in firsts.iter(){
                                    //     if let Atom::Terminal(ref x_str) = **first {
                                    //         println!("new symbol first {:?}", (&x_str).to_string());
                                    //         let new_action_item = Arc::new(TableItem {state: _state, action: Arc::new(action_table::Shift), lr_item: None});

                                    //         match new_action.entry((&x).to_string()) {
                                    //             Occupied(entry) => {
                                    //                 let mut m_entry = entry.into_mut();
                                    //                 if !m_entry.contains(&new_action_item) {
                                    //                     m_entry.insert(new_action_item);
                                    //                 }
                                    //             },
                                    //             Vacant(entry) => {
                                    //                 let mut new_cc = goto(&grammars_hashmap, &loop_cc, first.clone(), &mut firsts, &mut goto_cache);
                                    //                 let mut _new_set = HashSet::new();
                                    //                 _new_set.insert(new_action_item);
                                    //                 entry.insert(_new_set);

                                    //                 if working_ccs_hashmap.get(&new_cc) == None {
                                    //                     cc_to_append.push(new_cc); 
                                    //                 }
                                    //             }
                                    //         }
                                    //     }
                                    // }
                                },
                                Atom::Terminal(ref x) => {
                                    let new_action_item = Arc::new(TableItem {state: _state, action: Arc::new(Action::Shift), lr_item: None});

                                    match new_action.entry((&x).to_string()) {
                                        Occupied(entry) => {
                                            let mut m_entry = entry.into_mut();
                                            if !m_entry.contains(&new_action_item) {
                                                m_entry.insert(new_action_item);
                                            }
                                        },
                                        Vacant(entry) => {
                                            let mut _new_set = HashSet::new();
                                            _new_set.insert(new_action_item);
                                            entry.insert(_new_set);
                                        }
                                    }

                                }
                            }

                        }
                    // }

                 
                }
            }
        }

        // println!("action {:?}", new_action);
        action_table.push(new_action);
        goto_table.push(new_goto);

        cc_index += 1;
    }

    (action_table, goto_table)
}

#[derive(Debug,Clone)]
pub struct SyntaxTreeNode {
    pub value: Option<Arc<String>>,
    pub symbol: Arc<String>,
    // pub parent: Option<Weak<RefCell<SyntaxTreeNode>>>,
    pub children: Option<VecDeque<Box<SyntaxTreeNode>>>
}


impl Drop for SyntaxTreeNode {
    fn drop(&mut self) {
        let mut childrens: Vec<Option<VecDeque<Box<SyntaxTreeNode>>>> = vec![mem::replace(&mut self.children, None)];

        loop {
            match childrens.pop() {
                Some(n) => {
                    match n {
                        Some(mut x) => {
                            for item in x {
                                let mut _item = item;
                                childrens.push(mem::replace(&mut _item.children, None));
                            }
                        },
                        None => {}
                    }
                },
                None => break
            }
        }


    }
}

struct StateStackMark(*mut HashSet<Vec<u32>>);
unsafe impl Send for StateStackMark{}
pub fn parse(words: Vec<Arc<Lex>>, action: Vec<HashMap<String, HashSet<Arc<TableItem>>>>, goto: Vec<HashMap<String, u32>>) -> Vec<VecDeque<Box<SyntaxTreeNode>>>{

    fn _parse(words: Vec<Arc<Lex>>, 
            action: Vec<HashMap<String, HashSet<Arc<TableItem>>>>, 
            goto: Vec<HashMap<String, u32>>, 
            tree_stack: VecDeque<Box<SyntaxTreeNode>>,
            state_stack: Vec<u32>,
            word_stack: Vec<Arc<String>>,
            word_index: u32,
            curr_lex: Arc<Lex>,
            curr_word: Arc<String>,
            curr_value: Option<Arc<String>>,
            state: u32,
            cut_count: Vec<u32>,
            _table_items: Option<HashSet<Arc<TableItem>>>,
            state_stack_mark: HashSet<Vec<u32>>) -> Vec<VecDeque<Box<SyntaxTreeNode>>>{
        
        let mut tree_stack = tree_stack;
        let mut state_stack = state_stack;
        let mut word_stack = word_stack; 
        let mut word_index = word_index;
        let mut curr_lex = curr_lex;
        let mut curr_word = curr_word;
        let mut curr_value =curr_value;
        let mut state = state;
        let mut cut_count = cut_count;
        let mut _table_items = _table_items;
        let mut state_stack_mark = state_stack_mark;

        let mut paralle_trees: Vec<VecDeque<Box<SyntaxTreeNode>>> = Vec::new();

        // if state_stack_mark.contains(&state_stack) {
        //     println!("contains {:?}", state_stack_mark);
        //     return paralle_trees;
        // } else {
        //     state_stack_mark.insert(state_stack.clone());
        // }

        let (tx, rx): (Sender<Vec<VecDeque<Box<SyntaxTreeNode>>>>, Receiver<Vec<VecDeque<Box<SyntaxTreeNode>>>>) = mpsc::channel();
        let mut spawn_count = 0u32;
        loop {

            if let Some(ref table_items) = _table_items {
                if table_items.len() > 1 {

                    // println!("split talbe items {:?}", table_items);
                    // println!("stack {:?}", (state_stack.clone(), word_stack.clone()));
                    for table_item in table_items.iter() {
                        let __words = words.clone();
                        let __action = action.clone();
                        let __goto = goto.clone();
                        let __tree_stack = tree_stack.clone();
                        let __state_stack = state_stack.clone();
                        let __word_stack = word_stack.clone();
                        let __word_index = word_index.clone();
                        let __curr_lex = curr_lex.clone();
                        let __curr_word = curr_word.clone();
                        let __curr_value =curr_value.clone();
                        let __state = state.clone();
                        let __cut_count = cut_count.clone();
                        let mut __table_item = HashSet::new();
                        __table_item.insert(table_item.clone());
                        let __state_stack_mark = state_stack_mark.clone();

                        let thread_tx = tx.clone();
                        spawn_count += 1;
                        
                        thread::spawn(move || {
                           thread_tx.send(_parse(
                                __words, __action, __goto, __tree_stack, __state_stack, __word_stack,
                                __word_index, __curr_lex, __curr_word, __curr_value, __state,
                                __cut_count,  Some(__table_item), __state_stack_mark
                            )).unwrap(); 
                        });                        
                    }

                    break;
                
                } else {

                    println!(">>>>>>>>{:?}", word_stack);
                    println!("<<<<<<<<<{:?}", state_stack);
                    println!(">>>>>>>>current {:?}", curr_word);
                    let mut table_item = Arc::new(TableItem {state: 0u32, lr_item: None, action: Arc::new(Action::Reduce)});

                    for item in table_items.iter() 
                    {
                        table_item = item.clone();
                        break;
                    }
                    let _state = table_item.state;
                    let op_lr_item = table_item.lr_item.clone();
                    let _action = table_item.action.clone();

                    match *_action {
                        Action::Reduce => {
                            match op_lr_item {
                                Some(_lr_item) =>{
                                    let symbol = glr_grammar::get_atom_string(_lr_item.symbol.clone());

                                    let mut _cut_count = 0u32;
                                    for item in _lr_item.production.iter() {
                                        state_stack.pop();
                                        word_stack.pop();
                                        
                                        let _symbol = match **item {Atom::Symbol(ref x) => &***x, Atom::Terminal(ref x) => ""};
                                        match _symbol.find('~') {
                                            None => {
                                                _cut_count += 1;
                                            },
                                            _ => {
                                                if let Some(x) = cut_count.pop() {
                                                    _cut_count += x;
                                                }
                                            }
                                        }
                                    }

                                    match symbol.find('~') {
                                        Some(_) => {

                                            cut_count.push(_cut_count);
                                        },
                                        None => {
                                            let mut new_tree_node = Box::new(SyntaxTreeNode {value: None, symbol: match *_lr_item.symbol {Atom::Symbol(ref x) => x.clone(), Atom::Terminal(ref x) => x.clone()},  children: None});
                                            let mut tree_node_children: VecDeque<Box<SyntaxTreeNode>> = VecDeque::new();

                                            for i in 0.._cut_count as usize {
                                                match tree_stack.pop_back() {Some(mut x) => {
                                                    // x.parent = Some(new_tree_node.downgrade());
                                                    tree_node_children.push_front(x);     
                                                }, None => {}}
                                            }

                                            new_tree_node.children = Some(tree_node_children);
                                            tree_stack.push_back(new_tree_node);
                                        }
                                    }
                                    
                                    state = match state_stack.get(state_stack.len() - 1) {Some(x) => *x, None => break};
                                    word_stack.push(match *_lr_item.symbol {Atom::Symbol(ref x) => x.clone(), Atom::Terminal(ref x) => x.clone()});
                                    state_stack.push(match goto.get(state as usize) {Some(x) => match x.get(&symbol) {Some(x) => *x, None => 0}, None => 0});
                    
                                },
                                None => {}
                            }

                        },
                        Action::Shift => {
                            tree_stack.push_back(Box::new(SyntaxTreeNode {value: curr_value, symbol: curr_word.clone(), children: None}));
                            word_stack.push(curr_word);
                            state_stack.push(_state);

                            curr_lex = match words.get(word_index as usize) {Some(x) => x.clone(), None => break};
                            curr_word = match *curr_lex.atom {Atom::Symbol(ref x) => x.clone(), Atom::Terminal(ref x) => x.clone()};
                            curr_value = curr_lex.value.clone();

                            word_index += 1;
                        }
                    }
                    
                }

            }

            state = match state_stack.get(state_stack.len() - 1) {Some(x) => *x, None =>  break};
            


            _table_items = match action.get(state as usize) {
                Some(x) => {
                    match x.get(&**curr_word) {Some(x) => Some(x.clone()), None => {
                            // println!("break action inner word {:?}", (state, &**curr_word, x.clone()));
                            break
                        }
                    }
                },
                None => break
            }; 

        }

        // paralle_trees.push(if tree_stack.len() > 1 {VecDeque::new()} else {tree_stack});
        paralle_trees.push(tree_stack);
        for _ in 0..spawn_count {
            paralle_trees.extend(rx.recv().unwrap());
        }
        paralle_trees
    }


    let mut tree_stack: VecDeque<Box<SyntaxTreeNode>> = VecDeque::new();
    let mut state_stack: Vec<u32> = Vec::new();
    let mut word_stack: Vec<Arc<String>> = Vec::new();
    
    state_stack.push(0);

    let mut word_index = 1u32;
    let mut curr_lex = match words.get(0) {Some(x) => x.clone(), None => Arc::new(Lex {value: None, atom: Arc::new(Atom::Terminal(Arc::new("".to_string())))})};
    let mut curr_word = match *curr_lex.atom {Atom::Symbol(ref x) => x.clone(), Atom::Terminal(ref x) => x.clone()};
    let mut curr_value = curr_lex.value.clone();

    let mut cut_count: Vec<u32> = Vec::new();

    let mut _table_items = None;
    let mut state = 0u32;
    
    // let mut state_stack_mark: StateStackMark = StateStackMark(&mut HashSet::new());
    let mut state_stack_mark: HashSet<Vec<u32>> = HashSet::new();
    _parse(
        words, action, goto, tree_stack, state_stack, word_stack,
        word_index, curr_lex, curr_word,  curr_value, state,
        cut_count,  _table_items, state_stack_mark
    )

}

// fn main(){
// }

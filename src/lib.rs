use std::{fmt::Debug};

use crate::List::*;

// #[derive(Debug)]
#[derive(Clone)] 
pub enum List<i32> {
    Cons(i32, Box<List<i32>>),
    Nil,
}

impl List<i32> {
    pub fn head(&self) -> &i32 {
        match *self {
            Cons(ref head, _) => head,
            Nil => panic!("This is an empty list!!!")
        }
    }
    
    pub fn tail(&self) -> &Box<List<i32>> {
        match *self {
            Cons(_, ref tail) => tail,
            Nil => panic!("This is an empty list!!!")
        }
    }

    pub fn new() -> List<i32> {
        Nil
    }

    pub fn insert(self, elem: i32) -> List<i32> {
        Cons(elem, Box::new(self))
    }

    pub fn len(&self) -> i32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    pub fn print(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.print())
            },
            Nil => {
                format!("")
            },
        }
    }

    pub fn remove(&mut self, elem: i32) -> bool {
        match self {
            Cons(head, tail) => {
                if *head == elem {
                    *self = Cons(*tail.head(), tail.tail().clone());
                    true
                }
                else {
                    tail.remove(elem)
                }
            },
            Nil => {
                false
            },
        }
    }
}
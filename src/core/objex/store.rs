use std::collections::VecDeque;
use crate::core::objex::Objex;

#[derive(Default)]
pub struct ObjexStore {
    templates: VecDeque<Objex>,
}

impl ObjexStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn list(&self) -> Vec<Objex> {
        self.templates.iter().cloned().collect()
    }

    pub fn insert(&mut self, objex: Objex) -> Objex {
        self.templates.push_back(objex.clone());
        objex
    }
}


//! Module for declaration of top level items inside the type checker. The main structure of this
//! module is the [Module] structure that is responsible for storing the types of the top level
//! items.

use std::collections::HashMap;

use vulpi_intern::Symbol;
use vulpi_syntax::r#abstract::Qualified;

use crate::r#type::{r#virtual::Virtual, Type};

#[derive(Clone)]
pub enum Def {
    Enum(Vec<Qualified>),
    Record(Vec<Qualified>),
    Effect(Vec<Qualified>),
    Type,
}

#[derive(Clone)]
pub struct TypeData {
    pub kind: Type<Virtual>,
    pub binders: usize,
    pub module: Symbol,
    pub def: Def,
}

#[derive(Default, Clone)]
pub struct Module {
    /// The types of the functions.
    pub variables: im_rc::HashMap<Symbol, Type<Virtual>>,

    /// The types of the functions.
    pub constructors: im_rc::HashMap<Symbol, (Type<Virtual>, usize)>,

    /// The types of the types.
    pub types: im_rc::HashMap<Symbol, TypeData>,

    /// The fields of the records.
    pub fields: im_rc::HashMap<Symbol, Type<Virtual>>,

    /// The effects of some symbols.
    pub effects: im_rc::HashMap<Symbol, Type<Virtual>>,
}

#[derive(Default)]
pub struct Modules {
    /// The modules.
    pub modules: HashMap<Symbol, Module>,
}

impl Modules {
    pub fn new() -> Self {
        Self {
            modules: Default::default(),
        }
    }

    pub fn get(&mut self, id: Symbol) -> &mut Module {
        self.modules.entry(id).or_default()
    }
}
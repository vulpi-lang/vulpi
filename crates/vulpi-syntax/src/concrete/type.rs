use vulpi_location::Spanned;
use vulpi_macros::Show;

use crate::concrete::Lower;
use crate::tokens::Token;

use super::{Parenthesis, Path, Upper};

#[derive(Show)]
pub struct Effects {
    pub left_brace: Token,
    pub effects: Vec<(Box<Type>, Option<Token>)>,
    pub right_brace: Token,
}

#[derive(Show)]
pub struct TypeArrow {
    pub left: Box<Type>,
    pub arrow: Token,
    pub effects: Option<Effects>,
    pub right: Box<Type>,
}

#[derive(Show)]
pub struct TypeApplication {
    pub func: Box<Type>,
    pub args: Vec<Box<Type>>,
}

#[derive(Show)]
pub struct TypeForall {
    pub forall: Token,
    pub params: Vec<Lower>,
    pub dot: Token,
    pub body: Box<Type>,
}

#[derive(Show)]
pub enum TypeKind {
    Parenthesis(Parenthesis<Box<Type>>),
    Upper(Path<Upper>),
    TypeVariable(Lower),
    Arrow(TypeArrow),
    Application(TypeApplication),
    Forall(TypeForall),
    Unit(Token),
}

pub type Type = Spanned<TypeKind>;

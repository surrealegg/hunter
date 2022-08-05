use crate::{tokens::TokenKind, utils::Span};

#[derive(Debug)]
pub enum LiteralKind {
    Boolean(bool),
    Char(String),
    String(String),
    Identifier(String),
    Integer(i64),
}

#[derive(Debug)]
pub struct Literal {
    pub kind: LiteralKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum BinaryKind {
    As,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Equal,
    DoubleEqual,
    BangEqual,
    Great,
    GreatEqual,
    Less,
    LessEqual,
    RightShift,
    LeftShift,
    PlusEqual,
    MinusEqual,
    AsteriskEqual,
    SlashEqual,
    RightShiftEqual,
    LeftShiftEqual,
}

impl TryFrom<TokenKind> for BinaryKind {
    type Error = ();

    fn try_from(value: TokenKind) -> Result<Self, Self::Error> {
        match value {
            TokenKind::As => Ok(BinaryKind::As),
            TokenKind::Plus => Ok(BinaryKind::Plus),
            TokenKind::Minus => Ok(BinaryKind::Minus),
            TokenKind::Asterisk => Ok(BinaryKind::Asterisk),
            TokenKind::Slash => Ok(BinaryKind::Slash),
            TokenKind::Equal => Ok(BinaryKind::Equal),
            TokenKind::DoubleEqual => Ok(BinaryKind::DoubleEqual),
            TokenKind::BangEqual => Ok(BinaryKind::BangEqual),
            TokenKind::Great => Ok(BinaryKind::Great),
            TokenKind::GreatEqual => Ok(BinaryKind::GreatEqual),
            TokenKind::Less => Ok(BinaryKind::Less),
            TokenKind::LessEqual => Ok(BinaryKind::LessEqual),
            TokenKind::RightShift => Ok(BinaryKind::RightShift),
            TokenKind::LeftShift => Ok(BinaryKind::LeftShift),
            TokenKind::PlusEqual => Ok(BinaryKind::PlusEqual),
            TokenKind::MinusEqual => Ok(BinaryKind::MinusEqual),
            TokenKind::AsteriskEqual => Ok(BinaryKind::AsteriskEqual),
            TokenKind::SlashEqual => Ok(BinaryKind::SlashEqual),
            TokenKind::RightShiftEqual => Ok(BinaryKind::RightShiftEqual),
            TokenKind::LeftShiftEqual => Ok(BinaryKind::LeftShiftEqual),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum UnaryKind {
    Not,
    Plus,
    Minus,
}

impl TryFrom<TokenKind> for UnaryKind {
    type Error = ();

    fn try_from(value: TokenKind) -> Result<Self, Self::Error> {
        match value {
            TokenKind::Not => Ok(UnaryKind::Not),
            TokenKind::Plus => Ok(UnaryKind::Plus),
            TokenKind::Minus => Ok(UnaryKind::Minus),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Unary {
    pub expr: Box<Expression>,
    pub kind: UnaryKind,
    pub span: Span,
}

#[derive(Debug)]
pub struct Binary {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub kind: BinaryKind,
    pub span: Span,
}

#[derive(Debug)]
pub struct Call {
    pub name: String,
    pub arguments: Vec<Expression>,
    pub span: Span,
}

#[derive(Debug)]
pub struct Grouping {
    pub expr: Box<Expression>,
    pub span: Span,
}

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Binary(Binary),
    Unary(Unary),
    Call(Call),
    Grouping(Grouping),
}

#[derive(Debug)]
pub struct VariableDeclaration {
    pub name: String,
    pub value: Expression,
    pub mutable: bool,
    pub span: Span,
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct While {
    pub expression: Expression,
    pub block: Vec<Statement>,
}

#[derive(Debug)]
pub struct Break {
    pub span: Span,
}

#[derive(Debug)]
pub struct Continue {
    pub span: Span,
}

#[derive(Debug)]
pub enum Statement {
    Expression(Expression),
    VariableDeclaration(VariableDeclaration),
    Block(Block),
    Loop(Block),
    While(While),
    Break(Break),
    Continue(Continue),
}

impl Expression {
    pub fn span(&self) -> Span {
        match self {
            Expression::Literal(literal) => literal.span,
            Expression::Binary(binary) => binary.span,
            Expression::Unary(unary) => unary.span,
            Expression::Call(call) => call.span,
            Expression::Grouping(grouping) => grouping.span,
        }
    }
}

impl Statement {
    pub fn span(&self) -> Span {
        match self {
            Statement::Expression(expr) => expr.span(),
            Statement::VariableDeclaration(decl) => decl.span,
            Statement::Break(stat) => stat.span,
            Statement::Continue(stat) => stat.span,
            _ => Span::default(),
        }
    }
}

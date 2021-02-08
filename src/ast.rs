use super::log_rule;
use super::symbol::*;
use std::fmt::{Debug, Error, Formatter};
use walrus::ir::*;
use walrus::InstrSeqBuilder;
use walrus::*;

log_rule!();

pub trait Compile {
    fn compile(
        &self,
        module: &mut walrus::Module,
        builder: &mut InstrSeqBuilder,
        symbol_table: &mut SymbolTable,
    ) -> Result<(), &'static str>;
}

#[derive(Copy, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

impl Debug for Opcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Opcode::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
        }
    }
}

impl Compile for Opcode {
    fn compile(
        &self,
        _module: &mut walrus::Module,
        builder: &mut InstrSeqBuilder,
        symbol_table: &mut SymbolTable,
    ) -> Result<(), &'static str> {
        use self::Opcode::*;
        match *self {
            Mul => {
                builder.binop(BinaryOp::I32Mul);
                return Ok(());
            }
            Div => {
                builder.binop(BinaryOp::I32DivS);
                return Ok(());
            }
            Add => {
                builder.binop(BinaryOp::I32Add);
                return Ok(());
            }
            Sub => {
                builder.binop(BinaryOp::I32Sub);
                return Ok(());
            }
        };
    }
}

#[derive(Clone)]
pub enum Expr {
    Number(i32),
    Variable(String),
    Op(Box<Expr>, Opcode, Box<Expr>),
    Call(String, Vec<Box<Expr>>),
    Error,
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Self::Number(n) => write!(fmt, "{:?}", n),
            Self::Variable(ref identifier) => write!(fmt, "{:?}", identifier),
            Self::Op(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Self::Call(ref identifier, ref exprs) => write!(
                fmt,
                "Function {{ name:{:?}, parameters: {:?}}}",
                identifier, exprs
            ),
            Self::Error => write!(fmt, "error"),
        }
    }
}

impl Compile for Expr {
    fn compile(
        &self,
        module: &mut walrus::Module,
        builder: &mut InstrSeqBuilder,
        symbol_table: &mut SymbolTable,
    ) -> Result<(), &'static str> {
        use self::Expr::*;
        match *self {
            Number(n) => {
                builder.i32_const(n);
                return Ok(());
            }
            Op(ref l, op, ref r) => {
                l.compile(module, builder, symbol_table);
                r.compile(module, builder, symbol_table);
                op.compile(module, builder, symbol_table);
                return Ok(());
            }
            Variable(ref identifier) => {
                if let Some(attr) = symbol_table.lookup(identifier) {
                    match attr {
                        Attribute::Image(local_id, _) | Attribute::Material(local_id, _) => {
                            builder.local_get(*local_id);
                            return Ok(());
                        }
                        _ => {
                            log(&format!(
                                "Error: variable {:?} is neither an image nor a number.",
                                identifier
                            ));
                            return Err("Error");
                        }
                    }
                } else {
                    log(&format!("Error:  variable {:?} doesn't exist.", identifier));
                    return Err("Error");
                }
            }
            Call(ref identifier, ref exprs) => {
                if symbol_table.lookup(identifier).is_none() {
                    log(&format!(
                        "Error: function {:?} doesn't exist. Please try using an existing function from the library.",
                        identifier
                    ));
                    return Err("Error");
                };
                let attr = symbol_table.lookup(identifier).unwrap();

                match attr {
                    Attribute::Func(func_id, arguments, _) => {
                        let arguments = arguments.clone();
                        if exprs.len() != arguments.len() {
                            log(&format!(
                                "Error: function {:?} should take {:?} parameters instead of {:?} ",
                                identifier,
                                arguments.len(),
                                exprs.len()
                            ));
                            return Ok(());
                        }
                        for i in 0..exprs.len() {
                            let expr = &*exprs[i];
                            let argument_tp = arguments[i];
                            match expr {
                                Number(_) | Call(_, _) | Op(_, _, _) => {
                                    let expr_compile_result =
                                        expr.compile(module, builder, symbol_table);
                                    if !expr_compile_result.is_ok() {
                                        return expr_compile_result;
                                    }
                                }
                                Variable(ref var_ident) => {
                                    if let Some(attr) = symbol_table.lookup(var_ident) {
                                        if argument_tp == walrus::ValType::I32 {
                                            match attr {
                                                Attribute::Image(_, _) | Attribute::Number(_) => {
                                                    let expr_compile_result =
                                                        expr.compile(module, builder, symbol_table);
                                                    if !expr_compile_result.is_ok() {
                                                        return expr_compile_result;
                                                    }
                                                }
                                                _ => {
                                                    log(&format!(
                                                        "Error: {:?} has a wrong type",
                                                        expr
                                                    ));
                                                    return Err("Error");
                                                }
                                            }
                                        } else {
                                            log(&format!("Error: {:?} has a wrong type", expr));
                                            return Err("Error");
                                        }
                                    } else {
                                        log(&format!(
                                            "Error: Variable {:?} does not exist",
                                            var_ident
                                        ));
                                        return Err("Error");
                                    }
                                }
                                _ => {}
                            }
                        }
                        builder.call(*func_id);
                        symbol_table.library_tracker.add_image(None, None);
                        return Ok(());
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        return Ok(());
    }
}

// #[derive(Debug)]
#[derive(Clone)]
pub enum Statement {
    VarDef(String, Box<Expr>),
    Block(Vec<Statement>),
}

impl Debug for Statement {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match &*self {
            Self::VarDef(ref identifier, ref expr) => {
                write!(fmt, "identifier: {:?}, expression: {:?}", identifier, expr)
            }
            Self::Block(ref statements) => {
                write!(fmt, "statements: {:?}", statements)
            }
        }
    }
}

impl Compile for Statement {
    fn compile(
        &self,
        module: &mut walrus::Module,
        builder: &mut InstrSeqBuilder,
        symbol_table: &mut SymbolTable,
    ) -> Result<(), &'static str> {
        use self::Statement::*;
        match &*self {
            VarDef(ref identifier, ref expr) => {
                if identifier.parse::<i32>().is_ok() {
                    log(&format!(
                        "Error: please use a non-numeric name for a variable."
                    ));
                    return Err(&format!(
                        "Error: please use a non-numeric name for a variable."
                    ));
                }
                let expr = &**expr;
                match expr {
                    Expr::Number(_) | Expr::Op(_, _, _) => {
                        let expr_compile_result = expr.compile(module, builder, symbol_table);
                        if expr_compile_result.is_ok() {
                            let local_id = module.locals.add(ValType::I32);
                            builder.local_set(local_id);
                            symbol_table
                                .insert(identifier.to_string(), Attribute::Number(local_id));
                            return Ok(());
                        } else {
                            return expr_compile_result;
                        };
                    }
                    Expr::Variable(var_right) => {
                        if let Some(attr) = symbol_table.lookup(&var_right) {
                            match attr {
                                Attribute::Number(var_right_local_id) => {
                                    builder.local_get(*var_right_local_id);
                                    let local_id = module.locals.add(ValType::I32);
                                    builder.local_set(local_id);
                                    symbol_table.insert(
                                        identifier.clone(),
                                        Attribute::Number(*var_right_local_id),
                                    );
                                    return Ok(());
                                }
                                Attribute::Image(var_right_local_id, image_info) => {
                                    builder.local_get(*var_right_local_id);
                                    let local_id = module.locals.add(ValType::I32);
                                    builder.local_set(local_id);
                                    symbol_table.insert(
                                        identifier.clone(),
                                        Attribute::Image(*var_right_local_id, None),
                                    );
                                    return Ok(());
                                }
                                Attribute::Material(_, _) => {
                                    return Ok(());
                                }
                                _ => {
                                    return Ok(());
                                }
                            }
                        } else {
                            log(&format!(
                                "Error:  variable {:?} doesn't exist, please define {:?} with something else.",
                                var_right, identifier
                            ));
                            return Err("Error");
                        }
                    }
                    Expr::Call(func_ident, _exprs) => {
                        if let Some(Attribute::Func(_, _, returns)) =
                            symbol_table.lookup(&func_ident)
                        {
                            let call_compile_result = expr.compile(module, builder, symbol_table);
                            if call_compile_result.is_ok() {
                                if *returns == vec![ValType::I32] {
                                    let local_id = module.locals.add(ValType::I32);
                                    builder.local_set(local_id);
                                    symbol_table.insert(
                                        identifier.clone(),
                                        Attribute::Image(local_id, None),
                                    );
                                    symbol_table
                                        .library_tracker
                                        .add_image(Some(identifier.clone()), None);
                                    return Ok(());
                                } else {
                                    return call_compile_result;
                                }
                            } else {
                                return call_compile_result;
                            }
                        } else {
                            return Err("Error");
                        }
                    }
                    _ => {
                        log(&format!(
                            "Error: please define {:?} with a number, image or call to a function.",
                            identifier
                        ));
                        return Err("Error");
                    }
                }
            }
            Block(statements) => {
                for statement in statements {
                    let statement_compile_result = statement.compile(module, builder, symbol_table);
                    if statement_compile_result.is_ok() {
                        // let mut new_function_builder =
                        //     FunctionBuilder::new(&mut module.types, &vec![], &[]);
                        // let mut new_builder = RefCell::new(new_function_builder.func_body());
                        builder.block(InstrSeqType::Simple(Option::None), |builder| {
                            statement.compile(module, builder, symbol_table);
                        });
                    } else {
                        return statement_compile_result;
                    }
                }
                return Ok(());
            }
            _ => {
                return Ok(());
            }
        }
    }
}

#[derive(Debug)]
pub struct Prototype {
    pub identifier: String,
    pub params: Vec<String>,
}

impl Prototype {
    pub fn new(identifier: String, params: Vec<String>) -> Self {
        Prototype { identifier, params }
    }
}

impl Compile for Prototype {
    fn compile(
        &self,
        _module: &mut walrus::Module,
        builder: &mut InstrSeqBuilder,
        symbol_table: &mut SymbolTable,
    ) -> Result<(), &'static str> {
        return Ok(());
    }
}

#[derive(Debug)]
pub struct Function {
    pub prototype: Prototype,
    pub block: Statement,
}

impl Function {
    pub fn new(prototype: Prototype, block: Statement) -> Self {
        Function { prototype, block }
    }
}

impl Compile for Function {
    fn compile(
        &self,
        module: &mut walrus::Module,
        builder: &mut InstrSeqBuilder,
        symbol_table: &mut SymbolTable,
    ) -> Result<(), &'static str> {
        self.prototype.compile(module, builder, symbol_table);

        let block_compile_result = self.block.compile(module, builder, symbol_table);

        return block_compile_result;
    }
}

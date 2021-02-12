use super::compiler::Memory;
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
        memories: &mut Vec<&Memory>,
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
        _symbol_table: &mut SymbolTable,
        _memories: &mut Vec<&Memory>,
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
        memories: &mut Vec<&Memory>,
    ) -> Result<(), &'static str> {
        use self::Expr::*;
        match *self {
            Number(n) => {
                builder.i32_const(n);
                return Ok(());
            }
            Op(ref l, op, ref r) => {
                let l_compile_result = l.compile(module, builder, symbol_table, memories);
                if !l_compile_result.is_ok() {
                    return l_compile_result;
                }
                let r_compile_result = r.compile(module, builder, symbol_table, memories);
                if !r_compile_result.is_ok() {
                    return r_compile_result;
                }
                let op_compile_result = op.compile(module, builder, symbol_table, memories);
                if !op_compile_result.is_ok() {
                    return op_compile_result;
                }
                return Ok(());
            }
            Variable(ref identifier) => {
                if let Some(attr) = symbol_table.lookup(identifier) {
                    match attr {
                        Attribute::Image(local_id, _) => {
                            builder.local_get(*local_id);
                            return Ok(());
                        }
                        Attribute::Material(_, _, _) => {}
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
                match symbol_table.lookup(identifier).unwrap().clone() {
                    Attribute::Func(func_id, arguments, _) => {
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
                                        expr.compile(module, builder, symbol_table, memories);
                                    if !expr_compile_result.is_ok() {
                                        return expr_compile_result;
                                    }
                                }
                                Variable(ref var_ident) => {
                                    if let Some(attr) = symbol_table.lookup(var_ident) {
                                        if argument_tp == walrus::ValType::I32 {
                                            match attr {
                                                Attribute::Image(_, _) | Attribute::Number(_) => {
                                                    let expr_compile_result = expr.compile(
                                                        module,
                                                        builder,
                                                        symbol_table,
                                                        memories,
                                                    );
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
                        builder.call(func_id);
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
    Declare(String, String, Option<Box<Expr>>),
    Assignment(String, Box<Expr>),
    Block(Vec<Statement>),
}

impl Debug for Statement {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match &*self {
            Self::Declare(ref identifier, ref var_type, ref expr) => {
                write!(
                    fmt,
                    "identifier: {:?}, type: {:?}, expression: {:?}",
                    identifier, var_type, expr
                )
            }
            Self::Assignment(ref identifier, ref expr) => {
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
        memories: &mut Vec<&Memory>,
    ) -> Result<(), &'static str> {
        use self::Statement::*;
        match &*self {
            Declare(ref identifier, ref var_type, ref expr) => {
                if identifier.parse::<i32>().is_ok() {
                    log(&format!(
                        "Error: please use a non-numeric name for a variable."
                    ));
                    return Err("Error");
                }

                match var_type.as_str() {
                    "N" | "Number" | "number" => {
                        let local_id = module.locals.add(ValType::I32);
                        if expr.is_some() {
                            let expr = &**(expr.as_ref().unwrap());
                            match expr {
                                Expr::Variable(var_right_ident) => {
                                    if let Some(var_right) = symbol_table.lookup(&var_right_ident) {
                                        match var_right {
                                            Attribute::Number(var_right_local_id) => {
                                                builder.local_get(*var_right_local_id);
                                                builder.local_set(local_id);
                                            }
                                            _ => {
                                                log(&format!(
                                                    "Error: {:?} is not a number.",
                                                    var_right_ident
                                                ));
                                                symbol_table.remove(identifier);
                                                return Err("Error");
                                            }
                                        }
                                    } else {
                                        log(&format!(
                                            "Error: {:?} does not exist.",
                                            var_right_ident
                                        ));
                                        symbol_table.remove(identifier);
                                        return Err("Error");
                                    }
                                    symbol_table.insert(
                                        identifier.to_string(),
                                        Attribute::Number(local_id),
                                    );
                                }
                                Expr::Number(_) | Expr::Op(_, _, _) => {
                                    let expr_compile_result =
                                        expr.compile(module, builder, symbol_table, memories);
                                    if !expr_compile_result.is_ok() {
                                        symbol_table.remove(identifier);
                                        return expr_compile_result;
                                    } else {
                                        builder.local_set(local_id);
                                    }
                                    symbol_table.insert(
                                        identifier.to_string(),
                                        Attribute::Number(local_id),
                                    );
                                }
                                _ => {
                                    log(&format!(
                                        "Error: {:?} and {:?} has different type.",
                                        identifier, expr
                                    ));
                                    symbol_table.remove(identifier);
                                    return Err("Error");
                                }
                            }
                        }
                        return Ok(());
                    }
                    "I" | "Image" | "image" => {
                        let local_id = module.locals.add(ValType::I32);

                        if expr.is_some() {
                            let expr = &**(expr.as_ref().unwrap());
                            match expr {
                                Expr::Variable(right_ident) => {
                                    if let Some(right) = symbol_table.lookup(&right_ident) {
                                        match right {
                                            Attribute::Image(right_local_id, _) => {
                                                builder.local_get(*right_local_id);
                                                builder.local_set(local_id);
                                            }
                                            _ => {
                                                log(&format!(
                                                    "Error: {:?} is not an image.",
                                                    right_ident
                                                ));
                                                symbol_table.remove(identifier);
                                                return Err("Error");
                                            }
                                        }
                                    } else {
                                        log(&format!("Error: {:?} does not exist.", right_ident));
                                        symbol_table.remove(identifier);
                                        return Err("Error");
                                    }
                                    symbol_table.insert(
                                        identifier.to_string(),
                                        Attribute::Number(local_id),
                                    );
                                }
                                Expr::Call(right_ident, _) => {
                                    if let Some(right) = symbol_table.lookup(&right_ident) {
                                        match right {
                                            Attribute::Func(right_func_id, _, _) => {
                                                let right_compile_result = expr.compile(
                                                    module,
                                                    builder,
                                                    symbol_table,
                                                    memories,
                                                );
                                                if !right_compile_result.is_ok() {
                                                    symbol_table.remove(identifier);
                                                    return right_compile_result;
                                                }
                                            }
                                            _ => {
                                                log(&format!(
                                                    "Error: {:?} is not an image.",
                                                    right_ident
                                                ));
                                                symbol_table.remove(identifier);
                                                return Err("Error");
                                            }
                                        }
                                    } else {
                                        log(&format!("Error: {:?} does not exist.", right_ident));
                                        symbol_table.remove(identifier);
                                        return Err("Error");
                                    }
                                }
                                _ => {
                                    log(&format!(
                                        "Error: {:?} and {:?} has different type.",
                                        identifier, expr
                                    ));
                                    symbol_table.remove(identifier);
                                    return Err("Error");
                                }
                            }
                        }

                        symbol_table
                            .insert(identifier.to_string(), Attribute::Image(local_id, None));
                        return Ok(());
                    }
                    "M" | "Material" | "material" => {
                        return Ok(());
                    }
                    _ => {
                        log(&format!(
                            "Error: type {:?} not supported.",
                            var_type.as_str()
                        ));
                        return Err("Error");
                    }
                }
            }
            Assignment(ref identifier, ref expr) => {
                let expr = &**expr;
                if symbol_table.lookup(identifier).is_none() {
                    log(&format!(
                        "Error: {:?} doesn't exist. Please decalre or define it. Example: var image0:Image; or var image1:Image = file_001;",
                        identifier
                    ));
                    return Err("Error");
                }

                match symbol_table.lookup(identifier).unwrap().clone() {
                    Attribute::Number(left_local_id) => match expr {
                        Expr::Variable(right_ident) => {
                            if symbol_table.lookup(&right_ident).is_none() {
                                log(&format!("Error: {:?} does not exist.", right_ident));
                                return Err("Error");
                            }
                            match symbol_table.lookup(&right_ident) {
                                Some(Attribute::Number(right_local_id)) => {
                                    builder.local_get(*right_local_id);
                                    builder.local_set(left_local_id);
                                }
                                _ => {
                                    log(&format!("Error: {:?} is not a number.", expr));
                                    return Err("Error");
                                }
                            }
                        }
                        Expr::Number(_) | Expr::Op(_, _, _) => {
                            let expr_compile_result =
                                expr.compile(module, builder, symbol_table, memories);
                            if expr_compile_result.is_ok() {
                                builder.local_set(left_local_id);
                            } else {
                                return expr_compile_result;
                            };
                        }
                        _ => {
                            log(&format!(
                                "Error: value of {:?} should be as a number",
                                identifier,
                            ));
                            return Err("Error");
                        }
                    },
                    Attribute::Image(left_local_id, _) => {
                        log(&format!(""));
                        match expr {
                            Expr::Variable(right_ident) => {
                                if symbol_table.lookup(&right_ident).is_none() {
                                    log(&format!("Error: {:?} does not exist.", right_ident));
                                    return Err("Error");
                                }
                                match symbol_table.lookup(&right_ident).unwrap().clone() {
                                    Attribute::Image(right_local_id, right_image_info) => {
                                        let update_result = symbol_table.update(
                                            identifier,
                                            Attribute::Image(
                                                right_local_id.clone(),
                                                right_image_info.clone(),
                                            ),
                                        );
                                        if update_result.is_ok() {
                                            builder.local_get(right_local_id);
                                            builder.local_set(left_local_id);
                                        } else {
                                            return Err("Error");
                                        }
                                    }
                                    _ => {
                                        log(&format!("Error: {:?} is not an image.", right_ident));
                                        return Err("Error");
                                    }
                                }
                            }
                            Expr::Call(func_ident, _) => {
                                let call_compile_result =
                                    expr.compile(module, builder, symbol_table, memories);

                                if call_compile_result.is_ok() {
                                    match symbol_table.lookup(func_ident).unwrap().clone() {
                                        Attribute::Func(_, _, returns) => {
                                            if returns == vec![walrus::ValType::I32] {
                                                let update_result = symbol_table.update(
                                                    identifier,
                                                    Attribute::Image(left_local_id, None),
                                                );
                                                if update_result.is_ok() {
                                                    builder.local_set(left_local_id);
                                                } else {
                                                    return Err("Error");
                                                }
                                            } else {
                                                return call_compile_result;
                                            }
                                        }
                                        _ => {
                                            log(&format!(
                                            "Error: {:?} doesn't exist. Please use another function",
                                            func_ident
                                        ));
                                            return Err("Error");
                                        }
                                    }
                                } else {
                                    return call_compile_result;
                                }
                            }
                            _ => {
                                log(&format!(
                                    "Error: value of {:?} should be as an image",
                                    identifier,
                                ));
                                return Err("Error");
                            }
                        }
                    }
                    Attribute::Material(_, _, _) => {}
                    _ => {}
                }
            }
            Block(_) => {
                // builder.block(InstrSeqType::Simple(Option::None), |builder| {
                //     for statement in statements {

                //         let statement_compile_result =
                //             statement.compile(module, builder, symbol_table);
                //         if !statement_compile_result.is_ok() {
                //             return statement_compile_result;
                //         }
                //     }
                // });

                return Ok(());
            }
        }
        return Ok(());
    }
}

#[derive(Debug)]
pub struct Prototype {
    pub identifier: String,
    pub arguments: Vec<String>,
}

impl Prototype {
    pub fn new(identifier: String, arguments: Vec<String>) -> Self {
        Prototype {
            identifier,
            arguments,
        }
    }
}

impl Compile for Prototype {
    fn compile(
        &self,
        _module: &mut walrus::Module,
        _builder: &mut InstrSeqBuilder,
        _symbol_table: &mut SymbolTable,
        _memories: &mut Vec<&Memory>,
    ) -> Result<(), &'static str> {
        return Ok(());
    }
}

#[derive(Debug)]
pub struct Function {
    pub prototype: Prototype,
    pub statements: Vec<Statement>,
}

impl Function {
    pub fn new(prototype: Prototype, statements: Vec<Statement>) -> Self {
        Function {
            prototype,
            statements,
        }
    }
}

impl Compile for Function {
    fn compile(
        &self,
        module: &mut walrus::Module,
        builder: &mut InstrSeqBuilder,
        symbol_table: &mut SymbolTable,
        memories: &mut Vec<&Memory>,
    ) -> Result<(), &'static str> {
        let _prototype_compile_result =
            self.prototype
                .compile(module, builder, symbol_table, memories);

        for statement in &self.statements {
            let statement_compile_result =
                statement.compile(module, builder, symbol_table, memories);
            if !statement_compile_result.is_ok() {
                return statement_compile_result;
            }
        }
        return Ok(());
    }
}

use super::compiler::{Memory, MemoryValue};
use super::log_rule;
use super::symbol::*;
use std::fmt::{Debug, Error, Formatter};
use walrus::ir::*;
use walrus::InstrSeqBuilder;
use walrus::*;

log_rule!();

const ALIGN: u32 = 2;

pub trait Compile {
    fn compile(
        &self,
        module: &mut walrus::Module,
        builder: &mut InstrSeqBuilder,
        symbol_table: &mut SymbolTable,
        memories: &mut Memory,
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
        _memories: &mut Memory,
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
        memories: &mut Memory,
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
    Assignment(Vec<String>, Box<Expr>),
    Block(Vec<Statement>),
    Call(String, Vec<Box<Expr>>),
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
            Self::Call(ref identifier, ref exprs) => {
                write!(fmt, "function{:?}({:?})", identifier, exprs)
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
        memories: &mut Memory,
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
                    "N" | "Number" | "n" | "number" => {
                        let local_id = module.locals.add(ValType::I32);
                        symbol_table.insert(identifier.to_string(), Attribute::Number(local_id));
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
                    "I" | "Image" | "i" | "image" => {
                        let local_id = module.locals.add(ValType::I32);

                        if expr.is_some() {
                            let expr = &**(expr.as_ref().unwrap());
                            match expr {
                                Expr::Variable(right_ident) => {
                                    if symbol_table.lookup(&right_ident).is_none() {
                                        log(&format!("Error: {:?} does not exist.", right_ident));
                                        return Err("Error");
                                    }
                                    match symbol_table.lookup(&right_ident).unwrap().clone() {
                                        Attribute::Image(right_local_id, right_image) => {
                                            builder.local_get(right_local_id);
                                            builder.local_set(local_id);
                                            symbol_table.insert(
                                                identifier.to_string(),
                                                Attribute::Image(
                                                    right_local_id.clone(),
                                                    right_image.clone(),
                                                ),
                                            );
                                        }
                                        _ => {
                                            log(&format!(
                                                "Error: {:?} is not an image.",
                                                right_ident
                                            ));
                                            return Err("Error");
                                        }
                                    }
                                }
                                Expr::Call(right_ident, _) => {
                                    if symbol_table.lookup(&right_ident).is_none() {
                                        log(&format!("Error: {:?} does not exist.", right_ident));
                                        return Err("Error");
                                    }
                                    match symbol_table.lookup(&right_ident).unwrap().clone() {
                                        Attribute::Func(_, _, _) => {
                                            symbol_table.insert(
                                                identifier.to_string(),
                                                Attribute::Image(local_id.clone(), None),
                                            );
                                            let assignment_statement = Statement::Assignment(
                                                vec![identifier.clone()],
                                                Box::new(expr.clone()),
                                            );
                                            let assignment_compile_result = assignment_statement
                                                .compile(module, builder, symbol_table, memories);
                                            if assignment_compile_result.is_err() {
                                                symbol_table.remove(identifier);
                                                return assignment_compile_result;
                                            }
                                        }
                                        _ => {
                                            log(&format!(
                                                "Error: {:?} is not a function.",
                                                right_ident
                                            ));
                                            return Err("Error");
                                        }
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
                    "M" | "Material" | "m" | "material" => {
                        let (mem_id, offset) =
                            memories.store(builder, None, vec![MemoryValue::i32(i32::MAX); 32]);
                        symbol_table.insert(
                            identifier.clone(),
                            Attribute::Material(mem_id, offset, "PBRMetalness".to_string()),
                        );

                        if let Some(expression) = expr {
                            let assignment_statement = Statement::Assignment(
                                vec![identifier.clone()],
                                Box::new(*expression.clone()),
                            );
                            let assignment_compile_result = assignment_statement.compile(
                                module,
                                builder,
                                symbol_table,
                                memories,
                            );
                            if assignment_compile_result.is_err() {
                                symbol_table.remove(identifier);
                                return assignment_compile_result;
                            }
                        }
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
            Assignment(ref identifiers, ref expr) => {
                if identifiers.len() == 2 {
                    if symbol_table.lookup(&identifiers[0]).is_none() {
                        return Err("Error");
                    }
                    match symbol_table.lookup(&identifiers[0]).unwrap().clone() {
                        Attribute::Material(_, material_offset, material_type) => {
                            let channel_name = &identifiers[1];
                            if let Ok(channel_index) = symbol_table
                                .library_tracker
                                .material_info
                                .find_channel_index(&material_type, channel_name)
                            {
                                let expr = &**expr;
                                match expr {
                                    Expr::Variable(right_ident) => {
                                        if symbol_table.lookup(&right_ident).is_none() {
                                            log(&format!(
                                                "Error: {:?} does not exist.",
                                                right_ident
                                            ));
                                            return Err("Error");
                                        }
                                        match symbol_table.lookup(&right_ident).unwrap().clone() {
                                            Attribute::Image(right_local_id, _) => {
                                                memories.store(
                                                    builder,
                                                    Some(
                                                        material_offset
                                                            + 2 * u32::pow(2, ALIGN)
                                                            + 4 * channel_index,
                                                    ),
                                                    vec![MemoryValue::walrus_id(right_local_id)],
                                                );
                                            }
                                            _ => {
                                                log(&format!(
                                                    "Error: {:?} is not an image.",
                                                    right_ident
                                                ));
                                                return Err("Error");
                                            }
                                        }
                                    }
                                    Expr::Call(func_ident, _) => {
                                        match symbol_table.lookup(func_ident).unwrap().clone() {
                                            Attribute::Func(_, _, returns) => {
                                                if returns == vec![walrus::ValType::I32] {
                                                    let offset = material_offset
                                                        + 2 * u32::pow(2, ALIGN)
                                                        + u32::pow(2, ALIGN) * channel_index;
                                                    builder.i32_const(offset as i32);
                                                    let call_compile_result = expr.compile(
                                                        module,
                                                        builder,
                                                        symbol_table,
                                                        memories,
                                                    );
                                                    if call_compile_result.is_err() {
                                                        return call_compile_result;
                                                    }
                                                    builder.store(
                                                        memories.id,
                                                        walrus::ir::StoreKind::I32 {
                                                            atomic: false,
                                                        },
                                                        walrus::ir::MemArg {
                                                            align: ALIGN,
                                                            offset: 0,
                                                        },
                                                    );
                                                } else {
                                                    return Ok(());
                                                }
                                            }
                                            _ => {
                                                log(&format!(
                                            "Error: {:?} doesn't exist. Please use another function",
                                            func_ident
                                        ));
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            } else {
                                log(&format!("Error: Can't find channel {:?} of material type {:?} for variable {:?}",channel_name, material_type, identifiers[0]));
                            }
                        }
                        _ => {
                            log(&format!("Error: Please use a material."));
                        }
                    }
                } else if identifiers.len() == 1 {
                    let identifier = &identifiers[0];
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
                        Attribute::Image(left_local_id, _) => match expr {
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
                        },
                        Attribute::Material(_, left_offset, _) => match expr {
                            Expr::Variable(right_ident) => {
                                if let Some(Attribute::Material(_, right_offset, _)) =
                                    symbol_table.lookup(right_ident)
                                {
                                    memories.copy(
                                        builder,
                                        left_offset.clone(),
                                        right_offset.clone(),
                                        32,
                                    );
                                } else {
                                    log(&format!(
                                        "Error: Please define {:?} with a material variable;",
                                        identifier
                                    ));
                                    return Err("Error");
                                }
                            }
                            Expr::Call(right_func_ident, right_func_params) => {
                                if *right_func_ident != "new_material".to_string() {
                                    log(&format!(
                                    "Error: Please call new_material(width,height) to define {:?};",
                                    identifier
                                ));
                                    return Err("Error");
                                }
                                if right_func_params.len() != 2 {
                                    log(&format!(
                                    "Error: Please call new_material(width,height) to define {:?};",
                                    identifier
                                ));
                                    return Err("Error");
                                }

                                let width = &*right_func_params[0];
                                let height = &*right_func_params[1];

                                if let Expr::Number(width_i32) = width {
                                    memories.store(
                                        builder,
                                        Some(left_offset),
                                        vec![MemoryValue::i32(*width_i32)],
                                    );
                                } else {
                                    log(&format!(
                                        "Error: Please use a number for material's width",
                                    ));
                                    return Err("Error");
                                }
                                if let Expr::Number(height_i32) = height {
                                    memories.store(
                                        builder,
                                        Some(left_offset + 4),
                                        vec![MemoryValue::i32(*height_i32)],
                                    );
                                } else {
                                    log(&format!(
                                        "Error: Please use a number for material's height",
                                    ));
                                    return Err("Error");
                                }
                            }
                            _ => {
                                log(&format!(
                                "Error: Please define the material {:?} with another material variable, or call new_material(width,height) function",
                                identifier
                            ));
                                return Err("Error");
                            }
                        },
                        _ => {}
                    }
                } else {
                    log(&format!(
                        "Error: Please have a variable on the left side = to assign the value to."
                    ));
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
            Call(ref identifier, ref exprs) => match identifier.as_str() {
                "logger" => {
                    log(&format!("logging: {:?}", exprs));
                }
                "show" => {
                    for expr in exprs {
                        match &**expr {
                            Expr::Variable(expr_ident) => {
                                if symbol_table.lookup(&expr_ident).is_none() {
                                    log(&format!(
                                        "Error: {:?} doesn't exist. Please decalre or define it. Example: var image0:Image; or var image1:Image = file_001;",
                                        identifier
                                    ));
                                    return Err("Error");
                                }
                                match symbol_table.lookup(&expr_ident).unwrap().clone() {
                                    Attribute::Image(_, _) => {
                                        symbol_table
                                            .library_tracker
                                            .add_export_image(expr_ident.clone());
                                    }
                                    Attribute::Material(_, _, _) => {}
                                    _ => {
                                        log(&format!("Error: show() can only be used for image and material. show() is default for material."));
                                    }
                                }
                            }
                            _ => {
                                log(&format!("Error: show() can only be used for image and material. show() is default for material."));
                            }
                        }
                    }
                }
                _ => {
                    log(&format!("Error: Please store function {:?}'s return in a variable. Example: var image1:m = grayscale(file_001)", identifier));
                }
            },
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
        _memories: &mut Memory,
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
        memories: &mut Memory,
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

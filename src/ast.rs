// use super::Compiler::*;
use super::*;
use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};
use walrus::ir::*;
use walrus::FunctionId;
use walrus::InstrSeqBuilder;
use walrus::*;

log_rule!();

pub struct ItemTracker {
    images: HashMap<String, (usize, bool)>,
    pub image_id: usize,
}
impl ItemTracker {
    pub fn new() -> ItemTracker {
        ItemTracker {
            images: HashMap::new(),
            image_id: 0,
        }
    }
    pub fn add_image(&mut self, typ: &str, name: Option<String>, id: Option<usize>, export: bool) {
        match typ {
            "import" => {
                self.images.insert(
                    name.unwrap_or_default().clone(),
                    (self.image_id.clone(), export),
                );
                self.image_id += 1;
            }
            "compile" => match id {
                Some(id_) => {
                    self.images
                        .insert(name.unwrap_or_default().clone(), (id_, export));
                }
                None => match name {
                    Some(n) => {
                        self.images
                            .insert(n.clone(), (self.image_id.clone() - 1, export));
                    }
                    None => {
                        self.images
                            .insert(self.image_id.to_string(), (self.image_id.clone(), false));
                        self.image_id += 1;
                    }
                },
            },
            _ => {}
        }
    }

    pub fn find_image(&self, name: &String) -> Option<&(usize, bool)> {
        self.images.get(name)
    }

    pub fn get_image_names(&self) -> &HashMap<String, (usize, bool)> {
        &self.images
    }
}

fn variable_dependency_add(
    child_name: String,
    parents_name: Vec<String>,
    variable_dependency: &mut HashMap<String, Vec<String>>,
) -> Result<String, String> {
    let dpd = variable_dependency
        .entry(child_name.clone())
        .or_insert(vec![]);
    for d in parents_name {
        dpd.push(d.clone());
    }
    let result = variable_dependency_check(child_name.clone(), variable_dependency);
    if result.is_err() {
        return Err("err".to_string());
    } else {
        return Ok("ok".to_string());
    }
}

fn variable_dependency_check(
    name: String,
    variable_dependency: &HashMap<String, Vec<String>>,
) -> Result<(), Vec<String>> {
    let mut visited: HashMap<String, bool> = HashMap::new();
    let mut cycle: Vec<String> = vec![];
    if variable_has_cycle(name, &mut visited, variable_dependency, &mut cycle) {
        log(&format!(
            "Error: Cycle detected in variables, involving {:?}",
            cycle
        ));
        return Err(cycle);
    }
    Ok(())
}

fn variable_has_cycle(
    name: String,
    visited: &mut HashMap<String, bool>,
    variable_dependency: &HashMap<String, Vec<String>>,
    cycle: &mut Vec<String>,
) -> bool {
    if let Some(&k) = visited.get(&name) {
        if k {
            cycle.push(name.clone());
        }
        return k;
    }
    let v = visited.entry(name.clone()).or_insert(true);
    *v = true;

    if let Some(children) = variable_dependency.get(&name) {
        for child in children {
            if variable_has_cycle((*child).clone(), visited, variable_dependency, cycle) {
                return true;
            }
        }
    }
    let v = visited.get_mut(&(name.clone())).unwrap();
    *v = false;
    false
}

pub trait Compile {
    fn compile(
        &self,
        module: &mut walrus::Module,
        builder: &mut InstrSeqBuilder,
        local_ids: &mut HashMap<String, (String, LocalId)>,
        function_ids: &HashMap<String, (FunctionId, Vec<String>)>,
        variable_dependency: &mut HashMap<String, Vec<String>>,
        item_tracker: &mut ItemTracker,
    );
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
        local_ids: &mut HashMap<String, (String, LocalId)>,
        function_ids: &HashMap<String, (FunctionId, Vec<String>)>,
        variable_dependency: &mut HashMap<String, Vec<String>>,
        item_tracker: &mut ItemTracker,
    ) {
        use self::Expr::*;
        match *self {
            Number(n) => {
                builder.i32_const(n);
            }
            Op(ref l, op, ref r) => {
                l.compile(
                    module,
                    builder,
                    local_ids,
                    function_ids,
                    variable_dependency,
                    item_tracker,
                );
                r.compile(
                    module,
                    builder,
                    local_ids,
                    function_ids,
                    variable_dependency,
                    item_tracker,
                );
                op.compile(
                    module,
                    builder,
                    local_ids,
                    function_ids,
                    variable_dependency,
                    item_tracker,
                );
            }
            Variable(ref identifier) => {
                if let Some((type_name, local_id)) = (*local_ids).get(identifier) {
                    match type_name.as_str() {
                        "Image" | "Number" => {
                            builder.local_get(*local_id);
                        }
                        _ => {
                            log(&format!(
                                "Error: variable {:?} is neither an image nor a number.",
                                identifier
                            ));
                            return;
                        }
                    }
                } else {
                    log(&format!("Error:  variable {:?} doesn't exist.", identifier));
                }
            }
            Call(ref identifier, ref exprs) => {
                if let Some((func_id, params_type)) = function_ids.get(identifier) {
                    if exprs.len() != params_type.len() {
                        log(&format!(
                            "Error: function {:?} should take {:?} parameters instead of {:?} ",
                            identifier,
                            params_type.len(),
                            exprs.len()
                        ));
                        return;
                    }

                    for params_index in 0..params_type.len() {
                        let expr = &exprs[params_index];
                        let params_type = &params_type[params_index];

                        match **expr {
                            Number(n) => {
                                //?
                                if params_type.to_string() != "Number".to_string() {
                                    log(&format!("Error: {:?} should have type of a Number", expr));
                                } else {
                                    builder.i32_const(n);
                                }
                            }
                            Variable(ref ident) => {
                                if let Some((expr_type, expr_id)) = (*local_ids).get(ident) {
                                    match expr_type.as_str() {
                                        "Image" => {
                                            if params_type.to_string() == "Image".to_string() {
                                                builder.local_get(*expr_id);
                                            } else {
                                                log(&format!(
                                                    "Error: {:?} should have type of an Image",
                                                    expr
                                                ));
                                            }
                                        }
                                        "Number" => {
                                            if params_type.to_string() == "Number".to_string() {
                                                builder.local_get(*expr_id);
                                            } else {
                                                log(&format!(
                                                    "Error: {:?} should have type of a Number",
                                                    expr
                                                ));
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            Call(ref _inside_identifier, ref _inside_exprs) => {
                                expr.compile(
                                    module,
                                    builder,
                                    local_ids,
                                    function_ids,
                                    variable_dependency,
                                    item_tracker,
                                );
                                item_tracker.add_image("compile", None, None, true);
                            }
                            _ => {}
                        }
                    }

                    builder.call(*func_id);
                    item_tracker.add_image("compile", None, None, false);
                } else {
                    log(&format!(
                        "Error: function {:?} doesn't exist. Please try using an existing function from the library.",
                        identifier
                    ));
                    return;
                }
            }
            _ => {}
        }
    }
}

// #[derive(Debug)]
#[derive(Clone)]
pub struct VarDef {
    pub identifier: String,
    pub expr: Box<Expr>,
}

impl VarDef {
    pub fn new(identifier: String, expr: Box<Expr>) -> Self {
        VarDef { identifier, expr }
    }
}

impl Debug for VarDef {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            // Error => write!(fmt, "error"),
            _ => write!(
                fmt,
                "identifier: {:?}, expression: {:?}",
                self.identifier, self.expr
            ),
        }
    }
}

impl Compile for VarDef {
    fn compile(
        &self,
        module: &mut walrus::Module,
        builder: &mut InstrSeqBuilder,
        local_ids: &mut HashMap<String, (String, LocalId)>,
        function_ids: &HashMap<String, (FunctionId, Vec<String>)>,
        variable_dependency: &mut HashMap<String, Vec<String>>,
        item_tracker: &mut ItemTracker,
    ) {
        if self.identifier.parse::<i32>().is_ok() {
            log(&format!(
                "Error: please use a non-numeric name for a variable."
            ));
            return;
        }
        match &*(self.expr) {
            Expr::Number(n) => {
                builder.i32_const(*n);
                let new_id = module.locals.add(ValType::I32);
                builder.local_set(new_id);
                local_ids.insert(self.identifier.to_string(), ("Number".to_string(), new_id));
            }
            Expr::Variable(var_right) => {
                if let Some((var_right_type, var_right_id)) = (*local_ids).get(var_right) {
                    match var_right_type.as_str() {
                        "Image" => {
                            builder.local_get(*var_right_id);
                            let new_id = module.locals.add(ValType::I32);
                            builder.local_set(new_id);
                            local_ids
                                .insert(self.identifier.to_string(), ("Image".to_string(), new_id));

                            let (expr_image_id, _) = match item_tracker.find_image(var_right) {
                                Some(v) => v.clone(),
                                None => {
                                    log(&format!(
                                        "None matched, var_right: {:?}, local ids:{:?}",
                                        var_right, local_ids
                                    ));
                                    (0, false)
                                }
                            };
                            item_tracker.add_image(
                                "compile",
                                Some(self.identifier.clone()),
                                Some(expr_image_id),
                                true,
                            )
                        }
                        "Number" => {
                            builder.local_get(*var_right_id);
                            let new_id = module.locals.add(ValType::I32);
                            builder.local_set(new_id);
                            local_ids.insert(
                                self.identifier.to_string(),
                                ("Number".to_string(), new_id),
                            );
                        }
                        _ => {
                            log(&format!(
                                "Error: please define {:?} with an image or number.",
                                var_right
                            ));
                            return;
                        }
                    }

                    if variable_dependency_add(
                        self.identifier.clone(),
                        vec![var_right.clone()],
                        variable_dependency,
                    )
                    .is_err()
                    {
                        local_ids.remove(&self.identifier);
                        return;
                    }
                } else {
                    log(&format!(
                        "Error:  variable {:?} doesn't exist, please define {:?} with something else.",
                        var_right, self.identifier
                    ));
                    return;
                }
            }
            Expr::Call(_ident, _exprs) => {
                let new_id = module.locals.add(ValType::I32);
                self.expr.compile(
                    module,
                    builder,
                    local_ids,
                    function_ids,
                    variable_dependency,
                    item_tracker,
                );
                builder.i32_const(item_tracker.image_id as i32 - 1);
                builder.local_set(new_id);
                local_ids.insert(self.identifier.clone(), ("Image".to_string(), new_id));
                item_tracker.add_image("compile", Some(self.identifier.clone()), None, true);
            }
            _ => {
                log(&format!(
                    "Error: please define {:?} with a number, image or call to a function.",
                    self.identifier
                ));
                return;
            }
        }
    }
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
        _local_ids: &mut HashMap<String, (String, LocalId)>,
        _function_ids: &HashMap<String, (FunctionId, Vec<String>)>,
        _variable_dependency: &mut HashMap<String, Vec<String>>,
        _item_tracker: &mut ItemTracker,
    ) {
        use self::Opcode::*;
        match *self {
            Mul => builder.binop(BinaryOp::I32Mul),
            Div => builder.binop(BinaryOp::I32Mul),
            Add => builder.binop(BinaryOp::I32Mul),
            Sub => builder.binop(BinaryOp::I32Mul),
        };
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
        local_ids: &mut HashMap<String, (String, LocalId)>,
        _function_ids: &HashMap<String, (FunctionId, Vec<String>)>,
        _variable_dependency: &mut HashMap<String, Vec<String>>,
        _item_tracker: &mut ItemTracker,
    ) {
        for param in &self.params {
            let (_, id) = local_ids[param]; //?
            builder.local_set(id); //?
        }
    }
}

#[derive(Debug)]
pub struct Function {
    pub prototype: Prototype,
    pub vardefs: Vec<VarDef>,
}

impl Function {
    pub fn new(prototype: Prototype, vardefs: Vec<VarDef>) -> Self {
        Function { prototype, vardefs }
    }
}

impl Compile for Function {
    fn compile(
        &self,
        module: &mut walrus::Module,
        builder: &mut InstrSeqBuilder,
        local_ids: &mut HashMap<String, (String, LocalId)>,
        function_ids: &HashMap<String, (FunctionId, Vec<String>)>,
        variable_dependency: &mut HashMap<String, Vec<String>>,
        item_tracker: &mut ItemTracker,
    ) {
        self.prototype.compile(
            module,
            builder,
            local_ids,
            function_ids,
            variable_dependency,
            item_tracker,
        );
        for vardef in &self.vardefs {
            vardef.compile(
                module,
                builder,
                local_ids,
                function_ids,
                variable_dependency,
                item_tracker,
            );
        }
    }
}

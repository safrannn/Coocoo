use super::ast::*;
use super::*;
use id_arena::*;
use std::collections::HashMap;

log_rule!();

pub struct SymbolTable {
    table: HashMap<String, Attribute>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            table: HashMap::new(),
        }
    }

    pub fn insert(&mut self, symbol: String, attr: Attribute) {
        self.table.insert(symbol.clone(), attr);
    }

    pub fn lookup(&self, symbol: String) -> Option<&Attribute> {
        self.table.get(&symbol)
    }

    pub fn free(&mut self) {
        self.table.clear();
    }
}

pub enum Attribute {
    NumberAttr(i32),
    OpAttr(Box<Expr>, Opcode, Box<Expr>), //left, op, right
    ImageAttr(Id<walrus::Local>, Image),  // name, local_id, image info
    MaterialAttr(Id<walrus::Local>, i32, Material), //array length, material info
    CallAttr(Id<walrus::Function>, Vec<Box<Expr>>, Vec<Box<Expr>>), //func_id, params, return
    Error(),
}

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
#[derive(Clone, Copy)]
pub struct Image {
    pub image_id: i32,
    pub export: bool, // if need export
}

impl Image {
    pub fn new(image_id_: i32, export_: bool) -> Self {
        return Image {
            image_id: image_id_,
            export: export_,
        };
    }
}

pub struct ItemTracker {
    images: HashMap<String, Image>, //name, (id, if export)
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
                    Image::new(self.image_id.clone() as i32, export),
                );
                self.image_id += 1;
            }
            "compile" => match id {
                Some(id_) => {
                    self.images.insert(
                        name.unwrap_or_default().clone(),
                        Image::new(id_ as i32, export),
                    );
                }
                None => match name {
                    Some(n) => {
                        self.images.insert(
                            n.clone(),
                            Image::new((self.image_id.clone() - 1) as i32, export),
                        );
                    }
                    None => {
                        self.images.insert(
                            self.image_id.to_string(),
                            Image::new(self.image_id.clone() as i32, false),
                        );
                        self.image_id += 1;
                    }
                },
            },
            _ => {}
        }
    }

    pub fn find_image(&self, name: &String) -> Option<&Image> {
        self.images.get(name)
    }

    pub fn get_image_names(&self) -> &HashMap<String, Image> {
        &self.images
    }
}

type PBRMetalness = [Image; 11]; // diffuse, metalness, specular, normal, transparency, roughness, ambient_occlusion, displacement, emission, cavity, subsurfance_scattering

pub enum Material {
    PBRMetalMaterial(PBRMetalness),
}

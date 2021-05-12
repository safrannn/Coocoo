use super::*;
use id_arena::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

log_rule!();

#[derive(Debug, Clone)]
pub struct SymbolTable {
    table: HashMap<String, Attribute>,
    pub library_tracker: LibraryTracker,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            table: HashMap::new(),
            library_tracker: LibraryTracker::new(),
        }
    }

    pub fn insert(&mut self, ident: String, attr: Attribute) {
        match &attr {
            Attribute::Image(_, image) => {
                self.library_tracker
                    .add_image(Some(ident.clone()), image.clone());
            }
            Attribute::Material(_, offset, material_type) => {
                self.library_tracker.add_material(
                    ident.clone(),
                    offset.clone(),
                    material_type.clone(),
                );
            }
            _ => {}
        }
        self.table.insert(ident.clone(), attr);
    }

    pub fn lookup(&self, ident: &String) -> Option<&Attribute> {
        self.table.get(ident)
    }

    pub fn update(&mut self, ident: &String, new_attr: Attribute) -> Result<(), ()> {
        if self.lookup(ident).is_none() {
            return Err(());
        }

        *self
            .table
            .entry(ident.to_string())
            .or_insert(Attribute::Empty()) = new_attr.clone();

        match new_attr {
            Attribute::Image(_, image) => {
                self.library_tracker
                    .add_image(Some(ident.clone()), image.clone());
            }
            Attribute::Material(_, _, new_material_type) => {
                self.library_tracker
                    .update_material_type(ident, new_material_type.clone());
            }
            _ => {}
        }

        return Ok(());
    }

    pub fn remove(&mut self, ident: &String) {
        self.table.remove(ident);
    }

    pub fn free(&mut self) {
        self.table.clear();
    }
}

#[derive(Clone, Debug)]
pub enum Attribute {
    Number(Id<walrus::Local>),
    Image(Id<walrus::Local>, Option<Image>), // local_id, image info
    Material(walrus::MemoryId, u32, String), // memoryid, offset, material type
    Func(
        id_arena::Id<walrus::Function>,
        Vec<walrus::ValType>,
        Vec<walrus::ValType>,
    ), //func_id, arguments, return
    Empty(),
    Error(),
}

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
#[derive(Debug, Clone, Copy)]
pub struct Image {
    pub id: i32,
}

impl Image {
    pub fn new(image_id_: i32) -> Self {
        return Image { id: image_id_ };
    }

    pub fn copy(&self) -> Image {
        return Image {
            id: self.id.clone(),
        };
    }
}

#[derive(Debug, Clone)]
pub struct LibraryTracker {
    images: HashMap<String, Image>,
    pub next_image_id: i32,
    texture_exports: HashMap<String, i32>, // name, image_id
    pub material_info: MaterialInfo,
    materials: HashMap<String, (u32, String)>, // name, (starting offset, type)
}

impl LibraryTracker {
    pub fn new() -> LibraryTracker {
        LibraryTracker {
            images: HashMap::new(),
            next_image_id: 0,
            texture_exports: HashMap::new(),
            material_info: MaterialInfo::new(),
            materials: HashMap::new(),
        }
    }

    pub fn add_image(&mut self, name: Option<String>, image: Option<Image>) {
        if name.is_some() && image.is_some() {
            let id = image.unwrap().id;
            self.images.insert(name.unwrap().clone(), Image::new(id));
            if id >= self.next_image_id {
                self.next_image_id += 1;
            }
        } else if name.is_some() && image.is_none() {
            self.images.insert(
                name.unwrap().clone(),
                Image::new(self.next_image_id.clone() - 1),
            );
        } else if name.is_none() && image.is_none() {
            self.images.insert(
                self.next_image_id.to_string(),
                Image::new(self.next_image_id.clone()),
            );
            self.next_image_id += 1;
        }
    }

    pub fn find_image(&self, name: &String) -> Option<&Image> {
        self.images.get(name)
    }

    pub fn get_next_image_id(&self) -> i32 {
        return self.next_image_id;
    }

    pub fn add_export_image(&mut self, image_name: String) {
        if let Some(image) = self.images.get(&image_name) {
            self.texture_exports.insert(image_name.clone(), image.id);
        }
    }

    pub fn export_textures(&self) -> JsValue {
        JsValue::from_serde(&self.texture_exports).unwrap()
    }

    pub fn add_material(&mut self, name: String, offset: u32, material_type: String) {
        self.materials.insert(name, (offset, material_type));
    }

    pub fn update_material_type(&mut self, name: &String, new_material_type: String) {
        // self.materials.insert(name, (offset, material_type));// name, (starting offset, type)
        let old_info = self
            .materials
            .entry(name.to_string())
            .or_insert((0, "".to_string()));
        *old_info = (old_info.0, new_material_type);
    }

    pub fn export_materials(&mut self) -> JsValue {
        let mut result: HashMap<i32, Vec<String>> = HashMap::new();
        let align = 2;
        for (name, (start, material_type)) in &self.materials {
            if let Ok(channels) = self.material_info.get_material_channels(material_type) {
                let mut position = (*start as i32) / u32::pow(2, align) as i32 + 3;
                for channel in channels {
                    result.insert(position, vec![name.clone(), channel, material_type.clone()]);
                    position += 1;
                }
            }
        }

        JsValue::from_serde(&result).unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct MaterialInfo {
    channel_info: HashMap<String, (i32, Vec<String>)>,
}

impl MaterialInfo {
    fn new() -> Self {
        let mut channel_info: HashMap<String, (i32, Vec<String>)> = HashMap::new();
        let mut id = 0;
        channel_info.insert(
            "PBRMetalness".to_string(),
            (
                id,
                vec![
                    "diffuse".to_string(),
                    "metalness".to_string(),
                    "normal".to_string(),
                    "transparency".to_string(),
                    "roughness".to_string(),
                    "ao".to_string(),
                    "displacement".to_string(),
                    "emissive".to_string(),
                    "cavity".to_string(),
                    "subsurfance_scattering".to_string(),
                ],
            ),
        );
        id += 1;
        channel_info.insert(
            "PBRSpecular".to_string(),
            (
                id,
                vec![
                    "albedo".to_string(),
                    "specular".to_string(),
                    "normal".to_string(),
                    "transparency".to_string(),
                    "glosiness".to_string(),
                    "ao".to_string(),
                    "displacement".to_string(),
                    "emissive".to_string(),
                    "cavity".to_string(),
                    "subsurfance_scattering".to_string(),
                ],
            ),
        );
        id += 1;
        channel_info.insert(
            "UnityStandardSpecular".to_string(),
            (
                id,
                vec![
                    "albedo".to_string(),
                    "specular".to_string(),
                    "normal".to_string(),
                    "height".to_string(),
                    "ao".to_string(),
                    "emissive".to_string(),
                ],
            ),
        );
        id += 1;
        return MaterialInfo { channel_info };
    }

    pub fn find_channel_index(&self, material_type: &String, channel: &String) -> Result<u32, ()> {
        if let Some((_, channels)) = self.channel_info.get(material_type) {
            for (i, v) in channels.iter().enumerate() {
                if v == channel {
                    return Ok(i as u32);
                }
            }
        }
        return Err(());
    }

    pub fn get_material_channels(&self, material_type: &String) -> Result<Vec<String>, ()> {
        if let Some((_, channels)) = self.channel_info.get(material_type) {
            return Ok(channels.to_vec());
        } else {
            return Err(());
        }
    }

    pub fn get_material_type_id(&self, material_type: &String) -> Result<i32, ()> {
        if let Some(material_info) = self.channel_info.get(material_type) {
            return Ok(material_info.0);
        } else {
            return Err(());
        }
    }
}

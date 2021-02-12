use super::*;
use id_arena::*;
use std::collections::HashMap;

log_rule!();

#[derive(Clone)]
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
        match attr {
            Attribute::Image(_, image) => {
                self.library_tracker
                    .add_image(Some(ident.clone()), image.clone());
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

        match new_attr {
            Attribute::Image(_, image) => {
                self.library_tracker
                    .add_image(Some(ident.clone()), image.clone());
            }
            _ => {}
        }
        *self
            .table
            .entry(ident.to_string())
            .or_insert(Attribute::Empty()) = new_attr;
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
#[derive(Clone, Copy, Debug)]
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

#[derive(Clone)]
pub struct LibraryTracker {
    images: HashMap<String, Image>,
    pub next_image_id: i32,
    material: HashMap<String, Material>,
}

impl LibraryTracker {
    pub fn new() -> LibraryTracker {
        LibraryTracker {
            images: HashMap::new(),
            next_image_id: 0,
            material: HashMap::new(),
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
}

#[derive(Clone)]
pub struct Material {
    channel_info: HashMap<String, Vec<&'static str>>,
}

impl Material {
    fn new() -> Self {
        let mut channel_info: HashMap<String, Vec<&'static str>> = HashMap::new();
        channel_info.insert(
            "PBRMetalness".to_string(),
            vec![
                "diffuse",
                "metalness",
                "specular",
                "normal",
                "transparency",
                "roughness",
                "ambient_occlusion",
                "displacement",
                "emission",
                "cavity",
                "subsurfance_scattering",
            ],
        );
        return Material { channel_info };
    }

    pub fn find_channel_index(&self, type_name: &String, channel: &String) -> Result<u32, ()> {
        if let Some(channels) = self.channel_info.get(type_name) {
            for (i, &v) in channels.iter().enumerate() {
                if v == channel {
                    return Ok(i as u32);
                }
            }
        }
        return Err(());
    }
}

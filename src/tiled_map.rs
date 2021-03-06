extern crate tmx;
extern crate find_folder;

use specs::*;

#[derive(Debug)]
pub struct Map {
    pub tmx: tmx::Map,
    pub dimension: (u32, u32),
    pub tile_size: (u32, u32),
    pub tileset_file: String,
}

impl Component for Map {
    type Storage = HashMapStorage<Map>;
}

pub struct MapBuilder {
    map_name: Option<String>,
}

impl MapBuilder {
    pub fn new() -> MapBuilder {
        MapBuilder { map_name: None }
    }

    pub fn name(mut self, name: &String) -> MapBuilder {
        self.map_name = Some(name.clone());
        self
    }

    pub fn build(self) -> Map {

        let mut assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();

        assets.push(self.map_name.unwrap());

        let tmx_map = match tmx::Map::open(assets.as_path()) {
            Ok(map) => map,
            Err(e) => panic!("Got an error: {}", e),
        };

        let mut src: Option<String> = None;

        for set in tmx_map.tilesets() {
            src = Some(String::from(set.image().unwrap().source()));
            break; // Hence will work only with one tileset
        }

        let dimension = (tmx_map.width(), tmx_map.height());
        let tile_size = (tmx_map.tile_width(), tmx_map.tile_height());

        Map {
            tmx: tmx_map,
            tileset_file: src.unwrap(),
            dimension: dimension,
            tile_size: tile_size,
        }
    }
}

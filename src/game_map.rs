// use std::borrow::Borrow;
// use std::hash::{Hash, Hasher};
use std::collections::HashMap;

use crate::color_scheme::{ColorName, get_stone_color};
use noise::{Fbm, NoiseFn, ScalePoint};
use quicksilver::prelude::*;

pub struct GameMap {
    map: HashMap<u32, HashMap<u32, HashMap<u32, HashMap<u32, Vec<Tile>>>>>,
    pub chunk_size: u32,
    pub max_chuncks_x: u32,
    pub max_chuncks_y: u32,
    pub max_chuncks_z: u32,
    pub surface_level: u32,
}

impl GameMap {
    pub fn new() -> GameMap {
        let chunk_size: u32 = 64;
        let planet_circumference: u32 = 20000000;
        let planet_crust_thickness: u32 = 32000;
        let surface_level: u32 = 1000;
        let max_chuncks_x: u32 = planet_circumference / chunk_size;
        let max_chuncks_y: u32 = planet_circumference / chunk_size;
        let max_chuncks_z: u32 = planet_crust_thickness / chunk_size;
        GameMap {
            map: HashMap::with_capacity(chunk_size as usize),
            chunk_size,
            max_chuncks_x,
            max_chuncks_y,
            max_chuncks_z,
            surface_level,
        }
    }

    pub fn get_tile(&mut self, x: u32, y:u32, z:u32) -> Tile {
        //println!("get_tile x: {:?}, y: {:?}, z: {:?}", x, y, z);
        //println!("map.keys: {:?}", self.map);

        let chunk_size = self.chunk_size as usize;
        let (x_min, x_max, y_min, y_max, z_min, z_max) = GameMap::get_chunck_boundries(x, y, z, chunk_size as u32);
        //println!("x_min: {:?}, x_max: {:?}, y_min: {:?}, y_max: {:?}, z_min: {:?}, z_max: {:?}", x_min, x_max, y_min, y_max, z_min, z_max);
        let calculate_center = |min, size| {min + size/2};
        let center_x = calculate_center(x_min, chunk_size as u32);
        let center_y = calculate_center(y_min, chunk_size as u32);
        let center_z = calculate_center(z_min, chunk_size as u32);

        //println!("center x: {:?}, y: {:?}, z: {:?}", center_x,center_y,center_z);
        //println!("map.keys: {:?}", self.map.keys());
        if !self.map.contains_key(&center_x) {
            self.map.insert(center_x, HashMap::with_capacity(chunk_size));
        }
        let x_map = self.map.get_mut(&center_x).unwrap();
        //println!("x_map.len: {:?}", x_map.len());

        if !x_map.contains_key(&center_y) {
            x_map.insert(center_y, HashMap::with_capacity(chunk_size));
        }
        let y_map = x_map.get_mut(&center_y).unwrap();
        //println!("y_map.len: {:?}", y_map.len());
 
        if !y_map.contains_key(&center_z) {
            y_map.insert(center_z, GameMap::generate_map_chunk(
                    HashMap::with_capacity(chunk_size),
                    x_min, x_max, 
                    y_min, y_max, 
                    z_min, z_max,
                    &chunk_size)
                );
        }
        let chunk = &y_map.get(&center_z).unwrap();
        //println!("z_map.len: {:?}", z_map.len());
        
        let chunk_size_32 = self.chunk_size as u32;
        let chunk_x = x % chunk_size_32;
        let chunk_y = y % chunk_size_32;
        let chunk_z = z % chunk_size_32;
        let chunk_plane = &chunk.get(&chunk_z).unwrap();
        let i = (chunk_x + chunk_y * chunk_size_32) as usize;
        //println!("i: {:?}", i);
        
        //println!("get_tile returning tile: {:?}", map_plane[i]);
        chunk_plane[i]
    }

    pub fn generate_map_chunk(mut map: HashMap<u32, Vec<Tile>>,
                              x_min: u32, x_max: u32, 
                              y_min: u32, y_max: u32, 
                              z_min: u32, z_max: u32,
                              &chunk_size: &usize,
                              ) -> HashMap<u32, Vec<Tile>>{
        // Generate the chunk that x, y, z is located in
        // Get a block of 3d ridge noise, custom settings, 32x32x32 unscaled
        //let noise = NoiseBuilder::ridge_3d(width, height, depth)
        //let noise = NoiseBuilder::ridge_3d(width, height, depth)
            //.generate_scaled(-1.0, 1.0);
            //.with_freq(0.05)
            //.with_octaves(5)
            //.with_gain(2.0)
            //.with_seed(1337)
            //.with_lacunarity(0.5)

        //println!("chunk_size: {:?}", chunk_size);
        //println!("x_min: {:?}", x_min);
        //println!("x_max: {:?}", x_max);
        //println!("y_min: {:?}", y_min);
        //println!("y_max: {:?}", y_max);
        //println!("z_min: {:?}", z_min);
        //println!("z_max: {:?}", z_max);

        let noise_gen = ScalePoint::new(Fbm::new()).set_scale(0.1);
        for z in (z_min..z_max).rev() {
            let mut z_map = Vec::with_capacity(chunk_size * chunk_size);
            for y in y_min..y_max {
                for x in x_min..x_max {
                    //let val = noise.get((x % width) + (y % height) + (depth % depth)).unwrap();
                    let val = noise_gen.get([x as f64, y as f64, z as f64]);
                    //println!("{}", val);
                    //println!("x, y, z: {:?}, {:?}, {:?}", x, y, z);
                    
                    let mut tile = Tile {
                        pos: Vector::new(x as f32, y as f32),
                        depth: z,
                        glyph: '.',
                        color: get_stone_color(&val, &0.0, &1.0),
                    };

                    if val.abs() >= 0.2 {
                        tile.glyph = '#';
                    }
                    z_map.push(tile);
                }
            }
            map.insert(z % chunk_size as u32, z_map);
        }
        map
    }

    /// Given x, y, z and chunk_size returns the boundries of the 
    /// chunk_size x chunk_size x chunk_size chunk that (x, y, z) 
    /// is located in
    /// Return (x_min, x_max, y_min, y_max, z_min, z_max)
    fn get_chunck_boundries(x: u32, y: u32, z: u32, chunk_size: u32) -> (u32, u32, u32, u32, u32, u32) {

        // prevents off-by-one errors when 
        // coordinates fall directly on chunk boundries
        let cx = x + 1;
        let cy = y + 1;
        let cz = z + 1;

        let (x_min, x_max) = GameMap::round_to_boundries(cx, chunk_size);
        let (y_min, y_max) = GameMap::round_to_boundries(cy, chunk_size);
        let (z_min, z_max) = GameMap::round_to_boundries(cz, chunk_size);
        
        (x_min, x_max, y_min, y_max, z_min, z_max)
    }

    /// Find the nearest multiples of m that n is located between. Ex
    /// round_to_boundries(100, 64) should return (64, 128), the two
    /// multiples of 64 that 100 is located between.
    /// Return (n_min, n_max)
    fn round_to_boundries(n: u32, m: u32) -> (u32, u32) {
        if n == 0 {                                                       
            (0, m)
        } else {
            let max = ((n + m - 1) / m) * m;
            let min = max - m;
            (min, max)
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tile {
    pub pos: Vector,
    pub depth: u32,
    pub glyph: char,
    pub color: ColorName,
}

#[cfg(test)]
mod tests {
    // Import names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_round_to_boundries_0() {
        let n = 0;
        let m = 64;
        let (min, max) = GameMap::round_to_boundries(n, m);
        println!("min: {:?} == 0", min);
        println!("max: {:?} == 64", max);
        assert_eq!(min, 0);
        assert_eq!(max, 64);
    }

    #[test]
    fn test_round_to_boundries_1() {
        let n = 1;
        let m = 64;
        let (min, max) = GameMap::round_to_boundries(n, m);
        println!("min: {:?} == 0", min);
        println!("max: {:?} == 64", max);
        assert_eq!(min, 0);
        assert_eq!(max, 64);
    }

    #[test]
    fn test_round_to_boundries_63() {
        let n = 63;
        let m = 64;
        let (min, max) = GameMap::round_to_boundries(n, m);
        println!("min: {:?} == 0", min);
        println!("max: {:?} == 64", max);
        assert_eq!(min, 0);
        assert_eq!(max, 64);
    }

    #[test]
    fn test_round_to_boundries_64() {
        let n = 64;
        let m = 64;
        let (min, max) = GameMap::round_to_boundries(n, m);
        println!("min: {:?} == 0", min);
        println!("max: {:?} == 64", max);
        assert_eq!(min, 0);
        assert_eq!(max, 64);
    }

    #[test]
    fn test_round_to_boundries_128() {
        let n = 128;
        let m = 64;
        let (min, max) = GameMap::round_to_boundries(n, m);
        println!("min: {:?} == 64", min);
        println!("max: {:?} == 128", max);
        assert_eq!(min, 64);
        assert_eq!(max, 128);
    }

    #[test]
    fn test_round_to_boundries_129() {
        let n = 129;
        let m = 64;
        let (min, max) = GameMap::round_to_boundries(n, m);
        println!("min: {:?} == 128", min);
        println!("max: {:?} == 192", max);
        assert_eq!(min, 128);
        assert_eq!(max, 192);
    }

    #[test]
    fn test_round_to_boundries_192() {
        let n = 192;
        let m = 64;
        let (min, max) = GameMap::round_to_boundries(n, m);
        println!("min: {:?} == 128", min);
        println!("max: {:?} == 192", max);
        assert_eq!(min, 128);
        assert_eq!(max, 192);
    }

    #[test]
    fn test_round_to_boundries_175() {
        let n = 175;
        let m = 64;
        let (min, max) = GameMap::round_to_boundries(n, m);
        println!("min: {:?} == 128", min);
        println!("max: {:?} == 192", max);
        assert_eq!(min, 128);
        assert_eq!(max, 192);
    }
}

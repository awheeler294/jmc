use enum_map::{enum_map, Enum, EnumMap};
use quicksilver::prelude::*;

use std::collections::HashMap;
use std::time::{Duration, Instant};

mod game_map;
mod color_scheme;

const FONT_MONONOKI: &'static str = "mononoki-Regular.ttf";
const FONT_SQUARE: &'static str = "square.ttf";
const FONT_ZODIAC_SQUARE: &'static str = "zodiac-square.ttf";

#[derive(Clone, Copy, Debug, PartialEq)]
struct Position<T> {
    x: T,
    y: T,
    z: T,
}

struct Camera {
    position: Position<u32>,
    viewport_size: Vector,
    max_size_x: u32,
    max_size_y: u32,
    max_size_z: u32,
}

impl Camera {
    fn move_left(&mut self) {
        if self.position.x > 0 {
            self.position.x -= 1;
        }
    }

    fn move_right(&mut self) {
        if self.position.x < self.max_size_x {
            self.position.x += 1;
        }
    }

    fn move_up(&mut self) {
        if self.position.y > 0 {
            self.position.y -= 1;
        }
    }

    fn move_down(&mut self) {
        if self.position.y < self.max_size_y {
            self.position.y += 1;
        }
    }

    fn elevate(&mut self) {
        if self.position.z > 0 {
            self.position.z -= 1;
        }
    }

    fn lower(&mut self) {
        if self.position.z < self.max_size_z {
            self.position.z += 1;
        }
    }

    fn go_to(&mut self, x: u32, y: u32, z: u32) {
        if x <= self.max_size_x && 
           y <= self.max_size_y && 
           z <= self.max_size_z {
           self.position.x = x;
           self.position.y = y;
           self.position.z = z;
        }
    }
}

struct Tileset {
    tile_map: HashMap<char, Image>,
}

impl Tileset {

    fn new(glyph_map: Vec<(String, String)>, tile_size_px: Vector) -> Tileset {
        Tileset {
            tile_map: Tileset::render(glyph_map, tile_size_px),
        }
    }
    
    fn render(glyph_map: Vec<(String, String)>, tile_size_px: Vector) 
        -> HashMap<char, Image> {

        let mut tile_map = HashMap::new();
        for (font_name, glyphs) in glyph_map {
            tile_map.extend(Font::load(font_name).and_then(move |font: Font| {
                let tiles = font
                    .render(glyphs.as_str(), &FontStyle::new(tile_size_px.y, Color::WHITE))
                    .expect("could not render tileset.");
                let mut tile_map = HashMap::new();
                for (index, glyph) in glyphs.chars().enumerate() {
                    let pos = (index as u32 * tile_size_px.x as u32, 0);
                    let tile = tiles.subimage(Rectangle::new(pos, tile_size_px));
                    tile_map.insert(glyph, tile);
                }
                Ok(tile_map)
            }).wait().unwrap());
        }

        tile_map

    }

}

#[derive(Enum)]
enum UiComponent {
    Map,
    Title,
    Credits,
    Debug,
}

struct Game {
    title: Asset<Image>,
    font_info: Vec<Asset<Image>>,
    map: game_map::GameMap,
    entities: Vec<Entity>,
    player_id: usize,
    tileset: Tileset,
    tile_size_px: Vector,
    color_scheme: color_scheme::ColorScheme,
    camera: Camera,
    ui_components: EnumMap<UiComponent, bool>,
    input_timer: Instant,
}

impl State for Game {
    /// Load the assets and initialize the game
    fn new() -> Result<Self> {
        let color_scheme = color_scheme::ColorScheme {
            bg: String::from("#282828"),
            fg: String::from("#ebdbb2"),
            fg0: String::from("#fbf1c7"),
            fg1: String::from("#ebdbb2"),
            fg2: String::from("#d5c4a1"),
            fg3: String::from("#bdae93"),
            fg4: String::from("#a89984"),
            gray: String::from("#a89984"),
            light_gray: String::from("#928374"),
            red: String::from("#cc241d"),
            light_red: String::from("#fb4934"),
            green: String::from("#98971a"),
            light_green: String::from("#b8bb26"),
            yellow: String::from("#d79921"),
            light_yellow: String::from("#fabd2f"),
            blue: String::from("#458588"),
            light_blue: String::from("#83a598"),
            purple: String::from("#b16286"),
            light_purple: String::from("#d3869b"),
            aqua: String::from("#689d6a"),
            light_aqua: String::from("#8ec07c"),
            orange: String::from("#d65d0e"),
            light_orange: String::from("#fe8019"),
            void: String::from("#1d2021"),
            stone0: String::from("#282828"),
            stone1: String::from("#32302f"),
            stone2: String::from("#3c3836"),
            stone3: String::from("#504945"),
            stone4: String::from("#665c54"),
            stone5: String::from("#7c6f64"),
            stone6: String::from("#928374"),
        };

        let ui_components = enum_map! {
            UiComponent::Title => true,
            UiComponent::Map => true,
            UiComponent::Credits => false,
            UiComponent::Debug => true,
        };

        let title_style = FontStyle::new(72.0, Color::from_hex(&color_scheme.fg));
        let title = Asset::new(Font::load(FONT_MONONOKI).and_then(move |font| {
            font.render("Janus Mining Colony", &title_style)
        }));

        let mononoki_font_info_style = FontStyle::new(20.0, Color::from_hex(&color_scheme.fg));
        let square_font_info_style = FontStyle::new(12.0, Color::from_hex(&color_scheme.fg));
        let zodiac_square_font_info_style = FontStyle::new(12.0, Color::from_hex(&color_scheme.fg));
        let font_info = vec! {
            Asset::new(Font::load(FONT_MONONOKI).and_then(move |font| {
                font.render(
                    "Mononoki font by Matthias Tellen, terms: SIL Open Font License 1.1",
                    &mononoki_font_info_style,
                    )
            })),
            Asset::new(Font::load(FONT_SQUARE).and_then(move |font| {
                font.render(
                    "Square font by Wouter Van Oortmerssen, terms: CC BY 3.0",
                    &square_font_info_style,
                    )
            })),
            Asset::new(Font::load(FONT_ZODIAC_SQUARE).and_then(move |font| {
                font.render(
                    "Zodiac Square font by Elementalist, terms: CC0",
                    &zodiac_square_font_info_style,
                    )
            })),
        };

        let camera_width = 60;
        let camera_height = 30;
        let camera_size = Vector::new(camera_width, camera_height);

        let map = game_map::GameMap::new();
        
        let initial_pos_x = (map.max_chuncks_x * map.chunk_size) / 2;
        let initial_pos_y = (map.max_chuncks_y * map.chunk_size) / 2;
        let initial_pos_z = 32;
        
        let camera_pos = Position {
            x: initial_pos_x as u32, 
            y: initial_pos_y as u32, 
            z: initial_pos_z as u32,
        };

        let camera = Camera {
            position: camera_pos,
            viewport_size: camera_size,
            max_size_x: map.max_chuncks_x * map.chunk_size - camera_size.x as u32,
            max_size_y: map.max_chuncks_y * map.chunk_size - camera_size.y as u32,
            max_size_z: map.max_chuncks_z * map.chunk_size,
        };

        let mut entities = generate_entities(
            initial_pos_x, initial_pos_y, initial_pos_z);
        let player_id = entities.len();
        entities.push(Entity {
            pos: Vector::new(initial_pos_x + 33, initial_pos_y + 13),
            depth: initial_pos_z,
            glyph: '@',
            color: color_scheme::ColorName::Blue,
            hp: 3,
            max_hp: 5,
        });

        let tile_size_px = Vector::new(18, 18);
        let glyph_map = vec! {
            (String::from(FONT_SQUARE), 
             String::from("#@g.%")),

            (String::from(FONT_ZODIAC_SQUARE), 
             String::from("░▒▓∷•‧≈╠╬╣╔╗╚╝╦╩═║")),
        };
        let tileset = Tileset::new(glyph_map, tile_size_px);
        //let game_glyphs_extended = "░▒▓∷•‧≈╠╬╣╔╗╚╝╦╩";
        //let tileset_extended = Asset::new(Font::load(FONT_ZODIAC_SQUARE).and_then(move |text| {
        //    let tiles = text
        //        .render(game_glyphs_extended, &FontStyle::new(tile_size_px.y, Color::WHITE))
        //        .expect("Could not render the font tileset.");
        //    let mut tileset = HashMap::new();
        //    for (index, glyph) in game_glyphs_extended.chars().enumerate() {
        //        let pos = (index as u32 * tile_size_px.x as u32, 0);
        //        let tile = tiles.subimage(Rectangle::new(pos, tile_size_px));
        //        tileset.insert(glyph, tile);
        //    }
        //    Ok(tileset)
        //}));

        let input_timer = Instant::now();

        Ok(Self {
            title,
            font_info,
            map,
            entities,
            player_id,
            tileset,
            tile_size_px,
            color_scheme,
            camera,
            ui_components,
            input_timer,
        })
    }

    /// Process keyboard and mouse, update the game state
    fn update(&mut self, window: &mut Window) -> Result<()> {
        use ButtonState::*;

        if self.input_timer.elapsed() >= Duration::from_millis(100) {
            // camera controls
            let camera = &mut self.camera;

            if window.keyboard()[Key::Left].is_down() {
                self.input_timer = Instant::now();
                camera.move_left();
            }
            if window.keyboard()[Key::Right].is_down() {
                self.input_timer = Instant::now();
                camera.move_right();
            }
            if window.keyboard()[Key::Up].is_down() {
                self.input_timer = Instant::now();
                camera.move_up();
            }
            if window.keyboard()[Key::Down].is_down() {
                self.input_timer = Instant::now();
                camera.move_down();
            }
            if window.keyboard()[Key::RBracket].is_down() {
                self.input_timer = Instant::now();
                camera.elevate();
            }
            if window.keyboard()[Key::LBracket].is_down() {
                self.input_timer = Instant::now();
                camera.lower();
            }
            // ctrl + section
            if window.keyboard()[Key::LControl].is_down() || 
               window.keyboard()[Key::RControl].is_down() {
                // ctrl + left
                if window.keyboard()[Key::Left].is_down() { 
                    self.input_timer = Instant::now();
                    camera.go_to(0, 
                                 camera.position.y, 
                                 camera.position.z);
                }
                // ctrl + right
                if window.keyboard()[Key::Right].is_down() { 
                    self.input_timer = Instant::now();
                    camera.go_to(camera.max_size_x, 
                                 camera.position.y, 
                                 camera.position.z);
                }
                // ctrl + up
                if window.keyboard()[Key::Up].is_down() { 
                    self.input_timer = Instant::now();
                    camera.go_to(camera.position.x, 
                                 0, 
                                 camera.position.z);
                }
                // ctrl + down
                if window.keyboard()[Key::Down].is_down() { 
                    self.input_timer = Instant::now();
                    camera.go_to(camera.position.x, 
                                 camera.max_size_y, 
                                 camera.position.z);
                }
                // ctrl + rbracket
                if window.keyboard()[Key::RBracket].is_down() { 
                    self.input_timer = Instant::now();
                    camera.go_to(camera.position.x, 
                                 camera.position.y, 
                                 0);
                }
                // ctrl + lbracket
                if window.keyboard()[Key::LBracket].is_down() {
                    self.input_timer = Instant::now();
                    camera.go_to(
                        camera.position.x, 
                        camera.position.y, 
                        camera.max_size_z);
                }
            }
        
            // player controls
            let player = &mut self.entities[self.player_id];
            if window.keyboard()[Key::A].is_down() {
                self.input_timer = Instant::now();
                player.pos.x -= 1.0;
            }
            if window.keyboard()[Key::D].is_down() {
                self.input_timer = Instant::now();
                player.pos.x += 1.0;
            }
            if window.keyboard()[Key::W].is_down() {
                self.input_timer = Instant::now();
                player.pos.y -= 1.0;
            }
            if window.keyboard()[Key::S].is_down() {
                self.input_timer = Instant::now();
                player.pos.y += 1.0;
            } 
       
        }

        if window.keyboard()[Key::Escape].is_down() {
            window.close();
        }
        // ui controls
        let ui_components = &mut self.ui_components;
        if window.keyboard()[Key::C] == Pressed {
            ui_components[UiComponent::Credits] = !ui_components[UiComponent::Credits];
        }

        if window.keyboard()[Key::M] == Pressed {
            ui_components[UiComponent::Map] = !ui_components[UiComponent::Map];
        }

        if window.keyboard()[Key::T] == Pressed {
            ui_components[UiComponent::Title] = !ui_components[UiComponent::Title];
        }

        if window.keyboard()[Key::B] == Pressed {
            ui_components[UiComponent::Debug] = !ui_components[UiComponent::Debug];
        }

        Ok(())
    }

    /// Draw stuff on the screen
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::from_hex(&self.color_scheme.void))?;

        if self.ui_components[UiComponent::Title] {
            self.draw_title(window).unwrap();
        }

        if self.ui_components[UiComponent::Map] {
            self.draw_map(window).unwrap();
        }

        if self.ui_components[UiComponent::Credits] {
            self.draw_credits(window).unwrap();
        }

        if self.ui_components[UiComponent::Debug] {
            self.draw_debug(window).unwrap();
        }
        
        Ok(())
    }

}

impl Game {

    // fn draw_something(&mut self, window: &mut Window) -> Result<()> {
    //     Ok(())
    // }

    fn draw_title(&mut self, window: &mut Window) -> Result<()> {
        self.title.execute(|image| {
            window.draw(
                &image
                    .area()
                    .with_center((window.screen_size().x as u32 / 2, 40)),
                Img(&image),
            );
            Ok(())
        })?;
        Ok(())
    }

    fn draw_map(&mut self, window: &mut Window) -> Result<()> {
        let tile_size_px = self.tile_size_px;

        let (tileset, map, entities) = (
            &mut self.tileset.tile_map, 
            &mut self.map, 
            &self.entities
        );
        
        let (camera_x, camera_y, camera_z) = (
            self.camera.position.x, 
            self.camera.position.y, 
            self.camera.position.z
        );

        let camera_size_x = self.camera.viewport_size.x;
        let camera_size_y = self.camera.viewport_size.y;
        
        let color_scheme = &self.color_scheme;

        let offset_px = Vector::new(50, 100);
        
        let origin_offset = Vector::new(-(camera_x as i32), -(camera_y as i32));
        //println!("camera_pos: {:?}", self.camera_pos);

        for x in camera_x..camera_x + camera_size_x as u32 {
            for y in camera_y..camera_y + camera_size_y as u32 {
                //println!("camera_z: {:?}", camera_z);
                let tile = map.get_tile(x, y, camera_z);
                    if let Some(image) = tileset.get(&tile.glyph) {
                        let pos_px = tile.pos
                            .translate(origin_offset)
                            .times(tile_size_px);
                        //println!("x: {:?}, y: {:?}, z: {:?}", x, y, camera_z);
                        //println!("{:?}", tile);
                        let tile_color = Color::from_hex(color_scheme.get_color_code(&tile.color));
                        window.draw(
                            &Rectangle::new(offset_px + pos_px, image.area().size()),
                            Blended(&image, tile_color),
                        );
                    }
                }
        }

        for entity in entities.iter() {
            if entity.depth == camera_z 
               && (entity.pos.x as u32) >= camera_x 
               && (entity.pos.x as u32) < (camera_x + camera_size_x as u32)
               && (entity.pos.y as u32) >= camera_y
               && (entity.pos.y as u32) < (camera_y + camera_size_y as u32) 
            {
                if let Some(image) = tileset.get(&entity.glyph) {
                    let pos_px = entity.pos
                        .translate(origin_offset)
                        .times(tile_size_px);
                    window.draw(
                        &Rectangle::new(offset_px + pos_px, image.area().size()),
                        Blended(&image, Color::from_hex(color_scheme.get_color_code(&entity.color))),
                    );
                }
            }
        }

        Ok(())
    }

    fn draw_credits(&mut self, window: &mut Window) -> Result<()> {
        let mut y_offset = 60;
        for fi in self.font_info.iter_mut() {
            fi.execute(|image| {
                window.draw(
                    &image
                        .area()
                        .translate((2, window.screen_size().y as i32 - y_offset)),
                    Img(&image),
                );
                Ok(())
            })?;
            y_offset -= 20;
        }

        Ok(())
    }

    fn draw_debug(&mut self, window: &mut Window) -> Result<()> {
        let mononoki_font_info_style = FontStyle::new(20.0, Color::from_hex(&self.color_scheme.fg));
        let debug_string = format!("Player Pos: (x: {:?} y: {:?})\n
Camera Pos: (x: {:?} y: {:?} z: {:?})",
                                   self.entities[self.player_id].pos.x,
                                   self.entities[self.player_id].pos.y,
                                   self.camera.position.x,
                                   self.camera.position.y,
                                   self.camera.position.z,
                                  );
        let mut debug_info = Asset::new(Font::load(FONT_MONONOKI).and_then(move |font| {
            font.render(
                debug_string.as_str(),
                &mononoki_font_info_style,
                )
        }));

        debug_info.execute(|image| {
            window.draw(
                &image
                    .area()
                    .translate((2, window.screen_size().y as i32 - 60)),
                Img(&image),
            );
            Ok(())
        })?;

        Ok(())
    }

}

fn main() {
    std::env::set_var("WINIT_HIDPI_FACTOR", "1.0");
    let settings = Settings {
         scale: quicksilver::graphics::ImageScaleStrategy::Blur,
        ..Default::default()
    };
    run::<Game>("Janus Mining Colony", Vector::new(1280, 720), settings);
}

#[derive(Clone, Debug, PartialEq)]
struct Entity {
    pos: Vector,
    depth: u32,
    glyph: char,
    color: color_scheme::ColorName,
    hp: i32,
    max_hp: i32,
}

fn generate_entities(
    initial_pos_x: u32, initial_pos_y: u32, initial_pos_z: u32) 
    -> Vec<Entity> {
    vec![
        Entity {
            pos: Vector::new(initial_pos_x + 31, initial_pos_y + 12),
            depth: initial_pos_z,
            glyph: '@',
            color: color_scheme::ColorName::Red,
            hp: 1,
            max_hp: 1,
        },
        Entity {
            pos: Vector::new(initial_pos_x + 31, initial_pos_y + 13),
            depth: initial_pos_z,
            glyph: '@',
            color: color_scheme::ColorName::Green,
            hp: 1,
            max_hp: 1,
        },
        Entity {
            pos: Vector::new(initial_pos_x + 31, initial_pos_y + 14),
            depth: initial_pos_z,
            glyph: '@',
            color: color_scheme::ColorName::Orange,
            hp: 0,
            max_hp: 0,
        },
        Entity {
            pos: Vector::new(initial_pos_x + 32, initial_pos_y + 12),
            depth: initial_pos_z,
            glyph: '@',
            color: color_scheme::ColorName::Purple,
            hp: 0,
            max_hp: 0,
        },
        Entity {
            pos: Vector::new(initial_pos_x + 32, initial_pos_y + 13),
            depth: initial_pos_z,
            glyph: '@',
            color: color_scheme::ColorName::Yellow,
            hp: 0,
            max_hp: 0,
        },
        Entity {
            pos: Vector::new(initial_pos_x + 32, initial_pos_y + 14),
            depth: initial_pos_z,
            glyph: '@',
            color: color_scheme::ColorName::Aqua,
            hp: 0,
            max_hp: 0,
        },
        Entity {
            pos: Vector::new(initial_pos_x + 33, initial_pos_y + 12),
            depth: initial_pos_z,
            glyph: '@',
            color: color_scheme::ColorName::Gray,
            hp: 0,
            max_hp: 0,
        },
        // ░▒▓∷•‧≈╠╬╣╔╗╚╝╦╩═║
        Entity {
            pos: Vector::new(initial_pos_x + 34, initial_pos_y + 19),
            depth: initial_pos_z,
            glyph: '║',
            color: color_scheme::ColorName::Yellow,
            hp: 0,
            max_hp: 0,
        },
        Entity {
            pos: Vector::new(initial_pos_x + 34, initial_pos_y + 18),
            depth: initial_pos_z,
            glyph: '║',
            color: color_scheme::ColorName::Yellow,
            hp: 0,
            max_hp: 0,
        },
        Entity {
            pos: Vector::new(initial_pos_x + 34, initial_pos_y + 17),
            depth: initial_pos_z,
            glyph: '╔',
            color: color_scheme::ColorName::Yellow,
            hp: 0,
            max_hp: 0,
        },
        Entity {
            pos: Vector::new(initial_pos_x + 35, initial_pos_y + 17),
            depth: initial_pos_z,
            glyph: '═',
            color: color_scheme::ColorName::Yellow,
            hp: 0,
            max_hp: 0,
        },
    ]
}



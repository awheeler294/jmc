use enum_map::{enum_map, Enum, EnumMap};
use quicksilver::prelude::*;
use quicksilver::graphics::View;

use std::collections::HashMap;
use std::time::{Duration, Instant};

mod game_map;
mod color_scheme;
mod camera;

use game_map::GameMap;
use color_scheme::{ColorScheme, ColorName};
use camera::Camera;

const FONT_MONONOKI: &'static str = "mononoki-Regular.ttf";
const FONT_SQUARE: &'static str = "square.ttf";
const FONT_ZODIAC_SQUARE: &'static str = "zodiac-square.ttf";

#[derive(Clone, Copy, Debug, PartialEq)]
struct Position<T> {
    x: T,
    y: T,
    z: T,
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
                let mut _tile_map = HashMap::new();
                for (index, glyph) in glyphs.chars().enumerate() {
                    let pos = (index as u32 * tile_size_px.x as u32, 0);
                    let tile = tiles.subimage(Rectangle::new(pos, tile_size_px));
                    _tile_map.insert(glyph, tile);
                }
                Ok(_tile_map)
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
    map: GameMap,
    entities: Vec<Entity>,
    player_id: usize,
    tileset: Tileset,
    tile_size_px: Vector,
    color_scheme: ColorScheme,
    camera: Camera,
    ui_components: EnumMap<UiComponent, bool>,
    input_timer: Instant,
}

impl State for Game {
    /// Load the assets and initialize the game
    fn new() -> Result<Self> {
        let color_scheme = ColorScheme {
            bg:           String::from("#282828"),
            fg:           String::from("#ebdbb2"),
            fg0:          String::from("#fbf1c7"),
            fg1:          String::from("#ebdbb2"),
            fg2:          String::from("#d5c4a1"),
            fg3:          String::from("#bdae93"),
            fg4:          String::from("#a89984"),
            gray:         String::from("#a89984"),
            light_gray:   String::from("#928374"),
            red:          String::from("#cc241d"),
            light_red:    String::from("#fb4934"),
            green:        String::from("#98971a"),
            light_green:  String::from("#b8bb26"),
            yellow:       String::from("#d79921"),
            light_yellow: String::from("#fabd2f"),
            blue:         String::from("#458588"),
            light_blue:   String::from("#83a598"),
            purple:       String::from("#b16286"),
            light_purple: String::from("#d3869b"),
            aqua:         String::from("#689d6a"),
            light_aqua:   String::from("#8ec07c"),
            orange:       String::from("#d65d0e"),
            light_orange: String::from("#fe8019"),
            void:         String::from("#1d2021"),
            stone0:       String::from("#282828"),
            stone1:       String::from("#32302f"),
            stone2:       String::from("#3c3836"),
            stone3:       String::from("#504945"),
            stone4:       String::from("#665c54"),
            stone5:       String::from("#7c6f64"),
            stone6:       String::from("#928374"),
        };

        let ui_components = enum_map! {
            UiComponent::Title => true,
            UiComponent::Map => true,
            UiComponent::Credits => false,
            UiComponent::Debug => true,
        };

        let title_style = FontStyle::new(72.0, Color::from_hex(&color_scheme.fg));
        let title = Asset::new(Font::load(FONT_MONONOKI).and_then(move |font| {
            font.render("Janus 7 Mining Colony", &title_style)
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

        let map = GameMap::new();

        let camera_width = 60;
        let camera_height = 30;
       
        let initial_pos_x = (map.max_chuncks_x * map.chunk_size) / 2;
        let initial_pos_y = (map.max_chuncks_y * map.chunk_size) / 2;
        let initial_pos_z = 32;

        let camera = Camera::new(
            initial_pos_x, 
            initial_pos_y, 
            initial_pos_z, 
            map.max_chuncks_x * map.chunk_size - camera_width, 
            map.max_chuncks_y * map.chunk_size - camera_height,
            map.max_chuncks_z * map.chunk_size, 
            (camera_width, camera_height),
        ); 
        
        let mut entities = generate_entities(
            initial_pos_x, initial_pos_y, initial_pos_z);
        let player_id = entities.len();
        entities.push(Entity {
            pos: Vector::new(initial_pos_x + 29, initial_pos_y + 20),
            depth: initial_pos_z,
            glyph: '0',
            color: ColorName::LightOrange,
            hp: 3,
            max_hp: 5,
        });

        let tile_size_px = Vector::new(18, 18);
        let glyph_map = vec! {
            (String::from(FONT_SQUARE), 
             String::from("#@g.%08*")),

            (String::from(FONT_ZODIAC_SQUARE), 
             String::from("™↺∆░▒▓∷•‧≈╠╬╣╔╗╚╝╦╩═║")),
        };
        let tileset = Tileset::new(glyph_map, tile_size_px);
        
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
            if window.keyboard()[Key::Period].is_down() {
                self.input_timer = Instant::now();
                camera.lower();
            }
            if window.keyboard()[Key::Comma].is_down() {
                self.input_timer = Instant::now();
                camera.elevate();
            }
            if window.keyboard()[Key::RBracket].is_down() {
                self.input_timer = Instant::now();
                camera.zoom_in();
            }
            if window.keyboard()[Key::LBracket].is_down() {
                self.input_timer = Instant::now();
                camera.zoom_out();
            }
            // ctrl + section
            if window.keyboard()[Key::LControl].is_down() || 
               window.keyboard()[Key::RControl].is_down() {
                // ctrl + left
                if window.keyboard()[Key::Left].is_down() { 
                    self.input_timer = Instant::now();
                    camera.go_to(0.0, 
                                 camera.viewport.y(), 
                                 camera.z_position);
                }
                // ctrl + right
                if window.keyboard()[Key::Right].is_down() { 
                    self.input_timer = Instant::now();
                    camera.go_to(camera.max_x as f32, 
                                 camera.viewport.y(), 
                                 camera.z_position);
                }
                // ctrl + up
                if window.keyboard()[Key::Up].is_down() { 
                    self.input_timer = Instant::now();
                    camera.go_to(camera.viewport.x(), 
                                 0.0, 
                                 camera.z_position);
                }
                // ctrl + down
                if window.keyboard()[Key::Down].is_down() { 
                    self.input_timer = Instant::now();
                    camera.go_to(camera.viewport.x(), 
                                 camera.viewport.y(), 
                                 camera.z_position);
                }
                // ctrl + rbracket
                if window.keyboard()[Key::RBracket].is_down() { 
                    self.input_timer = Instant::now();
                    camera.go_to(camera.viewport.x(), 
                                 camera.viewport.y(), 
                                 camera.max_z);
                }
                // ctrl + lbracket
                if window.keyboard()[Key::LBracket].is_down() {
                    self.input_timer = Instant::now();
                    camera.go_to(
                        camera.viewport.x(), 
                        camera.viewport.y(), 
                        0);
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
        let window_view = View::new(
                Rectangle::new(Vector::new(0.0, 0.0), window.screen_size()
        ));
        window.set_view(window_view);
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

        let (tileset, map, entities) = (
            &mut self.tileset.tile_map, 
            &mut self.map, 
            &self.entities
        );
        
        let camera = &self.camera;
        
        let tile_size_px = self.tile_size_px * camera.zoom_factor;

        let (camera_x, camera_y, camera_z) = (
            (camera.viewport.x()) as u32, 
            (camera.viewport.y()) as u32, 
            camera.z_position,
        );       
        let camera_size_x = camera.viewport.width();
        let camera_size_y = camera.viewport.height();
        
        let color_scheme = &self.color_scheme;

        let offset_px = Vector::new(50, 100);
        
        let origin_offset = Vector::new(-(camera_x as i32), - (camera_y as i32));
        //println!("camera_pos: {:?}", self.camera_pos);

        for x in camera_x..camera_x + camera_size_x as u32 {
            for y in camera_y..camera_y + camera_size_y as u32 {
                //println!("camera_z: {:?}", camera_z);
                let tile = map.get_tile(x, y, camera_z);
                let pos_px = tile.pos
                    .translate(origin_offset)
                    .times(tile_size_px);
                //println!("x: {:?}, y: {:?}, z: {:?}", x, y, camera_z);
                //println!("{:?}", tile);
                let tile_color = Color::from_hex(
                    color_scheme.get_color_code(&tile.color));
                if camera.zoom_factor > 0.5 {
                    if let Some(image) = tileset.get(&tile.glyph) {
                        window.draw_ex(
                            &Rectangle::new(
                                offset_px + pos_px, image.area().size()
                            ),
                            Blended(&image, tile_color),
                            Transform::scale(
                                (camera.zoom_factor, camera.zoom_factor)
                            ),
                            0 // Z value
                        );
                    }
                }
                else {
                    window.draw_ex(
                            &Rectangle::new(
                                offset_px + pos_px, self.tile_size_px
                            ),
                            tile_color,
                            Transform::scale(
                                (camera.zoom_factor, camera.zoom_factor)),
                            0 // Z value
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
                    let entity_color = Color::from_hex(
                        color_scheme.get_color_code(&entity.color));
                    window.draw_ex(
                        &Rectangle::new(
                            offset_px + pos_px, image.area().size()),
                        Blended(&image, entity_color),
                        Transform::scale(
                                (camera.zoom_factor, camera.zoom_factor)),
                        1 // Z value

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
        let mononoki_font_info_style = FontStyle::new(
            20.0, Color::from_hex(&self.color_scheme.fg));

        let player_id = self.player_id;
        let player = &self.entities[player_id];
        let tile = self.map.get_tile(
            player.pos.x as u32, player.pos.y as u32, self.camera.z_position);

        let debug_string = format!("Player Pos: (x: {:?} y: {:?})  Tile: (Color: {:?} glyph: {:?} val: {:?})\n
Camera Pos: (x: {:?} y: {:?} z: {:?}), Zoom Factor: {:?}, viewport size: {:?}",
                                   player.pos.x,
                                   player.pos.y,
                                   tile.color,
                                   tile.glyph,
                                   tile.val,
                                   self.camera.viewport.x(),
                                   self.camera.viewport.y(),
                                   self.camera.z_position,
                                   self.camera.zoom_factor,
                                   self.camera.viewport.size(),
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
    run::<Game>("Janus 7 Mining Colony", Vector::new(1280, 720), settings);
}

#[derive(Clone, Debug, PartialEq)]
struct Entity {
    pos: Vector,
    depth: u32,
    glyph: char,
    color: ColorName,
    hp: i32,
    max_hp: i32,
}

fn generate_entities(
    initial_pos_x: u32, initial_pos_y: u32, initial_pos_z: u32) 
    -> Vec<Entity> {
    vec![
        Entity {
            pos: Vector::new(initial_pos_x + 27, initial_pos_y + 18),
            depth: initial_pos_z,
            glyph: '@',
            color: ColorName::Red,
            hp: 1,
            max_hp: 1,
        },
        Entity {
            pos: Vector::new(initial_pos_x + 27, initial_pos_y + 19),
            depth: initial_pos_z,
            glyph: '@',
            color: ColorName::Green,
            hp: 1,
            max_hp: 1,
        },
        Entity {
            pos: Vector::new(initial_pos_x + 27, initial_pos_y + 20),
            depth: initial_pos_z,
            glyph: '@',
            color: ColorName::Orange,
            hp: 0,
            max_hp: 0,
        },
        Entity {
            pos: Vector::new(initial_pos_x + 28, initial_pos_y + 18),
            depth: initial_pos_z,
            glyph: '@',
            color: ColorName::Purple,
            hp: 0,
            max_hp: 0,
        },
        Entity {
            pos: Vector::new(initial_pos_x + 28, initial_pos_y + 19),
            depth: initial_pos_z,
            glyph: '@',
            color: ColorName::Yellow,
            hp: 0,
            max_hp: 0,
        },
        Entity {
            pos: Vector::new(initial_pos_x + 28, initial_pos_y + 20),
            depth: initial_pos_z,
            glyph: '@',
            color: ColorName::Aqua,
            hp: 0,
            max_hp: 0,
        },
        Entity {
            pos: Vector::new(initial_pos_x + 29, initial_pos_y + 18),
            depth: initial_pos_z,
            glyph: '@',
            color: ColorName::Gray,
            hp: 0,
            max_hp: 0,
        },
        // ░▒▓∷•‧≈╠╬╣╔╗╚╝╦╩═║
        Entity {
            pos: Vector::new(initial_pos_x + 25, initial_pos_y + 19),
            depth: initial_pos_z,
            glyph: '║',
            color: ColorName::Yellow,
            hp: 0,
            max_hp: 0,
        },
        Entity {
            pos: Vector::new(initial_pos_x + 25, initial_pos_y + 18),
            depth: initial_pos_z,
            glyph: '║',
            color: ColorName::Yellow,
            hp: 0,
            max_hp: 0,
        },
        Entity {
            pos: Vector::new(initial_pos_x + 25, initial_pos_y + 17),
            depth: initial_pos_z,
            glyph: '╔',
            color: ColorName::Yellow,
            hp: 0,
            max_hp: 0,
        },
        Entity {
            pos: Vector::new(initial_pos_x + 26, initial_pos_y + 17),
            depth: initial_pos_z,
            glyph: '═',
            color: ColorName::Yellow,
            hp: 0,
            max_hp: 0,
        },
        Entity {
            pos: Vector::new(initial_pos_x + 29, initial_pos_y + 19),
            depth: initial_pos_z,
            glyph: '@',
            color: ColorName::Blue,
            hp: 3,
            max_hp: 5,
        }
    ]
}



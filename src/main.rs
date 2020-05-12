mod atoms;
use atoms::Atoms;

extern crate sfml;

static TILE_SIZE: f32 = 50.0;
static BOARD_SIZE: usize = 10;

use sfml::system::{Clock,Vector2f};
use sfml::window::{ContextSettings, VideoMode, Key, Style, Event, mouse::Button};
use sfml::graphics::{Sprite,Font,Texture,RenderStates, RenderWindow, RenderTarget, CircleShape, Color, Transformable, Shape};

trait Element {
    fn set_position(&mut self, position: &Vector2f);
    fn draw<RT: RenderTarget>(&self, target: &mut RT, rs: &mut RenderStates);
    fn restart();
    fn is_animated() -> bool;
}


fn main() {
    let mut atoms = Atoms::new( BOARD_SIZE, BOARD_SIZE);

    let font = match Font::from_file("Instruction.ttf") {
        Some(font) => font,
        None => panic!("Cannot load font.")
    };

    let stoneTexture = match Texture::from_file("stone.png") {
        Some(stoneTexture) => stoneTexture,
        None => panic!("Texture error.")
    };

    let stoneSize = stoneTexture.size();

    let mut stoneSprite = Sprite::with_texture( &stoneTexture );

    stoneSprite.set_scale( Vector2f::new (TILE_SIZE / stoneSize.x as f32, TILE_SIZE / stoneSize.y as f32 ) );

    let woodTexture = match Texture::from_file("wood.png") {
        Some(woodTexture) => woodTexture,
        None => panic!("Texture error.")
    };

    let woodSize = woodTexture.size();

    let mut woodSprite = Sprite::with_texture( &woodTexture );

    woodSprite.set_scale(  Vector2f::new (TILE_SIZE / woodSize.x as f32, TILE_SIZE / woodSize.y as f32 ));

    let pColor = [ Color::RED, Color::GREEN, Color::BLUE, Color::YELLOW ];
    let sColor = Color::WHITE;

    // Create the window of the application
    let mut window = RenderWindow::new(VideoMode::new( (10+BOARD_SIZE as u32) * TILE_SIZE as u32,
                                                        BOARD_SIZE as u32 * TILE_SIZE as u32,
                                                        32),
                                       "Atoms",
                                       Style::CLOSE,
                                       &ContextSettings::default());

    window.set_framerate_limit( 60 );

    let mut clock = Clock::start();

    // Create a CircleShape
    let mut circle = CircleShape::new(30., 20);
    circle.set_fill_color(&Color::RED);
    circle.set_position(Vector2f::new(100., 100.));

    while window.is_open() {
        if !atoms.finished {
            let elapsed = clock.elapsed_time();
            if elapsed.as_seconds() > 0.25 {
                atoms.recalculate_board();
                clock.restart();
                //drawables[ Atoms::Bang ]->restart();
            }
        }

        // Handle events
        let event = match window.poll_event() {
            Some(event) => {

                match event {
                    Event::Closed => window.close(),
                    Event::KeyPressed { code, .. } => { match code {
                        Key::Escape => { window.close() },
                        _ => { } } },
                    _ => { }
                }

                if atoms.finished {
                    match event {
                        Event::MouseButtonPressed { button, x, y } => {
                            match button {
                                Button::Left => { atoms.click( (x / TILE_SIZE as i32) as usize,
                                                                (y / TILE_SIZE as i32) as usize );
                                                  clock.restart();
                                                  //drawables[ Atoms::Bang ]->restart()
                                },
                                _ => {} }
                        },
                        Event::KeyPressed { code, .. } => { match code {
                            Key::R => { atoms.clear( true ) },
                            Key::C => { atoms.clear( false ) },
                            Key::Space => {  if !atoms.editing {
                                atoms.clear( true );
                            }
                                             atoms.editing = !atoms.editing;  },
                            _ => { } } },
                        _ => {}
                    }
                }
            },
            None => {}
        };


        // Clear the window
        window.clear(&Color::rgb(0, 200, 200));

        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                let content = atoms.get_content( x, y );
                // auto &cell = *drawables[ content ];
                // cell.setPosition( y*TILE_SIZE, x*TILE_SIZE );
                // window.draw( cell );
            }
        }

        for x in 0..BOARD_SIZE {
            for y in BOARD_SIZE..BOARD_SIZE+10 {
                //auto &cell = *drawables[ Atoms::Wall ];
                //cell.setPosition( y*TILE_SIZE, x*TILE_SIZE );
                //window.draw( cell );
            }
        }

        // Draw the shape
        window.draw(&circle);
        // Display things on screen
        window.display()
    }
}

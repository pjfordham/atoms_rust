mod atoms;
use atoms::Atoms;
use std::convert::TryInto;

extern crate sfml;

static TILE_SIZE: f32 = 50.0;
static BOARD_SIZE: usize = 10;

use sfml::system::{Clock,Vector2f};
use sfml::window::{ContextSettings, VideoMode, Key, Style, Event, mouse::Button};
use sfml::graphics::{Shape,RectangleShape,Drawable, Sprite,Font,Text,Texture,RenderStates, RenderWindow, RenderTarget, Color, Transformable};

trait _Element {
    fn set_position(&mut self, position: &Vector2f);
    fn restart(&mut self);
    fn is_animated(&mut self) -> bool;
}

struct Number<'a> {
    font : &'a Font,
    color : Color,
    background : &'a Sprite<'a>,
    number : u32,
    position: Vector2f
}

impl<'a> Number<'a> {
    fn new( _font : &'a Font, _color : Color, _background : &'a Sprite<'a>, _number : u32) -> Number<'a> {
        let number = Number{ font : _font, color : _color, background : _background, number : _number, position : Vector2f::new(0.,0.)  };
         number
    }
}

impl _Element for Number<'_> {
    fn set_position(&mut self, position: &Vector2f) {
        self.position.x = position.x;
        self.position.y = position.y;
    }
    fn restart(&mut self) {}
    fn is_animated(&mut self) -> bool {false}
}

impl Drawable for Number<'_> {

    fn draw<'se, 'tex, 'sh, 'shte>(
        &'se self,
        target: &mut dyn RenderTarget,
        _states: RenderStates<'tex, 'sh, 'shte>
    )
    where
        'se: 'sh, {

        let mut sprite = self.background.clone();
        sprite.set_position( self.position );
        target.draw(&sprite);

        let mut text = Text::new( &self.number.to_string(), &self.font, TILE_SIZE as u32 );

        // center text
        let text_rect = text.local_bounds();
        text.set_position( self.position );
        text.set_origin( Vector2f::new (text_rect.left + text_rect.width/2.0,
                                        text_rect.top  + text_rect.height/2.0));
        text.move_( Vector2f::new (0.5*TILE_SIZE, 0.5*TILE_SIZE));

        text.set_fill_color( &self.color );

        target.draw(&text);
    }
}

struct SpriteElement<'a> {
    background : &'a Sprite<'a>,
    position: Vector2f
}

impl<'a> SpriteElement<'a> {
    fn new(_background : &'a Sprite<'a>) -> SpriteElement<'a> {
        let number = SpriteElement{ background : _background, position : Vector2f::new(0.,0.)  };
         number
    }
}

impl _Element for SpriteElement<'_> {
    fn set_position(&mut self, position: &Vector2f) {
        self.position.x = position.x;
        self.position.y = position.y;
    }
    fn restart(&mut self) {}
    fn is_animated(&mut self) -> bool {false}
}

impl Drawable for SpriteElement<'_> {

    fn draw<'se, 'tex, 'sh, 'shte>(
        &'se self,
        target: &mut dyn RenderTarget,
        _states: RenderStates<'tex, 'sh, 'shte>
    )
    where
        'se: 'sh, {

        let mut sprite = self.background.clone();
        sprite.set_position( self.position );
        target.draw(&sprite);
    }
}

struct RectangleShapeElement {
    color : Color,
    position: Vector2f
}

impl<'a> RectangleShapeElement {
    fn new( _color : Color) -> RectangleShapeElement {
        let number = RectangleShapeElement{ color : _color, position : Vector2f::new(0.,0.)  };
         number
    }
}

impl _Element for RectangleShapeElement {
    fn set_position(&mut self, position: &Vector2f) {
        self.position.x = position.x;
        self.position.y = position.y;
    }
    fn restart(&mut self) {}
    fn is_animated(&mut self) -> bool {false}
}

impl Drawable for RectangleShapeElement {

    fn draw<'se, 'tex, 'sh, 'shte>(
        &'se self,
        target: &mut dyn RenderTarget,
        _states: RenderStates<'tex, 'sh, 'shte>
    )
    where
        'se: 'sh, {

        let mut shape = RectangleShape::new();
        shape.set_size(Vector2f::new(TILE_SIZE, TILE_SIZE));
        shape.set_fill_color( &self.color );
        shape.set_position( self.position );
        target.draw(&shape);
    }
}

struct VolatileNumber<'a> {
    font : &'a Font,
    color : Color,
    background : &'a Sprite<'a>,
    number : u32,
    position: Vector2f,
    master_clock : Clock
}

impl<'a> VolatileNumber<'a> {
    fn new( _font : &'a Font, _color : Color, _background : &'a Sprite<'a>, _number : u32) -> VolatileNumber<'a> {
        let number = VolatileNumber{ font : _font, color : _color, background : _background, number : _number, position : Vector2f::new(0.,0.), master_clock : Clock::start()  };
         number
    }
    fn draw<'se, 'tex, 'sh, 'shte>(
        &'se self,
        target: &mut dyn RenderTarget,
        _states: RenderStates<'tex, 'sh, 'shte>,
        frame : usize
    )
    where
        'se: 'sh, {
        let mut sprite = self.background.clone();
        sprite.set_position( self.position );
        target.draw(&sprite);

        let mut text = Text::new( &self.number.to_string(), &self.font, TILE_SIZE as u32 );

        // center text
        let text_rect = text.local_bounds();
        text.set_position( self.position );
        text.set_origin( Vector2f::new (text_rect.left + text_rect.width/2.0,
                                        text_rect.top  + text_rect.height/2.0));
        text.move_( Vector2f::new (0.5*TILE_SIZE, 0.5*TILE_SIZE));

        if  frame >= 25 {
            let dimness = Color::rgba(255,255,255, ((50-frame) as u8) *9 );
            text.set_fill_color( &(self.color * dimness) );
        } else {
            let dimness = Color::rgba(255,255,255, (frame as u8) *9 );
            text.set_fill_color( &(self.color * dimness) );
        }
        target.draw(&text);
    }

}

impl _Element for VolatileNumber<'_> {
    fn set_position(&mut self, position: &Vector2f) {
        self.position.x = position.x;
        self.position.y = position.y;
    }
    fn restart(&mut self) {}
    fn is_animated(&mut self) -> bool {false}
}

impl Drawable for VolatileNumber<'_> {

    fn draw<'se, 'tex, 'sh, 'shte>(
        &'se self,
        target: &mut dyn RenderTarget,
        _states: RenderStates<'tex, 'sh, 'shte>
    )
    where
        'se: 'sh, {
        let mut frame = self.master_clock.elapsed_time().as_milliseconds() / (1000 / 50);
        frame = frame % 50;
        self.draw( target, _states, frame.try_into().unwrap()  );
    }
}

trait Element: _Element + Drawable {
      fn as_drawable_ref(&mut self) -> & dyn Drawable;
}
impl<T: _Element + Drawable> Element for T {
  fn as_drawable_ref(&mut self) -> & dyn Drawable {
        self
    }
}



fn main() {
    let mut atoms = Atoms::new( BOARD_SIZE, BOARD_SIZE);

    let font = match Font::from_file("Instruction.ttf") {
        Some(font) => font,
        None => panic!("Cannot load font.")
    };

    let stone_texture = match Texture::from_file("stone.png") {
        Some(stone_texture) => stone_texture,
        None => panic!("Texture error.")
    };

    let stone_size = stone_texture.size();

    let mut stone_sprite = Sprite::with_texture( &stone_texture );

    stone_sprite.set_scale( Vector2f::new (TILE_SIZE / stone_size.x as f32, TILE_SIZE / stone_size.y as f32 ) );

    let wood_texture = match Texture::from_file("wood.png") {
        Some(wood_texture) => wood_texture,
        None => panic!("Texture error.")
    };

    let wood_size = wood_texture.size();

    let mut wood_sprite = Sprite::with_texture( &wood_texture );

    wood_sprite.set_scale(  Vector2f::new (TILE_SIZE / wood_size.x as f32, TILE_SIZE / wood_size.y as f32 ));

    let p_color = [ Color::RED, Color::GREEN, Color::BLUE, Color::YELLOW ];
    let s_color = Color::WHITE;

    // Create the window of the application
    let mut window = RenderWindow::new(VideoMode::new( (10+BOARD_SIZE as u32) * TILE_SIZE as u32,
                                                        BOARD_SIZE as u32 * TILE_SIZE as u32,
                                                        32),
                                       "Atoms",
                                       Style::CLOSE,
                                       &ContextSettings::default());

    window.set_framerate_limit( 60 );

    let mut clock = Clock::start();

    let mut x00 = SpriteElement::new( &stone_sprite );
    let mut x01 = RectangleShapeElement::new( Color::RED );
    let mut x02 = RectangleShapeElement::new( Color::YELLOW );
    let mut x03 = SpriteElement::new( &wood_sprite );
    let mut x04 = SpriteElement::new( &wood_sprite ); // FIXME to explosion

    let mut x05 = Number::new( &font, p_color[0], &wood_sprite, 1 );
    let mut x06 = Number::new( &font, p_color[0], &wood_sprite, 2 );
    let mut x07 = Number::new( &font, p_color[0], &wood_sprite, 3 );
    let mut x08 = VolatileNumber::new( &font, p_color[0], &wood_sprite, 1 );
    let mut x09 = VolatileNumber::new( &font, p_color[0], &wood_sprite, 2 );
    let mut x10 = VolatileNumber::new( &font, p_color[0], &wood_sprite, 3 );

    let mut x11 = Number::new( &font, p_color[1], &wood_sprite, 1 );
    let mut x12 = Number::new( &font, p_color[1], &wood_sprite, 2 );
    let mut x13 = Number::new( &font, p_color[1], &wood_sprite, 3 );
    let mut x14 = VolatileNumber::new( &font, p_color[1], &wood_sprite, 1 );
    let mut x15 = VolatileNumber::new( &font, p_color[1], &wood_sprite, 2 );
    let mut x16 = VolatileNumber::new( &font, p_color[1], &wood_sprite, 3 );

    let mut x17 = Number::new( &font, p_color[2], &wood_sprite, 1 );
    let mut x18 = Number::new( &font, p_color[2], &wood_sprite, 2 );
    let mut x19 = Number::new( &font, p_color[2], &wood_sprite, 3 );
    let mut x20 = VolatileNumber::new( &font, p_color[2], &wood_sprite, 1 );
    let mut x21 = VolatileNumber::new( &font, p_color[2], &wood_sprite, 2 );
    let mut x22 = VolatileNumber::new( &font, p_color[2], &wood_sprite, 3 );

    let mut x23 = Number::new( &font, p_color[3], &wood_sprite, 1 );
    let mut x24 = Number::new( &font, p_color[3], &wood_sprite, 2 );
    let mut x25 = Number::new( &font, p_color[3], &wood_sprite, 3 );
    let mut x26 = VolatileNumber::new( &font, p_color[3], &wood_sprite, 1 );
    let mut x27 = VolatileNumber::new( &font, p_color[3], &wood_sprite, 2 );
    let mut x28 = VolatileNumber::new( &font, p_color[3], &wood_sprite, 3 );

    let mut x29 = Number::new( &font, s_color, &wood_sprite, 1 );
    let mut x30 = Number::new( &font, s_color, &wood_sprite, 2 );

    let drawables: [& mut dyn Element; 31] = [
        &mut x00,
        &mut x01,
        &mut x02,
        &mut x03,
        &mut x04,
        &mut x05,
        &mut x06,
        &mut x07,
        &mut x08,
        &mut x09,
        &mut x10,
        &mut x11,
        &mut x12,
        &mut x13,
        &mut x14,
        &mut x15,
        &mut x16,
        &mut x17,
        &mut x18,
        &mut x19,
        &mut x20,
        &mut x21,
        &mut x22,
        &mut x23,
        &mut x24,
        &mut x25,
        &mut x26,
        &mut x27,
        &mut x28,
        &mut x29,
        &mut x30
   ];

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
        match window.poll_event() {
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

        window.clear( &Color::BLACK );

        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                let pos = Vector2f::new(x as f32*TILE_SIZE as f32, y as f32 *TILE_SIZE as f32 );
                let content = atoms.get_content( x, y );
                drawables[ content as usize ].set_position( &pos );
                window.draw( drawables[ content as usize ].as_drawable_ref() );
            }
        }

        for x in BOARD_SIZE..BOARD_SIZE+10 {
            for y in 0..BOARD_SIZE {
                let pos = Vector2f::new(x as f32*TILE_SIZE as f32, y as f32 *TILE_SIZE as f32 );
                drawables[ atoms::Drawable::Wall as usize ].set_position(&pos);
                window.draw( drawables[ atoms::Drawable::Wall as usize ].as_drawable_ref() );
            }
        }

        let mut text = Text::new( "Score Board", &font, (TILE_SIZE-5.0) as u32 );
        text.set_position( Vector2f::new(BOARD_SIZE as f32*(TILE_SIZE as f32 +9.5) as f32, TILE_SIZE as f32));
        text.set_fill_color(&Color::WHITE);

        // center text
        let text_rect = text.local_bounds();
        text.set_origin( Vector2f::new(text_rect.left + text_rect.width/2.0,
                       text_rect.top  + text_rect.height/2.0));
        text.move_( Vector2f::new(0.5*TILE_SIZE, 0.5*TILE_SIZE));

        window.draw(&text);

        for i in 0..4 {
            let s = if atoms.is_player_dead( i ) {
                format!("Player {}:    DEAD", i+1)
            } else {
                if atoms.game_over() {
                    format!("Player {}: WINNER!", i+1)
                } else {
                    format!("Player {}:     {:03}", i+1, atoms.get_player_score(i))
                }
            };
            let mut text = Text::new( &s, &font, (TILE_SIZE-5.0) as u32 );
            text.set_position( Vector2f::new(BOARD_SIZE as f32 *TILE_SIZE+5.0, TILE_SIZE*(i as f32+3.0)-5.0));
            if i == atoms.get_current_player() {
                text.set_outline_thickness(5.0);
                text.set_fill_color(&p_color[i]);
                text.set_outline_color(&Color::WHITE);
            } else {
                text.set_fill_color(&p_color[i]);
            }
            window.draw(&text);
        }

        // Display things on screen
        window.display()
    }
    atoms.dump_state();
}

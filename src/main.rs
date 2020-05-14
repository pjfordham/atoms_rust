mod atoms;
use atoms::Atoms;
use std::convert::TryInto;

extern crate sfml;

static TILE_SIZE: f32 = 50.0;
static BOARD_SIZE: usize = 10;
static BOARD_SIZE_F: f32 = BOARD_SIZE as f32;

use sfml::system::{Clock,Vector2f};
use sfml::window::{ContextSettings, VideoMode, Key, Style, Event, mouse::Button};
use sfml::graphics::{BlendMode,Transform,IntRect,Shape,RectangleShape,Drawable, Sprite,Font,Text,Texture,RenderStates, RenderWindow, RenderTarget, Color, Transformable};

struct Number<'a> { // Nummber struct is valid for lifetime 'a where a is the intersection of
    // the lifetime of the referenced Font and the referenced Sprite background.
    // The sprite also has lifetime 'a since it contains a reference to a texture that is only
    // valid for 'a. So if the texture is valid, then the sprite is valid, and the reference to
    // the sprite is valid, and the font is valid, then number is valid.
    // The color, number and posiion are owned directly by the Number.
    font : &'a Font,
    color : Color,
    background : &'a Sprite<'a>,
    number : u32,
}

// new takes the lifetimes of the font, and sprite as per above and returns a number with the intersection of those lifetimes
impl<'a> Number<'a> {
    fn new( _font : &'a Font, _color : Color, _background : &'a Sprite<'a>, _number : u32) -> Number<'a> {
        let number = Number{ font : _font, color : _color, background : _background, number : _number };
         number
    }
}

impl Drawable for Number<'_> {

    // Not sure about this one, the lifttime of self, se, must be equal to the lifetime of 'sh the shader in
    // the renderstates, the texture and the shader texture assumidly have to be ok too?
    fn draw<'se, 'tex, 'sh, 'shte>(
        &'se self,
        target: &mut dyn RenderTarget,
        states: RenderStates<'tex, 'sh, 'shte>
    )
    where
        'se: 'sh, {

        let my_states = RenderStates::new( states.blend_mode, states.transform, states.texture, states.shader );

        target.draw_with_renderstates(self.background, my_states);

        let mut text = Text::new( &self.number.to_string(), &self.font, TILE_SIZE as u32 );

        // center text
        let text_rect = text.local_bounds();
        text.set_origin( Vector2f::new (text_rect.left + text_rect.width/2.0,
                                        text_rect.top  + text_rect.height/2.0));
        text.move_( Vector2f::new (0.5*TILE_SIZE, 0.5*TILE_SIZE));

        text.set_fill_color( self.color );

        target.draw_with_renderstates(&text, states);
    }
}

struct SpriteElement<'a> // Again we have a reference to a sprite that must be valid, and that sprite has a reference to
// a texture that must be valid.
{
    background : &'a Sprite<'a>,
}

impl<'a> SpriteElement<'a> {
    fn new(_background : &'a Sprite<'a>) -> SpriteElement<'a> {
        let number = SpriteElement{ background : _background };
         number
    }
}

impl Drawable for SpriteElement<'_> {

    fn draw<'se, 'tex, 'sh, 'shte>(
        &'se self,
        target: &mut dyn RenderTarget,
        states: RenderStates<'tex, 'sh, 'shte>
    )
    where
        'se: 'sh, {

        target.draw_with_renderstates(self.background, states);
    }
}

struct RectangleShapeElement {
    color : Color,
}

impl<'a> RectangleShapeElement {
    fn new( _color : Color) -> RectangleShapeElement {
        let number = RectangleShapeElement{ color : _color };
         number
    }
}

impl Drawable for RectangleShapeElement {

    fn draw<'se, 'tex, 'sh, 'shte>(
        &'se self,
        target: &mut dyn RenderTarget,
        states: RenderStates<'tex, 'sh, 'shte>
    )
    where
        'se: 'sh, {

        let mut shape = RectangleShape::new();
        shape.set_size(Vector2f::new(TILE_SIZE, TILE_SIZE));
        shape.set_fill_color( self.color );
        target.draw_with_renderstates(&shape, states);
    }
}

struct VolatileNumber<'a> {
    font : &'a Font,
    color : Color,
    background : &'a Sprite<'a>,
    number : u32,
    master_clock : &'a Clock
}

impl<'a> VolatileNumber<'a> {
    fn new( _font : &'a Font, _color : Color, _background : &'a Sprite<'a>, _number : u32, clock : &'a Clock ) -> VolatileNumber<'a> {
        let number = VolatileNumber{ font : _font, color : _color, background : _background, number : _number, master_clock : clock  };
        number
    }
    fn draw<'se, 'tex, 'sh, 'shte>(
        &'se self,
        target: &mut dyn RenderTarget,
        states: RenderStates<'tex, 'sh, 'shte>,
        frame : usize
    )
    where
        'se: 'sh, {
        let my_states = RenderStates::new( states.blend_mode, states.transform, states.texture, states.shader );
        target.draw_with_renderstates(self.background, my_states);

        let mut text = Text::new( &self.number.to_string(), &self.font, TILE_SIZE as u32 );

        // center text
        let text_rect = text.local_bounds();
        text.set_origin( Vector2f::new (text_rect.left + text_rect.width/2.0,
                                        text_rect.top  + text_rect.height/2.0));
        text.move_( Vector2f::new (0.5*TILE_SIZE, 0.5*TILE_SIZE));

        if  frame >= 25 {
            let dimness = Color::rgba(255,255,255, ((50-frame) as u8) *9 );
            text.set_fill_color( self.color * dimness );
        } else {
            let dimness = Color::rgba(255,255,255, (frame as u8) *9 );
            text.set_fill_color( self.color * dimness );
        }
        target.draw_with_renderstates(&text, my_states);
    }

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

struct Explosion<'a> {
    background : &'a Sprite<'a>,
    explosion_sprite : [Sprite<'a> ; 12],
    master_clock : &'a Clock
}



impl<'a> Explosion<'a> {
    fn new( _background : &'a Sprite<'a>, _explosion_texture : &'a Texture, clock : &'a Clock ) -> Explosion<'a> {

        let mut number = Explosion{
            background : _background,
            explosion_sprite : [
                Sprite::with_texture( _explosion_texture ),
                Sprite::with_texture( _explosion_texture ),
                Sprite::with_texture( _explosion_texture ),
                Sprite::with_texture( _explosion_texture ),
                Sprite::with_texture( _explosion_texture ),
                Sprite::with_texture( _explosion_texture ),
                Sprite::with_texture( _explosion_texture ),
                Sprite::with_texture( _explosion_texture ),
                Sprite::with_texture( _explosion_texture ),
                Sprite::with_texture( _explosion_texture ),
                Sprite::with_texture( _explosion_texture ),
                Sprite::with_texture( _explosion_texture )
            ],
            master_clock : clock };

        for i in 0..12 {
            number.explosion_sprite[i].set_texture_rect( &IntRect::new(i as i32 * 96 ,0,96,96) );
            number.explosion_sprite[i].scale( Vector2f::new( TILE_SIZE / 96.0, TILE_SIZE / 96.0) );
        }
        number
    }
    fn draw<'se, 'tex, 'sh, 'shte>(
        &'se self,
        target: &mut dyn RenderTarget,
        states: RenderStates<'tex, 'sh, 'shte>,
        frame : usize
    )
    where
        'se: 'sh, {

        let my_states = RenderStates::new( states.blend_mode, states.transform, states.texture, states.shader );
        target.draw_with_renderstates( self.background, my_states );

        target.draw_with_renderstates( &self.explosion_sprite[frame], states );
    }

}

impl Drawable for Explosion<'_> {

    fn draw<'se, 'tex, 'sh, 'shte>(
        &'se self,
        target: &mut dyn RenderTarget,
        _states: RenderStates<'tex, 'sh, 'shte>
    )
    where
        'se: 'sh, {
        let mut frame = self.master_clock.elapsed_time().as_milliseconds() / (1000 / 48);
        frame = frame % 12;
        self.draw( target, _states, frame.try_into().unwrap()  );
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

    let explosion_texture = match Texture::from_file("explosion.png") {
        Some(exp_texture) => exp_texture,
        None => panic!("Texture error.")
    };

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

    let clock = Clock::start();

    let drawables: [& dyn Drawable; 31] = [
        &SpriteElement::new( &stone_sprite ),
        &RectangleShapeElement::new( Color::RED ),
        &RectangleShapeElement::new( Color::YELLOW ),
        &SpriteElement::new( &wood_sprite ),
        &Explosion::new( &wood_sprite, &explosion_texture, &clock ),
        &Number::new( &font, p_color[0], &wood_sprite, 1 ),
        &Number::new( &font, p_color[0], &wood_sprite, 2 ),
        &Number::new( &font, p_color[0], &wood_sprite, 3 ),
        &VolatileNumber::new( &font, p_color[0], &wood_sprite, 1, &clock ),
        &VolatileNumber::new( &font, p_color[0], &wood_sprite, 2, &clock ),
        &VolatileNumber::new( &font, p_color[0], &wood_sprite, 3, &clock ),
        &Number::new( &font, p_color[1], &wood_sprite, 1 ),
        &Number::new( &font, p_color[1], &wood_sprite, 2 ),
        &Number::new( &font, p_color[1], &wood_sprite, 3 ),
        &VolatileNumber::new( &font, p_color[1], &wood_sprite, 1, &clock ),
        &VolatileNumber::new( &font, p_color[1], &wood_sprite, 2, &clock ),
        &VolatileNumber::new( &font, p_color[1], &wood_sprite, 3, &clock ),
        &Number::new( &font, p_color[2], &wood_sprite, 1 ),
        &Number::new( &font, p_color[2], &wood_sprite, 2 ),
        &Number::new( &font, p_color[2], &wood_sprite, 3 ),
        &VolatileNumber::new( &font, p_color[2], &wood_sprite, 1, &clock ),
        &VolatileNumber::new( &font, p_color[2], &wood_sprite, 2, &clock ),
        &VolatileNumber::new( &font, p_color[2], &wood_sprite, 3, &clock ),
        &Number::new( &font, p_color[3], &wood_sprite, 1 ),
        &Number::new( &font, p_color[3], &wood_sprite, 2 ),
        &Number::new( &font, p_color[3], &wood_sprite, 3 ),
        &VolatileNumber::new( &font, p_color[3], &wood_sprite, 1, &clock ),
        &VolatileNumber::new( &font, p_color[3], &wood_sprite, 2, &clock ),
        &VolatileNumber::new( &font, p_color[3], &wood_sprite, 3, &clock ),
        &Number::new( &font, s_color, &wood_sprite, 1 ),
        &Number::new( &font, s_color, &wood_sprite, 2 ),
   ];

    let mut start_time = clock.elapsed_time();

    while window.is_open() {
        if !atoms.finished {
            let temp = clock.elapsed_time();
            let elapsed = temp - start_time;
            if elapsed.as_seconds() > 0.25 {
                atoms.recalculate_board();
                start_time = temp;
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
                                                  start_time = clock.elapsed_time();
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

        window.clear( Color::BLACK );

        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                let mut my_states = RenderStates::new( BlendMode::ALPHA, Transform::IDENTITY, None, None );
                my_states.transform.translate( x as f32 * TILE_SIZE, y as f32 * TILE_SIZE );
                let content = atoms.get_content( x, y );
                window.draw_with_renderstates( drawables[ content as usize ], my_states );
            }
        }

        for x in BOARD_SIZE..BOARD_SIZE+10 {
            for y in 0..BOARD_SIZE {
                let mut my_states = RenderStates::new( BlendMode::ALPHA, Transform::IDENTITY, None, None );
                my_states.transform.translate( x as f32 * TILE_SIZE, y as f32 * TILE_SIZE );
                window.draw_with_renderstates( drawables[ atoms::Drawable::Wall as usize ], my_states );
            }
        }

        let mut text = Text::new( "Score Board", &font, ( TILE_SIZE -5.0 ) as u32 );
        text.set_position( Vector2f::new(TILE_SIZE * ( ( BOARD_SIZE_F - 0.5 )  * 3.0 / 2.0 ),
                                         TILE_SIZE) );
        text.set_fill_color(Color::WHITE );

        // center text horizontally
        let text_rect = text.local_bounds();
        text.set_origin( Vector2f::new( text_rect.left + (text_rect.width/2.0), 0.0) );

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
            text.set_position( Vector2f::new(BOARD_SIZE_F * TILE_SIZE + 5.0,
                                             TILE_SIZE * (i as f32 +3.0 ) -5.0 ) );
            if i == atoms.get_current_player() {
                text.set_outline_thickness(5.0);
                text.set_fill_color( p_color[i] );
                text.set_outline_color(Color::WHITE);
            } else {
                text.set_fill_color( p_color[i] );
            }
            window.draw(&text);
        }

        // Display things on screen
        window.display()
    }
    atoms.dump_state();
}

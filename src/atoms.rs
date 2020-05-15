use std::cmp;

pub struct PseudoRandom {
    x: u64,
    w: u64,
    s: u64,
}

impl PseudoRandom {
    pub fn new() -> PseudoRandom {
        PseudoRandom{ x : 0, w : 0, s : 0xb5ad4eceda1ce2a9 }
    }

    fn msws(&mut self) -> u32 {
        self.x = self.x.wrapping_mul(self.x);
        self.w = self.w.wrapping_add(self.s);
        self.x = self.w.wrapping_add(self.w);
        self.x = (self.x>>32) | (self.x<<32);
        self.x as u32
    }
    pub fn range(&mut self, modu : u32) -> u32{
        self.msws() % (modu + 1)
    }
}

pub struct Atoms {
    pub editing : bool,
    pub finished : bool,
    rng : PseudoRandom,
    scores : [ u32; 4 ],
    current_player : usize,
    width : usize,
    height : usize,
    first_go: [bool;4],
    player : Vec<Vec<usize>>,
    map : Vec<Vec<u32>>,
    world : Vec<Vec<u32>>,
    other_world : Vec<Vec<u32>>,
}

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum Drawable {
    // Map Elements
    Wall = 0, Corner, Edge, Empty, Bang,

    // Player One
    P1One, P1Two, P1Three,
    P1VOne, P1VTwo, P1VThree,

    // Player Two
    P2One, P2Two, P2Three,
    P2VOne, P2VTwo, P2VThree,

    // Player Three
    P3One, P3Two, P3Three,
    P3VOne, P3VTwo, P3VThree,

    // Player Four
    P4One, P4Two, P4Three,
    P4VOne, P4VTwo, P4VThree,

    // Starting atoms
    SOne, STwo,
}

impl Atoms {

    pub fn new(_width : usize, _height : usize) -> Atoms {
        let mut atoms = Atoms{ width: _width,
                               height : _height,
                               current_player : 0,
                               editing : true,
                               finished : true,
                               rng : PseudoRandom::new(),
                               scores : [0 ; 4 ],
                               first_go : [true; 4],
                               player :vec![ vec! [ 0 ; _width ] ; _height],
                               map : vec![ vec! [ 0 ; _width ] ; _height],
                               world : vec![ vec! [ 0 ; _width ] ; _height],
                               other_world : vec![ vec! [ 0 ; _width ] ; _height] };
        atoms.clear( true );
        atoms.editing = false;
        atoms.clear( true );
        atoms
    }

    pub fn click(&mut self, i : usize, j : usize ) {
        if i < self.width && j < self.height {
            if self.editing {
                self.map[i][j] = if self.map[i][j] == 0 { 3 } else { 0 };
                self.calculate_map();
            } else {
                if self.map[i][j] != 0 && ( self.player[i][j] == self.current_player || self.world[i][j] == 0 ) {
                    self.world[i][j]+= 1;
                    self.player[i][j] = self.current_player;
                    self.finished = false;
                }
            }
        }
    }

    pub fn clear(&mut self, randomize : bool ) {
        if self.editing {
            for i in 0..self.height {
                for j in 0..self.width {
                    if i == 0 || j == 0 || i == self.height - 1 || j == self.width - 1 {
                        self.map[i][j] = 0;
                    } else {
                        self.map[i][j] = 3;
                    }
                }
            }
            self.calculate_map();
        } else {
            self.current_player = 0;
            self.scores = [0;4];
            self.first_go = [true;4];
            for i in 0..self.height {
                for j in 0..self.width {
                    if self.map[i][j] < 2 {
                        self.world[i][j] = 0;
                    } else {
                        self.world[i][j] = if randomize {self.rng.range(self.map[i][j]-1)} else {0}
                    }
                    self.other_world[i][j] = 0;
                    self.player[i][j] = 20;
                }
            }
        }
    }

    pub fn calculate_map(&mut self) {
        for i in 0..self.height {
            for j in 0..self.width {
                if self.map[i][j] != 0 {
                    self.map[i][j] =  3 -
                        if self.map[i-1][j] == 0 {1} else {0} -
                        if self.map[i][j-1] == 0 {1} else {0} -
                        if self.map[i+1][j] == 0 {1} else {0} -
                        if self.map[i][j+1] == 0 {1} else {0};
                }
            }
        }
    }

    pub fn recalculate_board(&mut self) {
        self.finished = true;
        for i in 0..self.height {
            for j in 0..self.width {
                self.other_world[i][j] = self.world[i][j];
            }
        }
        for i in 0..self.height {
            for j in 0..self.width {
                if self.map[i][j] != 0 {
                    if self.other_world[i][j] > self.map[i][j] {
                        self.world[i][j]-= self.map[i][j] + 1;
                        self.world[i-1][j]+=1;
                        self.world[i][j-1]+=1;
                        self.world[i+1][j]+=1;
                        self.world[i][j+1]+=1;
                        self.player[i-1][j]=self.current_player;
                        self.player[i][j-1]=self.current_player;
                        self.player[i+1][j]=self.current_player;
                        self.player[i][j+1]=self.current_player;
                        self.finished = false;
                    }
                } else {
                    self.world[i][j] = 0;
                }
            }
        }

        self.scores = [0;4];
        for i in 0..self.height {
            for j in 0..self.width {
                if self.map[i][j] != 0 {
                    if self.player[i][j] != 20 {
                        self.scores[self.player[i][j]] += self.world[i][j];
                    }
                }
            }
        }
        if self.finished
        {
            self.first_go[ self.current_player ] = false;
            loop {
                self.current_player = ( self.current_player + 1) % 4;
                if !(  self.scores[ self. current_player ] == 0 && ! self.first_go[  self.current_player ] ) {
                    break;
                }
            }
        }
    }

    pub fn get_content(&self, i : usize, j : usize) -> Drawable {
        if self.editing {
            match self.map[i][j] {
                0 => Drawable::Wall,
                1 => Drawable::Corner,
                2 => Drawable::Edge,
                _ => Drawable::Empty,
            }
        } else {
            if self.map[i][j] == 0 {
                Drawable::Wall
            }
            else if self.world[i][j] > self.map[i][j]  {
                Drawable::Bang
            }
            else {
                let is_volatile = self.world[i][j] == self.map[i][j];
                match self.world[i][j] {
                    0 => Drawable::Empty,
                    1 => match self.player[i][j] {
                        0 => if is_volatile {Drawable::P1VOne} else {Drawable::P1One},
                        1 => if is_volatile {Drawable::P2VOne} else {Drawable::P2One},
                        2 => if is_volatile {Drawable::P3VOne} else {Drawable::P3One},
                        3 => if is_volatile {Drawable::P4VOne} else {Drawable::P4One},
                        _ => Drawable::SOne,
                    },
                    2 => match self.player[i][j] {
                        0 => if is_volatile {Drawable::P1VTwo} else {Drawable::P1Two},
                        1 => if is_volatile {Drawable::P2VTwo} else {Drawable::P2Two},
                        2 => if is_volatile {Drawable::P3VTwo} else {Drawable::P3Two},
                        3 => if is_volatile {Drawable::P4VTwo} else {Drawable::P4Two},
                        _ => Drawable::STwo,
                    },
                    3 => match self.player[i][j] {
                        0 => if is_volatile {Drawable::P1VThree} else {Drawable::P1Three},
                        1 => if is_volatile {Drawable::P2VThree} else {Drawable::P2Three},
                        2 => if is_volatile {Drawable::P3VThree} else {Drawable::P3Three},
                        3 => if is_volatile {Drawable::P4VThree} else {Drawable::P4Three},
                        _ => Drawable::Bang,
                    },
                    _ => Drawable::Bang,
                }
            }
        }
    }

    pub fn get_current_player(&self) -> usize {
        self.current_player
    }

    pub fn is_player_dead(&self,  i : usize ) -> bool {
        self.scores[ i ] == 0 && !self.first_go[ i ]
    }

    pub fn get_player_score(&self,  i : usize ) -> u32 {
        self.scores[i]
    }

    pub fn game_over(&self) -> bool {
        if self.first_go[0] || self.first_go[1] || self.first_go[2] || self.first_go[3] {
            false
        } else {
            let mut max_score = 0;
            let mut total_atoms = 0;
            for score in 0..4 {
                max_score = cmp::max( max_score, self.scores[score] );
                total_atoms += self.scores[score];
            }
            (max_score == total_atoms)
        }
    }

    pub fn dump_state(&self) {
        for i in 0..self.width {
            for j in 0..self.height {
                print!( "{:02}:", self.player[i][j]);
            }
            println!();
        }
        println!();
        for i in 0..self.width {
            for j in 0..self.height {
                print!( "{:02}:", self.get_content(i,j) as u8 );
            }
            println!();
        }
        println!();
        for i in 0..self.width {
            for j in 0..self.height {
                print!( "{:02}:", self.world[i][j]);
            }
            println!();
        }
        println!();
        println!("{}", if self.game_over() {"Game over"} else {"Game not over"});
        for i in 0..4 {
            println!("{}:{}", if self.is_player_dead( i ) {"true "} else {"false"}, self.get_player_score(i));
        }
        println!();
    }
}

extern crate rand;

use rand::Rng;

use std::cmp;

struct Atoms {
    editing : bool,
    finished : bool,
    scores : [ i32; 4 ],
    current_player : usize,
    width : usize,
    height : usize,
    first_go: [bool;4],
    player : [[usize;10];10],
    map : [[i32;10];10],
    world : [[i32;10];10],
    other_world : [[i32;10];10]
}

#[derive(PartialEq)]
enum Drawable {
    // Map Elements
    Wall = 1, Corner, Edge, Empty, Bang,

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

   fn new(_width : usize, _height : usize) -> Atoms {
       let mut atoms = Atoms{ width: 10,
                              height : 10,
                              current_player : 0,
                              editing : true,
                              finished : false,
                              scores : [0,0,0,0],
                              first_go : [true; 4],
                              player : [[0;10];10],
                              map : [[0;10];10],
                              world : [[0;10];10],
                              other_world : [[0;10];10] };
       atoms.clear();
       atoms.editing = false;
       atoms.clear();
       atoms
   }

    fn click(&mut self, i : usize, j : usize ) {
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

    fn clear(&mut self) {
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
//            let mut rng = rand::thread_rng();
            self.current_player = 0;
            self.scores = [0;4];
            self.first_go = [true;4];
            for i in 0..self.height {
                for j in 0..self.width {
                    if self.map[i][j] < 2 {
                        self.world[i][j] = 0;
                    } else {
                        self.world[i][j] = 0;//rng.gen_range(0,map[i][j]-1);
                    }
                    self.other_world[i][j] = 0;
                    self.player[i][j] = 20;
                }
            }
        }
    }

    fn calculate_map(&mut self) {
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

    fn recalculate_board(&mut self) {
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

        if self.finished
        {
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
            self.first_go[ self.current_player ] = false;
            loop {
                self.current_player = ( self.current_player + 1) % 4;
                if !(  self.scores[ self. current_player ] == 0 && ! self.first_go[  self.current_player ] ) {
                    break;
                }
            }
        }
    }

    fn get_content(&self, i : usize, j : usize) -> Drawable {
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
            else {
                let is_volatile = self.world[i][j] == self.map[i][j];
                if self.world[i][j] > self.map[i][j]  {
                    Drawable::Bang
                }
                else {
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
    }

    fn get_current_player(&self) -> usize {self.current_player}

    fn is_player_dead(&self,  i : usize ) -> bool {
        self.scores[ i ] == 0 && !self.first_go[ i ]
    }

    fn get_player_score(&self,  i : usize ) -> i32 {
        self.scores[i]
    }

    fn game_over(&self) -> bool {
        let mut max_score = 0;
        let mut total_atoms = 0;
        if self.first_go[0] || self.first_go[1] || self.first_go[2] || self.first_go[3] {
            false
        } else {
            for score in 0..3 {
                max_score = cmp::max( max_score, self.scores[score] );
                total_atoms += self.scores[score];
            }
            (max_score == total_atoms)
        }
    }

    fn dump_state(&self) {
        for i in 0..10 {
            for j in 0..10 {
                print!( "{:02}:", self.player[i][j]);
            }
            println!();
        }
        println!();
        for i in 0..10 {
            for j in 0..10 {
                print!( "{:02}:", self.world[i][j]);// ) as u8);
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

fn main() {
    let mut atoms = Atoms::new( 10, 10);

    atoms.dump_state();
  let clicks = [ [ 8, 3 ], [ 7, 6 ], [ 1, 2 ], [ 8, 1 ], [ 6, 8 ], [ 5, 7 ], [ 8, 7 ], [ 1, 8 ], [ 8, 8 ], [ 4, 3 ], [ 3, 7 ], [ 2, 3 ], [ 4, 8 ], [ 3, 3 ], [ 3, 5 ], [ 7, 2 ], [ 7, 1 ], [ 3, 1 ], [ 3, 7 ], [ 1, 8 ], [ 8, 3 ], [ 7, 6 ], [ 8, 7 ], [ 2, 3 ], [ 7, 1 ], [ 5, 7 ], [ 8, 7 ], [ 2, 8 ], [ 8, 5 ], [ 1, 4 ], [ 8, 7 ], [ 3, 8 ], [ 4, 7 ], [ 4, 3 ], [ 8, 8 ], [ 1, 8 ], [ 4, 5 ], [ 3, 1 ], [ 7, 3 ], [ 1, 6 ], [ 7, 5 ], [ 3, 8 ], [ 1, 7 ], [ 2, 3 ], [ 8, 3 ], [ 4, 3 ], [ 1, 7 ], [ 3, 7 ], [ 8, 2 ], [ 8, 3 ], [ 3, 2 ], [ 4, 7 ], [ 8, 2 ], [ 7, 1 ], [ 1, 1 ], [ 4, 6 ], [ 7, 2 ], [ 4, 3 ], [ 1, 2 ], [ 2, 6 ], [ 7, 1 ], [ 4, 2 ], [ 8, 6 ], [ 5, 8 ], [ 7, 3 ], [ 5, 3 ], [ 1, 1 ], [ 2, 7 ], [ 1, 2 ], [ 1, 4 ], [ 3, 2 ], [ 3, 7 ], [ 7, 6 ], [ 4, 8 ], [ 5, 5 ] ];
 //   let clicks = [ [ 8, 3 ], [ 7, 6 ], [ 1, 2 ], [ 8, 1 ] ];

    for click in clicks.iter() {
        atoms.click(click[0],click[1]);
        atoms.dump_state();
        while !atoms.finished && !atoms.game_over() {
            atoms.recalculate_board();
            atoms.dump_state();
        }
    }

    println!("{}", atoms.get_current_player() );
    

}

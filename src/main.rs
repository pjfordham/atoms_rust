//extern crate rand;

//use rand::Rng;

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
                              finished : true,
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
            for score in 0..4 {
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
                print!( "{:02}:", self.get_content(i,j) as u8 );
            }
            println!();
        }
        println!();
        for i in 0..10 {
            for j in 0..10 {
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

fn main() {
    let mut atoms = Atoms::new( 10, 10);

    atoms.dump_state();
  let clicks = [ [ 9, 4 ], [ 8, 3 ], [ 9, 1 ], [ 7, 6 ], [ 5, 0 ], [ 9, 3 ], [ 6, 5 ], [ 1, 2 ], [ 3, 0 ], [ 5, 1 ], [ 8, 1 ], [ 6, 8 ], [ 9, 0 ], [ 9, 0 ], [ 1, 0 ], [ 6, 7 ], [ 5, 7 ], [ 6, 3 ], [ 5, 2 ], [ 5, 7 ], [ 8, 7 ], [ 6, 8 ], [ 2, 5 ], [ 0, 2 ], [ 5, 1 ], [ 2, 2 ], [ 5, 8 ], [ 0, 0 ], [ 2, 8 ], [ 2, 5 ], [ 4, 1 ], [ 1, 5 ], [ 1, 8 ], [ 9, 1 ], [ 0, 4 ], [ 8, 8 ], [ 4, 3 ], [ 7, 6 ], [ 4, 1 ], [ 3, 7 ], [ 2, 2 ], [ 1, 0 ], [ 5, 7 ], [ 1, 9 ], [ 2, 3 ], [ 2, 2 ], [ 6, 4 ], [ 0, 6 ], [ 4, 8 ], [ 6, 3 ], [ 9, 9 ], [ 2, 5 ], [ 8, 3 ], [ 2, 2 ], [ 0, 3 ], [ 1, 7 ], [ 3, 3 ], [ 3, 5 ], [ 5, 3 ], [ 7, 2 ], [ 5, 6 ], [ 0, 2 ], [ 3, 6 ], [ 7, 1 ], [ 4, 0 ], [ 8, 6 ], [ 7, 7 ], [ 5, 0 ], [ 0, 0 ], [ 3, 1 ], [ 6, 4 ], [ 2, 4 ], [ 7, 7 ], [ 7, 1 ], [ 3, 7 ], [ 1, 8 ], [ 6, 3 ], [ 9, 4 ], [ 0, 7 ], [ 8, 3 ], [ 5, 2 ], [ 4, 9 ], [ 9, 0 ], [ 7, 6 ], [ 8, 7 ], [ 4, 9 ], [ 4, 1 ], [ 7, 6 ], [ 3, 6 ], [ 9, 4 ], [ 4, 2 ], [ 0, 7 ], [ 0, 5 ], [ 5, 8 ], [ 7, 9 ], [ 5, 9 ], [ 9, 8 ], [ 5, 6 ], [ 5, 8 ], [ 6, 6 ], [ 5, 6 ], [ 4, 8 ], [ 2, 3 ], [ 7, 2 ], [ 2, 2 ], [ 1, 0 ], [ 5, 6 ], [ 6, 2 ], [ 3, 3 ], [ 0, 7 ], [ 7, 1 ], [ 3, 7 ], [ 6, 7 ], [ 7, 0 ], [ 5, 7 ], [ 6, 7 ], [ 7, 7 ], [ 3, 8 ], [ 7, 0 ], [ 7, 7 ], [ 5, 2 ], [ 7, 4 ], [ 1, 7 ], [ 5, 7 ], [ 2, 8 ], [ 9, 9 ], [ 3, 9 ], [ 9, 3 ], [ 8, 7 ], [ 4, 1 ], [ 4, 6 ], [ 2, 8 ], [ 1, 3 ], [ 5, 8 ], [ 8, 5 ], [ 5, 5 ], [ 1, 4 ], [ 8, 7 ], [ 1, 9 ], [ 7, 1 ], [ 3, 8 ], [ 5, 3 ], [ 2, 8 ], [ 4, 7 ], [ 5, 6 ], [ 9, 5 ], [ 4, 3 ], [ 6, 0 ], [ 1, 4 ], [ 2, 9 ], [ 5, 1 ], [ 9, 1 ], [ 6, 7 ], [ 0, 7 ], [ 3, 6 ], [ 6, 8 ], [ 9, 8 ], [ 8, 8 ], [ 7, 7 ], [ 1, 8 ], [ 0, 0 ], [ 4, 8 ], [ 6, 9 ], [ 7, 4 ], [ 9, 4 ], [ 4, 5 ], [ 1, 5 ], [ 3, 1 ], [ 4, 1 ], [ 9, 3 ], [ 3, 9 ], [ 9, 6 ], [ 9, 5 ], [ 8, 3 ], [ 2, 5 ], [ 2, 0 ], [ 9, 7 ], [ 4, 8 ], [ 1, 3 ], [ 2, 3 ], [ 3, 7 ], [ 4, 6 ], [ 7, 3 ], [ 0, 5 ], [ 1, 6 ], [ 6, 1 ], [ 6, 3 ], [ 0, 9 ], [ 2, 7 ], [ 7, 7 ], [ 4, 9 ], [ 7, 5 ], [ 7, 1 ], [ 6, 8 ], [ 7, 3 ], [ 4, 1 ], [ 4, 9 ], [ 7, 8 ], [ 3, 8 ], [ 1, 0 ], [ 1, 7 ], [ 9, 4 ], [ 9, 5 ], [ 2, 4 ], [ 7, 3 ], [ 3, 4 ], [ 6, 0 ], [ 2, 9 ], [ 2, 3 ], [ 5, 1 ], [ 6, 4 ], [ 2, 7 ], [ 8, 3 ], [ 8, 5 ], [ 4, 3 ], [ 5, 1 ], [ 5, 2 ], [ 4, 2 ], [ 6, 3 ], [ 3, 7 ], [ 3, 3 ], [ 2, 6 ], [ 0, 3 ], [ 4, 3 ], [ 3, 0 ], [ 8, 1 ], [ 1, 7 ], [ 3, 9 ], [ 3, 0 ], [ 6, 7 ], [ 3, 7 ], [ 0, 6 ], [ 5, 4 ], [ 4, 0 ], [ 5, 2 ], [ 5, 7 ], [ 1, 5 ], [ 8, 2 ], [ 3, 7 ], [ 6, 1 ], [ 1, 0 ], [ 1, 2 ], [ 1, 3 ], [ 8, 6 ], [ 6, 7 ], [ 2, 4 ], [ 3, 6 ], [ 4, 1 ], [ 8, 1 ], [ 1, 3 ], [ 2, 5 ], [ 1, 6 ], [ 8, 3 ], [ 5, 6 ], [ 5, 0 ], [ 9, 9 ], [ 1, 7 ], [ 6, 6 ], [ 9, 3 ], [ 3, 0 ], [ 8, 2 ], [ 6, 8 ], [ 3, 2 ], [ 0, 1 ], [ 8, 7 ], [ 4, 7 ], [ 8, 2 ], [ 6, 3 ], [ 0, 6 ], [ 8, 2 ], [ 8, 2 ], [ 3, 0 ], [ 2, 7 ], [ 8, 5 ], [ 5, 1 ], [ 9, 2 ], [ 7, 1 ], [ 2, 4 ], [ 9, 0 ], [ 9, 3 ], [ 6, 3 ], [ 1, 1 ], [ 4, 6 ], [ 2, 4 ], [ 3, 5 ], [ 6, 9 ], [ 5, 9 ], [ 5, 5 ], [ 7, 2 ], [ 6, 1 ], [ 4, 8 ], [ 5, 7 ], [ 5, 3 ], [ 7, 5 ], [ 5, 9 ], [ 3, 5 ], [ 3, 0 ], [ 5, 3 ], [ 9, 8 ], [ 7, 5 ], [ 0, 2 ], [ 7, 5 ], [ 7, 3 ], [ 6, 2 ], [ 4, 8 ], [ 4, 3 ], [ 3, 9 ], [ 5, 3 ], [ 3, 7 ], [ 7, 6 ], [ 1, 4 ], [ 8, 5 ], [ 6, 1 ], [ 6, 1 ], [ 9, 2 ], [ 2, 6 ], [ 2, 1 ], [ 1, 5 ], [ 2, 1 ], [ 1, 2 ], [ 6, 4 ], [ 3, 0 ], [ 5, 5 ], [ 9, 8 ], [ 2, 6 ], [ 4, 2 ], [ 7, 1 ], [ 9, 3 ], [ 4, 6 ], [ 7, 0 ], [ 3, 5 ], [ 1, 5 ], [ 4, 8 ], [ 6, 7 ], [ 4, 2 ], [ 7, 1 ], [ 5, 1 ], [ 5, 6 ], [ 2, 0 ], [ 7, 1 ], [ 9, 5 ], [ 8, 9 ], [ 5, 3 ], [ 0, 0 ], [ 3, 3 ], [ 1, 8 ], [ 8, 6 ], [ 5, 8 ], [ 5, 7 ]];

    for click in clicks.iter() {
        println!("[ {}, {} ]", click[0], click[1]);
        atoms.click(click[0],click[1]);
        atoms.dump_state();
        while !atoms.finished && !atoms.game_over() {
            atoms.recalculate_board();
            atoms.dump_state();
        }
    }
            atoms.dump_state();

    println!("{}", atoms.get_current_player() );
    

}

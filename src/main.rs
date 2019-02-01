/**
*	Rust Snake by kemicofa
*	
*
*/

use std::time::{Duration, Instant};
use std::thread::sleep;

struct Direction {
	x: i32,
	y: i32,
}

/** snake structure */
struct Snake {
        head: Block,
	body: Vec<Block>,
	dir: Direction,
}

/** struct methods for Snake */
impl Snake {
        fn new() -> Snake {
            Snake {
                head: Block{x:3,y:2,s:'@'},
                body: Vec::new(),
                dir: Direction{x:1,y:0}
            }
        }
	/** returns the current coordinates of the head position */
	fn current_position(&mut self)->(i32,i32) {
		let block = self.body.first().unwrap();
	        (block.x, block.y)
        }

	/** moves the snake by one to the current direction it's going */
	fn move_forward(&mut self){
                let len = self.body.len();
                self.body.rotate_right(len);
                if let Some(block) = self.body.first_mut(){
                    block.update(self.head.x, self.head.y);
                };
		self.head.update(self.head.x + self.dir.x, self.head.y + self.dir.y);	
	}

	/** adds a block to the snake */
	fn grow(&mut self){
		let block = Block {x: self.head.x, y: self.head.y, s: '#'};
		self.body.insert(0, block);
		self.head.update(self.head.x + self.dir.x, self.head.y + self.dir.y);
	}

	/** determines if the given coordinates belong to the snake */
	fn is_self(&self, x: i32, y: i32)->bool{
		self.body.iter().any(|block| block.x == x && block.y == y)
	}
} 


/** block peice used for the snake */
struct Block {
	x: i32,
	y: i32,
	s: char,
}

impl Block {
	fn update(&mut self, x: i32, y: i32){
		self.x = x;
		self.y = y;
	}
}

struct Game {
	score: i32,
	speed: i32,
        snake: Snake
}

impl Game {
        
        fn new() -> Game {
            Game {
                score: 0,
                speed: 0,
                snake: Snake::new()
            }
        }

	/** method used to start the game, also resets key variables */
	fn start(&mut self){
		self.score = 0;
		self.speed = 0;
		self.game_loop();
	}
	/** method that ends the snake game */
	fn end(&self){}
	/** game loop 60fps */
	fn game_loop(&self){
            
            let milli_per_frame:u64 = 1000/60;

            loop {
                let now = Instant::now();
                self.update();
                self.draw();
                
                
                let time_to_sleep = milli_per_frame - now.elapsed().as_millis();
                println!("{}", time_to_sleep);
                if time_to_sleep > 0 { sleep(Duration::from_millis(time_to_sleep)); }             
            }

        }
	/** method that updates all entities that need to be drawn */
	fn update(&self){}
	/** draws all entities */
	fn draw(&self){}
}

fn main() {
	let mut game = Game::new();
	game.start();
}


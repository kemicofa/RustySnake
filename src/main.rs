/**
*	Rust Snake by kemicofa
*	
*
*/

struct Direction {
	x: i32,
	y: i32,
}

/** snake structure */
struct Snake {
	blocks: Vec<Block>,
	dir: Direction,
}

/** struct methods for Snake */
impl Snake {
	/** returns the current coordinates of the head position */
	fn current_position(&self)->(i32,i32) {
		let block = self.blocks.first().unwrap();
		(block.x, block.y)
	}

	/** moves the snake by one to the current direction it's going */
	fn move_forward(&mut self){
		let bf = self.blocks.first().unwrap();
		let mut blocks = &self.blocks;
		let bl = blocks.pop().unwrap();
		bl.update(bf.x, bf.y);
		self.blocks.insert(1, bl);
		bf.update(bf.x + self.dir.x, bf.y + self.dir.y);	
	}

	/** adds a block to the snake */
	fn grow(&mut self){
		let bf = self.blocks.first().unwrap();
		let b = Block {x: bf.x, y: bf.y, s: '#'};
		self.blocks.insert(1, b);
		bf.update(bf.x + self.dir.x, bf.y + self.dir.y);
	}

	/** determines if the given coordinates belong to the snake */
	fn is_self(&self, x: i32, y: i32)->bool{
		loop {
			match self.blocks.iter().next() {
				Some(block) => { if block.x == x && block.y == y { break true }},
				None => { break false }
			}
		}
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
}

impl Game {
	/** method used to start the game, also resets key variables */
	fn start(&mut self){
		self.score = 0;
		self.speed = 0;
		self.game_loop();
	}
	/** method that ends the snake game */
	fn end(&self){}
	/** game loop */
	fn game_loop(&self){}
	/** method that updates all entities that need to be drawn */
	fn update(&self){}
	/** draws all entities */
	fn draw(&self){}
}

fn main() {
	let mut game = Game{score: 0, speed: 0};
	game.start();
}


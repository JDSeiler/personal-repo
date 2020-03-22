use rand::prelude::*;
use rand::distributions::{Distribution, Uniform};
use image;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
struct BrownianField {
    matrix: Vec<Vec<u8>>,
    size: usize,
    agent_x: usize,
    agent_y: usize,
}

impl BrownianField {
    fn new(field_size: usize) -> BrownianField {
	let mut outer_vec = Vec::with_capacity(field_size);
	for _i in 0..field_size {
	    outer_vec.push(vec![0; field_size])
	}
	BrownianField {
	    matrix: outer_vec,
	    size: field_size,
	    agent_x: field_size / 2,
	    agent_y: field_size / 2
	}
    }

    fn choose_direction() -> Direction {
	let gen_range = Uniform::from(0..4);
	let mut rng = thread_rng();
	let value: i32 = gen_range.sample(&mut rng);
	match value {
	    0 => Direction::Up,
	    1 => Direction::Down,
	    2 => Direction::Left,
	    3 => Direction::Right,
	    _ => panic!()
	}
    }

    fn is_valid_direction(&self, chosen_direction: &Direction) -> bool {
	let maximum_index = self.matrix.capacity() - 1;
	match chosen_direction {
	    Direction::Up => if self.agent_y == maximum_index { false } else { true },
	    Direction::Down => if self.agent_y == 0 { false } else { true },
	    Direction::Left => if self.agent_x == 0 { false } else { true },
	    Direction::Right => if self.agent_x == maximum_index { false } else { true }
	}

    }

    fn move_agent(&mut self) {
	let chosen_direction = BrownianField::choose_direction();
	match self.is_valid_direction(&chosen_direction) {
	    true => {
		self.update_matrix(&chosen_direction);
	    },
	    false => {
		// do nothing, can't move outside the grid
	    }
	}
    }

    fn update_matrix(&mut self, validated_direction: &Direction) {
	match validated_direction {
	    Direction::Up => {
		self.agent_y += 1;
	    },
	    Direction::Down => {
		self.agent_y -= 1;
	    },
	    Direction::Left => {
		self.agent_x -= 1;
	    }
	    Direction::Right => {
		self.agent_x += 1;
	    }
	}

	let column = self.matrix.get_mut(self.agent_x);

	let cell = column.unwrap().get_mut(self.agent_y).unwrap();
	
	match cell < &mut 250 {
	    true => { *cell += 5; },
	    false => {
		println!("Cell is already at max value");
	    }
	}
    }

    fn peek_cell(&self, x: usize, y: usize) -> u8 {
	let cell = self.matrix.get(x).unwrap().get(y).unwrap();
	*cell
    }

    fn debug_print_matrix(&self) {
	let empty_cell = " ";
	let grade_one = "░";
	let grade_two = "▒";
	let grade_three = "▓";

	for column_index in 0..self.size {
	    let column = self.matrix.get(column_index).unwrap();
	    for cell in column {
		if *cell == 0 {
		    print!("{}", empty_cell);
		} else if *cell < 10 {
		    print!("{}", grade_one);
		} else if *cell < 20 {
		    print!("{}", grade_two);
		} else {
		    print!("{}", grade_three);
		}
	    }
	    println!("");
	}
    }
}

fn generate_image(bf: &BrownianField) {
    let size: u32 = bf.size as u32;
    let mut buffer: image::RgbImage = image::ImageBuffer::new(size, size);

    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
	let cell_value = bf.peek_cell(x as usize, y as usize);
	let gray_scale = 255 - cell_value;
	*pixel = image::Rgb([gray_scale, gray_scale, gray_scale]);
    }

    buffer.save("random_walk.png").unwrap();
}


fn main() {
    let field_size = 3000;
    let mut bf = BrownianField::new(field_size);
    for _i in 0..60_000_000 {
	bf.move_agent();
    }
    generate_image(&bf);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
	let _new_bf = BrownianField::new(10);
    }

    #[test]
    fn choose_direction() {
	for _ in 0..100 {
	    let _direction = BrownianField::choose_direction();
	}
    }

    #[test]
    fn validate_direction() {
	let field_size = 10;
	let mut bf = BrownianField::new(field_size);
	let some_direction = BrownianField::choose_direction();
	assert_eq!(bf.is_valid_direction(&some_direction), true);

	bf.agent_x = field_size - 1;
	assert_eq!(bf.is_valid_direction(&Direction::Right), false);
	bf.agent_y = field_size - 1;
	assert_eq!(bf.is_valid_direction(&Direction::Up), false);

	bf.agent_x = 0;
	assert_eq!(bf.is_valid_direction(&Direction::Left), false);
	bf.agent_y = 0;
	assert_eq!(bf.is_valid_direction(&Direction::Down), false);

    }

    #[test]
    fn validate_move() {
	let field_size = 10;
	let mut bf = BrownianField::new(field_size);

	bf.move_agent();
	let agent_x = bf.agent_x;
	let agent_y = bf.agent_y;

	assert_eq!(bf.peek_cell(agent_x, agent_y), 1);
    }
}


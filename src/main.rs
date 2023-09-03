use rand::{thread_rng, Rng};
use std::collections::HashMap;

// x, z, -x, -z

#[derive(PartialEq, Copy, Clone)]
struct Square {
    walls: [bool; 4],
    x: usize,
    z: usize,
}

impl Square {
    pub fn new_walls(x: usize, z: usize, walls: [bool; 4]) -> Self {
        Self { walls, x, z }
    }

    pub fn new(x: usize, z: usize) -> Self {
        Self {
            walls: [false, false, false, false],
            x,
            z,
        }
    }

    pub fn has_wall(&self, index: usize) -> bool {
        self.walls[index]
    }

    pub fn set_wall(&mut self, index: usize, new_value: bool) {
        self.walls[index] = new_value
    }

    pub fn x(&self) -> usize {
        self.x
    }
    pub fn z(&self) -> usize {
        self.z
    }
}

#[derive(Clone)]
struct Grid {
    squares: Vec<Vec<Square>>,
}

impl Grid {
    pub fn new(x: usize, z: usize) -> Self {
        let mut squares = Vec::new();

        for x_i in 0..x {
            let mut zs = Vec::new();
            for z_i in 0..z {
                zs.push(Square::new(x_i, z_i));
            }
            squares.push(zs);
        }
        Self { squares }
    }

    pub fn is_valid(&self) -> bool {
        let mut square_stack: Vec<Square> = Vec::new();
        let mut visited: Vec<Square> = Vec::new();

        square_stack.push(self.squares[0][0]);

        while !square_stack.is_empty() {
            let square = square_stack.pop().unwrap();
            if visited.contains(&square) {
                continue;
            }

            if square.x() < self.squares.len() - 1 && !square.has_wall(0) {
                let new_square = self.squares[square.x() + 1][square.z()];
                if !visited.contains(&new_square) {
                    square_stack.push(new_square);
                }
            }

            if square.x() > 0 && !square.has_wall(2) {
                let new_square = self.squares[square.x() - 1][square.z()];
                if !visited.contains(&new_square) {
                    square_stack.push(new_square);
                }
            }

            if square.z() < self.squares[0].len() - 1 && !square.has_wall(1) {
                let new_square = self.squares[square.x()][square.z() + 1];
                if !visited.contains(&new_square) {
                    square_stack.push(new_square);
                }
            }

            if square.z() > 0 && !square.has_wall(3) {
                let new_square = self.squares[square.x()][square.z() - 1];
                if !visited.contains(&new_square) {
                    square_stack.push(new_square);
                }
            }

            visited.push(square);
        }

        visited.len() == self.squares.len() * self.squares[0].len()
    }

    pub fn set_wall(&mut self, x: usize, z: usize, index: usize, has_wall: bool) {
        self.squares[x][z].set_wall(index, has_wall);

        if index == 0 && x < self.squares.len() - 1 {
            self.squares[x + 1][z].set_wall(2, has_wall);
        } else if index == 1 && z < self.squares[0].len() - 1 {
            self.squares[x][z + 1].set_wall(3, has_wall);
        } else if index == 2 && x > 0 {
            self.squares[x - 1][z].set_wall(0, has_wall);
        } else if index == 3 && z > 0 {
            self.squares[x][z - 1].set_wall(1, has_wall);
        }
    }

    pub fn get_square(&self, x: usize, z: usize) -> Square {
        self.squares[x][z]
    }

    pub fn get_wall(&self, x: usize, z: usize, index: usize) -> bool {
        self.squares[x][z].has_wall(index)
    }

    pub fn randomize_grid(&mut self) {
        let x_size = self.squares.len();
        let z_size = self.squares[0].len();

        for _ in 0..(x_size * z_size * 2) {
            let mut rng = thread_rng();
            let x = rng.gen_range(0..x_size);
            let z = rng.gen_range(0..z_size);
            let index = rng.gen_range(0..4);

            let old_value = self.get_wall(x, z, index);

            self.set_wall(x, z, index, !old_value);

            if !self.is_valid() {
                self.set_wall(x, z, index, old_value);
            }
        }
    }

    pub fn set_border_walls(&mut self) {
        let z_size = self.squares[0].len();
        let x_size = self.squares.len();
        for x in 0..x_size {
            self.squares[x][0].set_wall(3, true);
            self.squares[x][z_size - 1].set_wall(1, true);
        }
        for z in 0..z_size {
            self.squares[0][z].set_wall(2, true);
            self.squares[x_size - 1][z].set_wall(0, true);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Grid;

    #[test]
    fn test_grid() {
        let mut grid = Grid::new(5, 5);

        assert!(grid.is_valid());

        for x in 0..5 {
            grid.set_wall(x, 2, 1, true);
        }

        assert!(!grid.is_valid());

        grid.set_wall(2, 2, 1, false);

        assert!(grid.is_valid());
    }
}

fn main() {
    let mut rng = thread_rng();

    let input: [Square; 16] = [
        Square::new_walls(1, 200, [false, false, true, true]),
        Square::new_walls(1, 212, [true, false, true, false]),
        Square::new_walls(1, 224, [false, true, false, true]),
        Square::new_walls(1, 236, [false, true, true, false]),
        Square::new_walls(13, 200, [false, false, true, false]),
        Square::new_walls(13, 212, [false, false, false, true]),
        Square::new_walls(13, 224, [true, false, false, false]),
        Square::new_walls(13, 236, [false, true, false, false]),
        Square::new_walls(25, 200, [false, false, false, false]),
        Square::new_walls(25, 212, [true, true, true, false]),
        Square::new_walls(25, 224, [false, true, true, true]),
        Square::new_walls(25, 236, [false, false, false, false]),
        Square::new_walls(37, 200, [true, false, false, true]),
        Square::new_walls(37, 212, [true, false, true, true]),
        Square::new_walls(37, 224, [true, true, false, true]),
        Square::new_walls(37, 236, [true, true, false, false]),
    ];

    let mut source_square_ids: HashMap<[bool; 4], String> = HashMap::new();

    let mut total = 0;
    for square in input {
        source_square_ids.insert(square.walls, format!("cstr{}", total));
        println!("<structure clear=\"false\" id=\"{}\"><region><cuboid min=\"{},{},{}\" size=\"10,10,10\"/></region></structure>", source_square_ids.get(&square.walls).expect("?"), square.x(), 0, square.z());
        total += 1;
    }

    let mut square_cache: HashMap<[bool; 4], Vec<Square>> = HashMap::new();

    for square in input {
        if square_cache.contains_key(&square.walls) {
            let squares = square_cache.get_mut(&square.walls).expect("WTF");
            squares.push(square);
        } else {
            let mut vec = Vec::new();
            vec.push(square);
            square_cache.insert(square.walls, vec);
        }
    }

    for grid_try in 0..15 {
        let x_size = 5;
        let z_size = 5;

        let mut grid = Grid::new(x_size, z_size);

        grid.randomize_grid();
        grid.set_border_walls();
        grid.set_wall(2, 0, 3, false);
        grid.set_wall(2, 4, 1, false);

        let mut total = 0;
        for x in 0..x_size {
            for z in 0..z_size {
                let destination_square = grid.get_square(x, z);
                let options = square_cache.get(&destination_square.walls).expect("What?");
                let index = rng.gen_range(0..options.len());
                let source_square = options.get(index).expect("Wslkdsjf");
                println!("<dynamic trigger=\"always\" id=\"{}dstrid{}\" structure=\"{}\" location=\"{},0,{}\"><filter><variable var=\"structure_choice\">{}</variable></filter></dynamic>", grid_try, total, source_square_ids.get(&source_square.walls).expect("?"), x * 10, z * 10, grid_try);
                total += 1;
            }
        }
    }
}

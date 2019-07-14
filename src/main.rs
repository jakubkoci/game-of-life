extern crate piston_window;

use piston_window::*;
use std::fmt;
use std::thread;

const MILLISECOND: u32 = 1000 * 1000;

fn main() {
    let width = 640;
    let height = 480;

    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [width, height])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let cell_size = 20;
    let rows = height / cell_size;
    let columns = width / cell_size;

    let live = [0.0, 0.0, 0.0, 1.0];
    let dead = [1.0, 1.0, 1.0, 1.0];
    //
    let mut world = World::new(rows, columns);

    // Blinker
    world.update_cell(10, 10, true);
    world.update_cell(11, 10, true);
    world.update_cell(12, 10, true);

    while let Some(event) = window.next() {
        println!("render...");
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);

            world.for_each_cell(|i, j| {
                let is_alive = world.is_alive(i, j);
                rectangle(
                    if is_alive == true { live } else { dead },
                    [
                        (j * cell_size) as f64,
                        (i * cell_size) as f64,
                        cell_size as f64,
                        cell_size as f64,
                    ],
                    context.transform,
                    graphics,
                );
            })
        });
        world = world.next_state();
        thread::sleep(std::time::Duration::new(0, 100 * MILLISECOND));
    }
}

struct World {
    rows: u32,
    columns: u32,
    cells: Vec<Vec<bool>>,
}

impl World {
    fn new(rows: u32, columns: u32) -> World {
        World {
            rows,
            columns,
            cells: vec![vec![false; columns as usize]; rows as usize],
        }
    }

    fn is_alive(&self, row: u32, column: u32) -> bool {
        let is_alive = self.cells[row as usize][column as usize];
        return is_alive;
    }

    fn update_cell(&mut self, row: u32, column: u32, value: bool) {
        self.cells[row as usize][column as usize] = value;
    }

    fn count_living_neighbours(&self, row: u32, column: u32) -> u32 {
        let mut result = 0;
        if (row > 0 && column > 0) && self.is_alive(row - 1, column - 1) {
            result += 1
        }
        if (row > 0) && self.is_alive(row - 1, column) {
            result += 1
        }
        if (row > 0 && column < self.columns - 1) && self.is_alive(row - 1, column + 1) {
            result += 1
        }
        if (column > 0) && self.is_alive(row, column - 1) {
            result += 1
        }
        if (column < self.columns - 1) && self.is_alive(row, column + 1) {
            result += 1
        }
        if (row < self.rows - 1 && column > 0) && self.is_alive(row + 1, column - 1) {
            result += 1
        }
        if (row < self.rows - 1) && self.is_alive(row + 1, column) {
            result += 1
        }
        if (row < self.rows - 1 && column < self.columns - 1) && self.is_alive(row + 1, column + 1)
        {
            result += 1
        }
        result
    }

    fn next_state_of_cell(&self, row: u32, column: u32) -> bool {
        if self.is_alive(row, column) {
            if self.count_living_neighbours(row, column) < 2 {
                return false;
            }
            if self.count_living_neighbours(row, column) > 3 {
                return false;
            }
        }

        if !self.is_alive(row, column) {
            if self.count_living_neighbours(row, column) != 3 {
                return false;
            }
        }

        return true;
    }

    fn next_state(&self) -> World {
        let mut new_world = World::new(self.rows, self.columns);
        for row in 0..self.rows {
            for column in 0..self.columns {
                let cell_next_state = self.next_state_of_cell(row, column);
                new_world.update_cell(row, column, cell_next_state);
            }
        }
        new_world
    }

    fn for_each_cell<F>(&self, mut func: F)
    where
        F: FnMut(u32, u32),
    {
        let rows = self.rows;
        let columns = self.columns;
        for i in 0..rows {
            for j in 0..columns {
                func(i, j);
            }
        }
    }
}

impl fmt::Display for World {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.

        let mut output = String::new();

        let rows = self.rows;
        let columns = self.columns;
        for i in 0..rows {
            output.push_str("\n");
            for j in 0..columns {
                let cell = if self.is_alive(i, j) { "x" } else { "o" };
                output.push_str(cell);
            }
        }

        write!(f, "{}", output)
    }
}

#[test]
fn creates_world_with_dead_cell() {
    let world = World::new(10, 10);
    assert_eq!(false, world.is_alive(0, 0));
    assert_eq!(false, world.is_alive(9, 9));
}

#[test]
fn updates_cell_at_given_coordinates() {
    let mut world = World::new(10, 10);

    world.update_cell(0, 0, true);
    world.update_cell(9, 9, true);

    assert_eq!(true, world.is_alive(0, 0));
    assert_eq!(true, world.is_alive(9, 9));
}

#[test]
fn implements_display_trait() {
    let mut world = World::new(10, 10);

    world.update_cell(0, 0, true);
    world.update_cell(9, 9, true);

    println!("\nPrint World {}\n", world);
}

#[test]
fn counts_living_diagonal_neighbours() {
    let mut world = World::new(3, 3);

    world.update_cell(0, 0, true);
    world.update_cell(0, 2, true);
    world.update_cell(2, 0, true);
    world.update_cell(2, 2, true);

    println!("\nPrint World {}\n", world);
    assert_eq!(4, world.count_living_neighbours(1, 1))
}

#[test]
fn counts_living_perpendicular_neighbours() {
    let mut world = World::new(3, 3);

    world.update_cell(0, 1, true);

    world.update_cell(1, 0, true);
    world.update_cell(1, 2, true);
    world.update_cell(2, 1, true);

    println!("\nPrint World {}\n", world);
    assert_eq!(4, world.count_living_neighbours(1, 1))
}

#[test]
fn living_cell_with_less_than_2_neighbours_will_die() {
    let mut world = World::new(3, 3);

    world.update_cell(1, 1, true);

    world.update_cell(1, 0, true);

    println!("\nPrint World {}\n", world);
    assert_eq!(false, world.next_state_of_cell(1, 1));
}

#[test]
fn living_cell_with_2_neighbours_will_live() {
    let mut world = World::new(3, 3);

    world.update_cell(1, 1, true);

    world.update_cell(0, 0, true);
    world.update_cell(1, 0, true);

    println!("\nPrint World {}\n", world);
    assert_eq!(true, world.next_state_of_cell(1, 1));
}

#[test]
fn living_cell_with_3_neighbours_will_live() {
    let mut world = World::new(3, 3);

    world.update_cell(1, 1, true);

    world.update_cell(0, 0, true);
    world.update_cell(1, 0, true);
    world.update_cell(2, 0, true);

    println!("\n\nPrint World {}\n", world);
    assert_eq!(true, world.next_state_of_cell(1, 1));
}

#[test]
fn living_cell_with_more_than_3_neighbours_will_die() {
    let mut world = World::new(3, 3);

    world.update_cell(1, 1, true);

    world.update_cell(0, 0, true);
    world.update_cell(1, 0, true);
    world.update_cell(2, 0, true);
    world.update_cell(0, 1, true);

    println!("\nPrint World {}\n", world);
    assert_eq!(false, world.next_state_of_cell(1, 1));
}

#[test]
fn dead_cell_with_3_neighbours_will_live() {
    let mut world = World::new(3, 3);

    world.update_cell(1, 1, false);

    world.update_cell(0, 0, true);
    world.update_cell(1, 0, true);
    world.update_cell(2, 0, true);

    println!("\nPrint World {}\n", world);
    assert_eq!(true, world.next_state_of_cell(1, 1));
}

#[test]
fn dead_cell_becomes_living_cell() {
    let mut world = World::new(3, 3);

    world.update_cell(1, 1, false);

    world.update_cell(0, 0, true);
    world.update_cell(1, 0, true);
    world.update_cell(2, 0, true);

    let new_world = world.next_state();

    println!("\nPrint World {}\n", world);
    println!("\nPrint World {}\n", new_world);
    assert_eq!(true, new_world.is_alive(1, 1));
}

#[test]
fn living_cell_becomes_dead_cell() {
    let mut world = World::new(3, 3);

    world.update_cell(1, 1, true);

    world.update_cell(0, 0, true);
    world.update_cell(1, 0, true);
    world.update_cell(2, 0, true);
    world.update_cell(0, 1, true);

    let new_world = world.next_state();

    println!("\nPrint World {}\n", world);
    println!("\nPrint World {}\n", new_world);
    assert_eq!(false, new_world.is_alive(1, 1));
}

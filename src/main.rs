extern crate piston_window;

use piston_window::*;
// use std::thread;

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

    let mut world = World::new(rows, columns);

    world.update_cell(10, 10, true);

    while let Some(event) = window.next() {
        // println!("render...");
        // thread::sleep(std::time::Duration::new(1, 0));

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

    fn for_each_cell<F>(&self, mut func: F)
    where
        F: FnMut(u32, u32),
    {
        let rows = self.rows;
        let columns = self.columns;
        for i in 0..rows - 1 {
            for j in 0..columns - 1 {
                func(i, j);
            }
        }
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

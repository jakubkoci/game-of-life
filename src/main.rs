extern crate piston_window;

use piston_window::*;

fn main() {
    let width = 640;
    let height = 480;

    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [width, height])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);

            let cell_size = 20;
            let rows = height / cell_size;
            let columns = width / cell_size;

            let live = [0.0, 0.0, 0.0, 1.0];
            let dead = [1.0, 1.0, 1.0, 1.0];

            let mut world = vec![vec![false; columns as usize]; rows as usize];

            world[10][10] = true;

            for i in 0..rows - 1 {
                for j in 0..columns - 1 {
                    let is_alive = world[i as usize][j as usize];

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
                }
            }
        });
    }
}

// struct World {
//     rows: u32,
//     columns: u32,
//     cells: Vec<Vec<bool>>,
// }

// impl World {
//     fn new(rows: u32, columns: u32) -> World {
//         World {
//             rows,
//             columns,
//             cells: vec![vec![false; columns as usize]; rows as usize],
//         }
//     }

//     fn updateState(&self, row: u32, column: u32, value: bool) {
//         &self.cells[row as usize][column as usize] = value;
//     }
// }

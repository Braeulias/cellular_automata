use macroquad::miniquad::KeyCode::W;
use macroquad::prelude::*;


const WIDTH: usize = 100;
const HEIGHT: usize = 100;
const CELL_SIZE: f32 = 5.0;


struct Grid {
    cells: [[bool; WIDTH]; HEIGHT]
}

impl Grid {
    fn new() -> Self {
        Grid {
            cells: [[false; WIDTH]; HEIGHT]
        }
    }

    fn draw(&self) {
        let window_width = screen_width();
        let window_height = screen_height();

        // Calculate the grid's position to center it
        let grid_width = WIDTH as f32 * CELL_SIZE;
        let grid_height = HEIGHT as f32 * CELL_SIZE;
        let x_offset = (window_width - grid_width) / 2.0;
        let y_offset = (window_height - grid_height) / 2.0;

        // Draw each cell
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let cell_color = if self.cells[y][x] {
                    BLACK
                } else {
                    WHITE
                };

                // Draw the cell with the offset applied
                draw_rectangle(
                    x as f32 * CELL_SIZE + x_offset,
                    y as f32 * CELL_SIZE + y_offset,
                    CELL_SIZE,
                    CELL_SIZE,
                    cell_color,
                );
            }
        }

        // Draw the outer border around the entire grid
        let border_thickness = 2.0;
        let border_color = GRAY;

        draw_rectangle_lines(
            x_offset,
            y_offset,
            grid_width,
            grid_height,
            border_thickness,
            border_color,
        );

    }

    fn toggle_cell(&mut self, x: usize,y: usize) {
        self.cells[y][x] = true;
    }

    fn update(&mut self) {
        let mut new_cells = [[false; WIDTH]; HEIGHT];

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let live_neighbours = self.count_live_neighbours(x,y);

                new_cells[y][x] = match (self.cells[y][x], live_neighbours) {
                    (true, 2) | (_, 3) => true,
                    _ => false,
                }
            }
        }

        self.cells = new_cells;

    }

    fn count_live_neighbours(&self, x: usize, y: usize) -> usize {
        let mut count = 0;

        for dy in [-1, 0, 1].iter().cloned() {
            for dx in [-1, 0, 1].iter().cloned() {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = (x as isize + dx).rem_euclid(WIDTH as isize) as usize;
                let ny = (y as isize + dy).rem_euclid(WIDTH as isize) as usize;

                if self.cells[ny][nx] {
                    count += 1;
                }
            }
        }

        count
    }

    fn clear(&mut self) {
        self.cells = [[false; WIDTH]; HEIGHT];
    }



    fn handle_input(&mut self) {

        if is_key_pressed(KeyCode::R) {
            let num_Cell_to_fill = 100;

        }

        if is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();

            // Calculate the grid's position to center it
            let window_width = screen_width();
            let window_height = screen_height();
            let grid_width = WIDTH as f32 * CELL_SIZE;
            let grid_height = HEIGHT as f32 * CELL_SIZE;
            let x_offset = (window_width - grid_width) / 2.0;
            let y_offset = (window_height - grid_height) / 2.0;

            // Convert mouse position to grid coordinates
            let grid_x = ((mouse_x - x_offset) / CELL_SIZE) as usize;
            let grid_y = ((mouse_y - y_offset) / CELL_SIZE) as usize;

            if grid_x < WIDTH && grid_y < HEIGHT {
                self.toggle_cell(grid_x, grid_y);
            }
        }



        if is_key_pressed(KeyCode::C) {
            self.clear();
        }


    }

}




#[macroquad::main("Cellular Automata")]
async fn main() {


    let mut grid = Grid::new();
    let mut running = false;

    loop {
        clear_background(WHITE);

        if is_key_pressed(KeyCode::Space) {
            running = !running;
        }

        if running {
            grid.update()
        }

        grid.handle_input();
        grid.draw();

        next_frame().await;
    }
}

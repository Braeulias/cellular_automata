use macroquad::prelude::*;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;
const CELL_SIZE: f32 = 5.0;

struct Dropdown {
    is_open: bool,
    selected_index: usize,
    options: Vec<&'static str>,
    position: Vec2,
    size: Vec2,
}

impl Dropdown {
    fn new(position: Vec2, size: Vec2, options: Vec<&'static str>) -> Self {
        Dropdown {
            is_open: false,
            selected_index: 0,
            options,
            position,
            size,
        }
    }

    fn draw(&mut self) {
        // Draw the dropdown button
        let button_color = if self.is_open { LIGHTGRAY } else { GRAY };
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.size.x,
            self.size.y,
            button_color,
        );

        let selected_option = self.options[self.selected_index];
        let text_dimensions = measure_text(selected_option, None, 20, 1.0);
        draw_text(
            selected_option,
            self.position.x + (self.size.x - text_dimensions.width) / 2.0,
            self.position.y + (self.size.y + text_dimensions.height) / 2.0 - 5.0,
            20.0,
            BLACK,
        );

        if self.is_open {
            // Draw dropdown options
            let option_height = 30.0;
            let options_height = option_height * self.options.len() as f32;
            for (i, option) in self.options.iter().enumerate() {
                let y = self.position.y + self.size.y + (i as f32 * option_height);
                draw_rectangle(self.position.x, y, self.size.x, option_height, LIGHTGRAY);
                draw_text(
                    option,
                    self.position.x + 10.0,
                    y + (option_height / 2.0) + 5.0,
                    20.0,
                    BLACK,
                );

                if is_mouse_button_pressed(MouseButton::Left) {
                    let (mouse_x, mouse_y) = mouse_position();
                    if mouse_x >= self.position.x
                        && mouse_x <= self.position.x + self.size.x
                        && mouse_y >= y
                        && mouse_y <= y + option_height
                    {
                        self.selected_index = i;
                        self.is_open = false;
                    }
                }
            }

            // Draw a background rectangle to cover the area below the dropdown when open
            draw_rectangle(
                self.position.x,
                self.position.y + self.size.y,
                self.size.x,
                options_height,
                Color::from_rgba(0, 0, 0, 128),
            );
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            if mouse_x >= self.position.x
                && mouse_x <= self.position.x + self.size.x
                && mouse_y >= self.position.y
                && mouse_y <= self.position.y + self.size.y
            {
                self.is_open = !self.is_open;
            } else if !self.is_open {
                self.is_open = false;
            }
        }
    }

    fn get_selected_option(&self) -> &str {
        self.options[self.selected_index]
    }
}

enum RuleSet {
    GameOfLife,
    HighLife,
    BriansBrain,
    Seeded,
    DayNight,
    MorleysGarden,
    Diffusion,
    SierpinskiTriangle,
    Custom(String),
}

impl RuleSet {
    fn update_grid(&self, grid: &mut Grid) {
        match self {
            RuleSet::GameOfLife => grid.update_conway(),
            RuleSet::HighLife => grid.update_highlife(),
            RuleSet::BriansBrain => grid.update_brians_brain(),
            RuleSet::Seeded => grid.update_seeded(),
            RuleSet::DayNight => grid.update_day_night(),
            RuleSet::MorleysGarden => grid.update_morleys_garden(),
            RuleSet::Diffusion => grid.update_diffusion(),
            RuleSet::SierpinskiTriangle => grid.update_sierpinski(),
            RuleSet::Custom(custom_rule) => grid.update_custom(custom_rule),
        }
    }
}

struct Grid {
    cells: [[bool; WIDTH]; HEIGHT],
}

impl Grid {
    fn new() -> Self {
        Grid {
            cells: [[false; WIDTH]; HEIGHT],
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
                let cell_color = if self.cells[y][x] { BLACK } else { WHITE };

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


    fn update_conway(&mut self) {
        let mut new_cells = [[false; WIDTH]; HEIGHT];

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let live_neighbours = self.count_live_neighbours(x, y);

                new_cells[y][x] = match (self.cells[y][x], live_neighbours) {
                    (true, 2) | (_, 3) => true,
                    _ => false,
                }
            }
        }

        self.cells = new_cells;
    }

    fn update_highlife(&mut self) {
        let mut new_cells = [[false; WIDTH]; HEIGHT];

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let live_neighbours = self.count_live_neighbours(x, y);

                new_cells[y][x] = match (self.cells[y][x], live_neighbours) {
                    (_, 3) | (_, 6) | (true, 2) => true,
                    _ => false,
                }
            }
        }

        self.cells = new_cells;
    }

    fn update_brians_brain(&mut self) {
        let mut new_cells = [[false; WIDTH]; HEIGHT];

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.cells[y][x] {
                    new_cells[y][x] = false;
                } else if self.count_live_neighbours(x, y) == 2 {
                    new_cells[y][x] = true;
                }
            }
        }

        self.cells = new_cells;
    }

    fn update_seeded(&mut self) {
        let mut new_cells = [[false; WIDTH]; HEIGHT];

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let live_neighbours = self.count_live_neighbours(x, y);

                new_cells[y][x] = match (self.cells[y][x], live_neighbours) {
                    (_, 3) => true,
                    _ => false,
                };
            }
        }

        self.cells = new_cells;
    }

    fn update_day_night(&mut self) {
        let mut new_cells = [[false; WIDTH]; HEIGHT];

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let live_neighbours = self.count_live_neighbours(x, y);

                new_cells[y][x] = match (self.cells[y][x], live_neighbours) {
                    (_, 3) | (_, 6) | (_, 7) | (_, 8) => true,
                    (true, 2) => true,
                    _ => false,
                };
            }
        }

        self.cells = new_cells;
    }

    fn update_morleys_garden(&mut self) {
        let mut new_cells = [[false; WIDTH]; HEIGHT];

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let live_neighbours = self.count_live_neighbours(x, y);

                new_cells[y][x] = match (self.cells[y][x], live_neighbours) {
                    (_, 3) | (_, 6) => true,
                    _ => false,
                };
            }
        }

        self.cells = new_cells;
    }

    fn update_diffusion(&mut self) {
        let mut new_cells = [[false; WIDTH]; HEIGHT];

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let mut live_neighbors = 0;

                for dy in [-1, 0, 1].iter().cloned() {
                    for dx in [-1, 0, 1].iter().cloned() {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        let nx = (x as isize + dx).rem_euclid(WIDTH as isize) as usize;
                        let ny = (y as isize + dy).rem_euclid(HEIGHT as isize) as usize;

                        if self.cells[ny][nx] {
                            live_neighbors += 1;
                        }
                    }
                }

                new_cells[y][x] = live_neighbors >= 2;
            }
        }

        self.cells = new_cells;
    }

    fn update_sierpinski(&mut self) {
        let mut new_cells = [[false; WIDTH]; HEIGHT];

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if (x & y) == 0 {
                    new_cells[y][x] = true;
                }
            }
        }

        self.cells = new_cells;
    }

    fn update_custom(&mut self, custom_rule: &String) {
        // Implement custom rule logic here
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

    fn random_fill(&mut self, fill_percentage: f32) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.cells[y][x] = rand::gen_range(0.0, 1.0) < fill_percentage;
            }
        }
    }

    fn handle_input(&mut self) {
        static mut DRAW_MODE: bool = true; // true for drawing, false for erasing

        if is_key_pressed(KeyCode::R) {
            let fill_percentage = 0.4; // 40% fill
            self.random_fill(fill_percentage);
        }

        if is_key_pressed(KeyCode::D) {
            unsafe {
                DRAW_MODE = !DRAW_MODE;
            }
        }

        if is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();

            let window_width = screen_width();
            let window_height = screen_height();
            let grid_width = WIDTH as f32 * CELL_SIZE;
            let grid_height = HEIGHT as f32 * CELL_SIZE;
            let x_offset = (window_width - grid_width) / 2.0;
            let y_offset = (window_height - grid_height) / 2.0;

            let grid_x = ((mouse_x - x_offset) / CELL_SIZE) as usize;
            let grid_y = ((mouse_y - y_offset) / CELL_SIZE) as usize;

            if grid_x < WIDTH && grid_y < HEIGHT {
                unsafe {
                    self.cells[grid_y][grid_x] = DRAW_MODE;
                }
            }
        }

        if is_key_pressed(KeyCode::C) {
            self.clear();
        }
    }
}

fn draw_keybinds() {
    let x_position = 10.0;
    let y_position_start = 10.0;
    let line_spacing = 20.0;

    let keybinds = [
        "Space: Start/Pause simulation",
        "S: Step forward one generation",
        "R: Randomly fill grid",
        "C: Clear grid",
        "D: Toggle draw/erase mode",
        "Mouse Click: Toggle cell state",
    ];

    for (i, &keybind) in keybinds.iter().enumerate() {
        draw_text(
            keybind,
            x_position,
            y_position_start + i as f32 * line_spacing,
            20.0,
            BLACK,
        );
    }
}

#[macroquad::main("Cellular Automata")]
async fn main() {
    let mut grid = Grid::new();
    let mut running = false;
    let mut step_forward = false;
    let speed = 50.0; // Customize speed

    // Calculate the grid's position to center it
    let window_width = screen_width();
    let window_height = screen_height();
    let grid_width = WIDTH as f32 * CELL_SIZE;
    let grid_height = HEIGHT as f32 * CELL_SIZE;
    let x_offset = (window_width - grid_width) / 2.0;
    let y_offset = (window_height - grid_height) / 2.0;

    let mut dropdown = Dropdown::new(
        Vec2::new(x_offset + (grid_width - 200.0) / 2.0, y_offset - 40.0),
        Vec2::new(200.0, 30.0),
        vec![
            "Game of Life",
            "HighLife",
            "Brian's Brain",
            "Seeded",
            "Day & Night",
            "Morley's Garden",
            "Diffusion",
            "Sierpinski Triangle",
        ],
    );

    let mut selected_ruleset = RuleSet::GameOfLife;

    loop {
        clear_background(WHITE);

        if is_key_pressed(KeyCode::Space) {
            running = !running;
        }

        if is_key_pressed(KeyCode::S) {
            step_forward = true;
        }

        if running || step_forward {
            selected_ruleset.update_grid(&mut grid);
            step_forward = false;
            std::thread::sleep(std::time::Duration::from_secs_f32(1.0 / speed));
        }

        grid.handle_input();
        grid.draw();

        dropdown.draw();

        match dropdown.get_selected_option() {
            "Game of Life" => selected_ruleset = RuleSet::GameOfLife,
            "HighLife" => selected_ruleset = RuleSet::HighLife,
            "Brian's Brain" => selected_ruleset = RuleSet::BriansBrain,
            "Seeded" => selected_ruleset = RuleSet::Seeded,
            "Day & Night" => selected_ruleset = RuleSet::DayNight,
            "Morley's Garden" => selected_ruleset = RuleSet::MorleysGarden,
            "Diffusion" => selected_ruleset = RuleSet::Diffusion,
            "Sierpinski Triangle" => selected_ruleset = RuleSet::SierpinskiTriangle,
            _ => {}
        }

        draw_keybinds();

        next_frame().await;
    }
}

use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use std::time::Duration;

mod framebuffer;
use framebuffer::Framebuffer;

const WIDTH: usize = 200; // Ancho de la cuadrícula
const HEIGHT: usize = 200; // Altura de la cuadrícula para mostrar la evolución
const WINDOW_WIDTH: usize = 800; // Ancho de la ventana
const WINDOW_HEIGHT: usize = 600; // Altura de la ventana

// Define los colores de inicio y fin para el gradiente blanco a negro
const COLOR_START: (u8, u8, u8) = (255, 255, 255); // Blanco
const COLOR_END: (u8, u8, u8) = (0, 0, 0); // Negro

// Interpola entre dos colores basados en una proporción
fn interpolate_color(start: (u8, u8, u8), end: (u8, u8, u8), t: f32) -> u32 {
    let r = start.0 as f32 + t * (end.0 as f32 - start.0 as f32);
    let g = start.1 as f32 + t * (end.1 as f32 - start.1 as f32);
    let b = start.2 as f32 + t * (end.2 as f32 - start.2 as f32);
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

// Regla 30: Calcula el próximo estado de una célula basado en sus vecinos
fn rule_30(left: usize, center: usize, right: usize) -> usize {
    match (left, center, right) {
        (1, 1, 1) => 0,
        (1, 1, 0) => 0,
        (1, 0, 1) => 0,
        (1, 0, 0) => 1,
        (0, 1, 1) => 1,
        (0, 1, 0) => 1,
        (0, 0, 1) => 1,
        (0, 0, 0) => 0,
        _ => 0,
    }
}

// Inicializa la cuadrícula con una sola célula activa en el centro
fn initialize_grid() -> Vec<Vec<usize>> {
    let mut grid = vec![vec![0; WIDTH]; HEIGHT];
    grid[0][WIDTH / 2] = 1; // Activa la célula central en la primera fila
    grid
}

// Actualiza la cuadrícula aplicando la Regla 30 con "loop" en los bordes
fn update_grid(grid: &mut Vec<Vec<usize>>, colors: &mut Vec<Vec<u32>>, current_row: usize) {
    for x in 0..WIDTH {
        let left = if x == 0 { grid[current_row][WIDTH - 1] } else { grid[current_row][x - 1] };
        let center = grid[current_row][x];
        let right = if x == WIDTH - 1 { grid[current_row][0] } else { grid[current_row][x + 1] };

        let new_state = rule_30(left, center, right);
        if current_row + 1 < HEIGHT {
            grid[current_row + 1][x] = new_state;
            if new_state == 1 {
                let t = (current_row + 1) as f32 / HEIGHT as f32;
                colors[current_row + 1][x] = interpolate_color(COLOR_START, COLOR_END, t);
            }
        }
    }
}

// Renderiza la cuadrícula en el framebuffer
fn render(framebuffer: &mut Framebuffer, grid: &Vec<Vec<usize>>, colors: &Vec<Vec<u32>>) {
    framebuffer.clear();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let color = if grid[y][x] == 1 { colors[y][x] } else { 0x000000 };
            framebuffer.point(x, y, color);
        }
    }
}

// Aplicar reglas del Juego de la Vida de Conway
fn conway_rules(grid: &Vec<Vec<usize>>, new_grid: &mut Vec<Vec<usize>>, colors: &mut Vec<Vec<u32>>) {
    let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let mut live_neighbors = 0;
            for &(dx, dy) in &directions {
                let nx = (x as isize + dx).rem_euclid(WIDTH as isize) as usize;
                let ny = (y as isize + dy).rem_euclid(HEIGHT as isize) as usize;
                if grid[ny][nx] == 1 {
                    live_neighbors += 1;
                }
            }

            new_grid[y][x] = match (grid[y][x], live_neighbors) {
                (1, 2) | (1, 3) => 1,
                (0, 3) => {
                    colors[y][x] = 0xFFFFFF; // Blanco para las nuevas células vivas
                    1
                },
                _ => 0,
            };
        }
    }
}

// Generar colores brillantes aleatorios
fn generate_random_color(rng: &mut rand::rngs::ThreadRng) -> u32 {
    let r = rng.gen_range(128..256) as u32;
    let g = rng.gen_range(128..256) as u32;
    let b = rng.gen_range(128..256) as u32;
    (r << 16) | (g << 8) | b
}

fn insert_pattern(grid: &mut Vec<Vec<usize>>, colors: &mut Vec<Vec<u32>>, pattern: &[(usize, usize)], offset_x: usize, offset_y: usize) {
    let mut rng = rand::thread_rng();
    let color = generate_random_color(&mut rng);
    for &(x, y) in pattern.iter() {
        let nx = (offset_x + x) % WIDTH;
        let ny = (offset_y + y) % HEIGHT;
        grid[ny][nx] = 1;
        colors[ny][nx] = color;
    }
}

// Patrón Pulsar (Periodo 3)
const PULSAR: [(usize, usize); 48] = [
    (2, 4), (2, 5), (2, 6), (2, 10), (2, 11), (2, 12),
    (4, 2), (5, 2), (6, 2), (4, 7), (5, 7), (6, 7),
    (4, 9), (5, 9), (6, 9), (4, 14), (5, 14), (6, 14),
    (7, 4), (7, 5), (7, 6), (7, 10), (7, 11), (7, 12),
    (9, 4), (9, 5), (9, 6), (9, 10), (9, 11), (9, 12),
    (10, 2), (11, 2), (12, 2), (10, 7), (11, 7), (12, 7),
    (10, 9), (11, 9), (12, 9), (10, 14), (11, 14), (12, 14),
    (14, 4), (14, 5), (14, 6), (14, 10), (14, 11), (14, 12),
];

fn main() {
    let frame_delay = Duration::from_millis(100);

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let mut window = Window::new(
        "Wolfram's Rule 30 with Conway's Game of Life",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    ).unwrap();

    let mut grid = initialize_grid();
    let mut colors = vec![vec![0x000000; WIDTH]; HEIGHT];
    let mut current_row = 0;

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        if current_row + 1 < HEIGHT {
            update_grid(&mut grid, &mut colors, current_row);
            current_row += 1;
        } else {
            let mut new_grid = grid.clone();
            conway_rules(&grid, &mut new_grid, &mut colors);
            grid = new_grid;

            // Insertar patrón Pulsar en lugares aleatorios donde haya espacio muerto
            let mut rng = rand::thread_rng();
            for _ in 0..5 {
                let offset_x = rng.gen_range(0..WIDTH);
                let offset_y = rng.gen_range(0..HEIGHT);
                insert_pattern(&mut grid, &mut colors, &PULSAR, offset_x, offset_y);
            }
        }

        render(&mut framebuffer, &grid, &colors);

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}

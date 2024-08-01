# Conway's Game of Life with Wolfram's Rule 30
#### Rodrigo Mansilla Dubon
#### 22611
#### Universidad del Valle de Guatemala

This project implements Conway's Game of Life combined with Wolfram's Rule 30. The cells initially follow Rule 30 and display a gradient from white to black. When the cells transition to following Conway's Game of Life, they exhibit an explosion of colors in the form of pulsar patterns and cellular growth.

## Features

- **Rule 30**: Initial generation follows Wolfram's Rule 30 with a gradient from white to black.
- **Conway's Game of Life**: Once Rule 30 is complete, the cells transition to following Conway's Game of Life rules.
- **Color Explosion**: Pulsar patterns are inserted randomly in the grid with bright, random colors.
- **Real-time Rendering**: The grid is rendered in real-time using the `minifb` library.

## Demo
![giphy-_1_](https://github.com/user-attachments/assets/5969b9af-f0b8-4ec3-b85a-2c2e4e39162b)



## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) installed on your machine.

### Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/your_username/your_repository.git
    cd your_repository
    ```

2. Run the project:
    ```sh
    cargo run
    ```

## Code Overview

### Main Components

- **Rule 30**: The initial grid generation follows Wolfram's Rule 30, creating a gradient effect.
- **Conway's Game of Life**: After completing Rule 30 generation, the grid transitions to Conway's Game of Life rules.
- **Color Interpolation**: Generates a gradient from white to black during Rule 30 and random bright colors for pulsar patterns.
- **Real-time Rendering**: The grid is updated and rendered in real-time using `minifb`.

### Key Functions

- `interpolate_color`: Interpolates between two colors based on a given proportion.
- `rule_30`: Computes the next state of a cell based on its neighbors using Rule 30.
- `initialize_grid`: Initializes the grid with a single active cell in the center.
- `update_grid`: Updates the grid following Rule 30 and assigns colors.
- `render`: Renders the grid onto the framebuffer.
- `conway_rules`: Applies Conway's Game of Life rules to the grid.
- `generate_random_color`: Generates a random bright color.
- `insert_pattern`: Inserts a pulsar pattern with a given color.



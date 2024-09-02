# Cellular Automata Simulation

This is a simple interactive cellular automata simulation built using the [Macroquad](https://github.com/not-fl3/macroquad) game development framework in Rust. The simulation allows you to explore various cellular automata rule sets and visualize their behavior on a grid. The project includes a customizable dropdown menu for selecting different rule sets, and supports user interaction for modifying the grid.

## Features

- **Multiple Cellular Automata Rules:** Includes popular rules like Game of Life, HighLife, Brian's Brain, and others.
- **Interactive Grid:** Draw, erase, and clear cells directly on the grid.
- **Control Simulation:** Start, pause, or step through the simulation.
- **Customizable Speed:** Adjust the simulation speed.

## Rule Sets

The simulation includes the following rule sets:

- **Game of Life:** Classic Conway's Game of Life.
- **HighLife:** A variant of Game of Life with additional rules.
- **Brian's Brain:** A cyclic cellular automaton.
- **Seeded:** A variation where cells come to life with exactly three neighbors.
- **Day & Night:** A rule that supports survival patterns.
- **Morley's Garden:** A chaotic rule set.
- **Diffusion:** A rule set based on diffusion.
- **Sierpinski Triangle:** Generates the Sierpinski triangle pattern.

## Keybindings

- **Space:** Start/Pause the simulation.
- **S:** Step forward one generation.
- **R:** Randomly fill the grid.
- **C:** Clear the grid.
- **D:** Toggle draw/erase mode.
- **Mouse Click:** Toggle the state of individual cells.

## How to Run

1. **Clone the repository:**
   ```sh
   git clone https://github.com/Braeulias/cellular_automata.git
   cd cellular-automata
   ```

2. **Build and run the project:**
   Ensure you have Rust and Cargo installed. Then, run:
   ```sh
   cargo run
   ```

3. **Interact with the simulation:**
   - Use the dropdown menu to select different rule sets.
   - Use the keybindings to control the simulation and interact with the grid.

## Future Enhancements

- Implement custom rule logic through user input.
- Add more rule sets and visualizations.
- Enhance the UI with more interactive controls and visual feedback.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## Acknowledgments

- Thanks to the [Macroquad](https://github.com/not-fl3/macroquad) community for the awesome game development framework.

---

Enjoy experimenting with cellular automata!

# Recipe Manager

A simple, GUI-based recipe management application built with Rust and Iced.

## Features

- Add, edit, and delete recipes
- View a list of all recipes
- Save and load recipes from a file
- User-friendly graphical interface

## Prerequisites

Before you begin, ensure you have met the following requirements:

- Rust programming language (latest stable version)
- Cargo package manager

You can install Rust and Cargo by following the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

## Installation

1. Clone this repository:
   ```
   git clone https://github.com/yourusername/recipe-manager.git
   cd recipe-manager
   ```

2. Build the project:
   ```
   cargo build --release
   ```

## Usage

To run the Recipe Manager:

```
cargo run --release
```

### Adding a Recipe

1. Enter the recipe name, ingredients (comma-separated), instructions (line-separated), and number of servings in the input fields.
2. Click the "Add Recipe" button.

### Editing a Recipe

1. Click the "Edit" button next to the recipe you want to modify.
2. Update the recipe details in the input fields.
3. Click the "Update Recipe" button to save your changes.

### Deleting a Recipe

Click the "Delete" button next to the recipe you want to remove.

### Saving Recipes

Click the "Save Recipes" button to save all recipes to a file named `recipes.json`.

### Loading Recipes

Click the "Load Recipes" button to load recipes from the `recipes.json` file.

## Contributing

Contributions to the Recipe Manager are welcome. Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## Acknowledgments

- [Rust Programming Language](https://www.rust-lang.org/)
- [Iced GUI Library](https://github.com/iced-rs/iced)

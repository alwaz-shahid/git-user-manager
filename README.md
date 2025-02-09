
---

![Git User Manager Logo](https://github.com/user-attachments/assets/a4609709-3db5-42a6-976b-bdea333fd502)

<hr/>

# Git User Manager CLI Tool

## Overview

Git User Manager is a command-line interface (CLI) tool designed to simplify managing Git user configurations on your local machine. It provides an interactive menu for listing users, displaying the current user, adding a new user, and changing the active user seamlessly.

## Features

- **List All Configured Git Users**: View all users configured on your machine.
- **Display Currently Active Git User**: Quickly check who the current active Git user is.
- **Add New User**: Add a new user to the global Git configuration.
- **Switch Users**: Switch to a different user from the list of configured users.

## Usage

This tool runs interactively without requiring command-line arguments. Simply follow the prompts in the menu to perform actions.

### Getting Started

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/alwaz-shahid/git-user-manager.git
   cd git-user-manager
   ```

2. **Build and Run**:
   ```bash
   cargo build --release
   cargo run --release
   ```

3. **Download from Releases**:
   Alternatively, you can download the pre-built binaries from the [releases page](https://github.com/alwaz-shahid/git-user-manager/releases).

## Features to Add

- [ ] Add current user to JSON file if exists.
- [ ] Handle duplicate entries.
- [ ] Create a GUI version.
- [ ] Explicitly handle "You don't have permissions to push username/repository" issue on Windows.
- [ ] Improve README documentation.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any questions or feedback, feel free to open an issue or contact the maintainer directly.

---

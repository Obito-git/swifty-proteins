# Swifty Proteins

**Swifty Proteins** is a project focused on protein visualization and account management. The project utilizes various technologies and tools to achieve its goals.

## Getting Started

To start working with Swifty Proteins, you need to set up the necessary components:

- **Retrieve PDB Files**: The project must use the RCSB website to retrieve .pdb files. You can access the website at [RCSB](https://www.rcsb.org). To download a specific .pdb file, use the link format: `https://files.rcsb.org/download/{PDB_ID}.pdb`.

- **Rust Crate for 3D Object Generation**: For generating 3D objects, you can use the `gltf` Rust crate. You can find the crate [here](https://crates.io/crates/gltf).

- **Rust Backend**: The backend of the project is built using [Rocket](https://rocket.rs/), a web framework for Rust.

- **Database Integration**: [Diesel](https://diesel.rs/guides/getting-started) is used for database operations. Install the Diesel CLI with the following command:
  ```sh
  cargo install diesel_cli --no-default-features --features postgres
  ```
  Start migrations using:
  ```sh
  diesel migration run
  ```

- **Flutter Library for 3D Object Rendering**: To render 3D objects in Flutter, you can utilize the [flutter_3d](https://fluttergems.dev/3d/) library.

### General Tasks
- [ ] Add an icon.
- [ ] Implement a launch screen that appears for a few seconds.

### Login View
- [ ] Utilize Firebase for account management.
- [ ] Implement authentication screen with options for fingerprint and password.
- [ ] Display a popup when authentication fails.
- [ ] Always show the authentication screen.

### Protein List View
- [ ] Add all proteins from ligands.txt (use enum or file as a resource).
- [ ] Implement a list view with a search bar.
- [ ] Create a service to retrieve .pdb files from the RCSB website.
- [ ] Display a popup on error.
- [ ] Implement animation for loading.

### Protein View
- [ ] Implement functionality. (TODO)

Feel free to mark tasks as completed as you progress with the project. Good luck!

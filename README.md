# Swifty Proteins

**Swifty Proteins** is a project focused on protein visualization and account management. The project utilizes various technologies and tools to achieve its goals.

backend:
- api: rocket
- orm: diesel
- database: SQLite

frontend:
- flutter

## Getting Started

To start working with Swifty Proteins, we need to set up the necessary components:

- **Retrieve PDB Files**: The project must use the RCSB website to retrieve .pdb files. You can access the website at [RCSB](https://www.rcsb.org). To download a specific .pdb file, use the link format: `https://files.rcsb.org/download/{PDB_ID}.pdb`.

notice:
- [ ] The url for proteins isn't correct need to find the right one

- **Rust Crate for 3D Object Generation**: For generating 3D objects, we can use the `gltf` Rust crate. The crate [here](https://crates.io/crates/gltf).

- **Rust Backend**: The backend of the project is built using [Rocket](https://rocket.rs/), a web framework for Rust.

- **Database Integration**: [Diesel](https://diesel.rs/guides/getting-started) is used for database operations. To put in docker:

Install the Diesel CLI with the following command:
  ```sh
  export DATABASE_URL=postgres://username:password@localhost/diesel_demo


  cargo install diesel_cli --no-default-features --features postgres
  diesel migration run
  ```

- **Flutter Library for 3D Object Rendering**: To render 3D objects in Flutter, we can utilize the [flutter_3d](https://fluttergems.dev/3d/) library.

### General Tasks
- [x] Define different routes
- [x] Add an icon.
- [x] Implement a launch screen that appears for a few seconds. (It is default feature of flutter)

### Login View
- [x] Login screen with username and password fields.
- [x] Implement a button to switch between login and signup.
- [x] Signup screen with username and password fields.
- [x] Basic non-empty validation for username and password.
- [ ] handle finger print authentication
- [x] Display a popup when authentication fails (displayed when API fails).
- [ ] Always show the authentication screen.

### Protein List View
- [x] Add all proteins from ligands.txt (use enum or file as a resource).
- [x] Implement a list view with a search bar.
- [ ] Display a popup on error.
- [x] Implement animation for loading.

### Protein View
- [ ] Implement functionality. (mostly ok)

### Backend

#### Auth 

- [x] Implement signin and signup endpoints
- [x] Password hashing
- [x] Login and password validation

##### Sign up
- [x] Not json body (should return 400/404)
- [x] Invalid json body (should return 422)
- [x] Non alphanumeric username (should return 400)
- [x] Non alphanumeric password (should return 400)
- [x] Short/long username (should return 400)
- [x] Short/long password (should return 400)
- [x] Username already exists (should return 409)

#### Proteins
- [x] API with list of all avalaible proteins
- [x] Mock of endpoint to retrieve 3d object
- [x] Provide possibility to store .glb files locally
- [x] Store .glb file metadata in the database
- [ ] make protein endpoints secured
- [ ] Proxy that connects previous API and PDB APIs (or wiki if possible)
- [ ] Endpoints to retrieve atom or protein info, like wiki
- [ ] Actual implementation of mapping from .pdb to 3d object
- [ ] Mechanism that saves wiki info and 3d object to the database

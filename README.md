# swifty_proteins

swifty_proteins

## Getting Started


This project must use the RCSB website to retrieve .pdb files.

Website:                https://www.rcsb.org
Link to retrieve .pdb:  https://files.rcsb.org/download/{PDB_ID}.pdb

Rust crate for generate 3d obj:
https://crates.io/crates/gltf

Rust back:
https://rocket.rs/

Flutter lib to render 3d obj:
https://fluttergems.dev/3d/

General:
[ ] Icon
[ ] Launch screen (A view appeared for few sec)

Login view:
[ ] Firebase for account managing
[ ] Auth screen (fingerprint, password)
[ ] Popup when auth failed
[ ] Always show auth screen

Protein list view:
[ ] Add all proteins from ligands.txt (enum/use file as resource)
[ ] Make it list and implement search bar
[ ] Implement service that should retrieve .pdb from rcsb website
[ ] Popup on error
[ ] Animation on load

Protein view:
[ ] 
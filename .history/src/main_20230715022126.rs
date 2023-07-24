// --------------------------------------------------------------------
// Project: blake3 hasher
// Creation date: 2023/07/04
// Author: Makertronic 
// web: http://www.makertronic-yt.com
// --------------------------------------------------------------------

use std::env;
use std::fs::File;
use std::fs;
use std::io::{self, Read};
use blake3::Hasher;
use hex;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Veuillez spécifier un nom de répertoire.");
        std::process::exit(1);
    }

    let path = &args[1];
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let mut file = File::open(&path)?;
            let mut buf = Vec::new();
            file.read_to_end(&mut buf)?;

            // Premier hashage
            let mut hasher1 = Hasher::new();
            hasher1.update(&buf);
            let hash1 = hasher1.finalize();
            println!("Le premier hash Blake3 du fichier {} est : {}", path.display(), hex::encode(hash1.as_bytes()));

            // Deuxième hashage sur le premier hash
            let mut hasher2 = Hasher::new();
            hasher2.update(hash1.as_bytes());
            let hash2 = hasher2.finalize();
            println!("Le deuxième hash Blake3 du fichier {} est : {}", path.display(), hex::encode(hash2.as_bytes()));
        }
    }

    Ok(())
}

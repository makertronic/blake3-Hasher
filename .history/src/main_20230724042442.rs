// --------------------------------------------------------------------
// Project: blake3 double hasher
// Creation date: 2023/07/04
// Author: Makertronic 
// web: http://www.makertronic-yt.com
// --------------------------------------------------------------------

use std::env;
use std::fs::File;
use std::fs;
use std::io::{self, BufReader};
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
            let file = File::open(&path)?;
            let mut reader = BufReader::new(file);

            let mut hasher = Hasher::new();
            io::copy(&mut reader, &mut hasher)?;
            let hash = hasher.finalize();
            println!("Le hash Blake3 du fichier {} est : {}", path.display(), hex::encode(hash.as_bytes()));
        }
    }

    Ok(())
}

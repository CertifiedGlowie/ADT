use aes::Aes256Dec;
use aes::cipher::{BlockDecrypt, KeyInit, generic_array::GenericArray};
use std::fs::File;
use std::io::prelude::*;
use structopt::StructOpt;
use hex::decode;
use std::path::Path;
use std::fs;

#[derive(StructOpt)]
struct Opt {
 /// Path to the file to decrypt
 #[structopt(short, long)]
 file: String,

 /// Key to decrypt with
 #[structopt(short, long)]
 key: String,
}

fn main() {
  let opt = Opt::from_args();
  if opt.file != "" && opt.key != "" {
    decrypt(&opt);
  } else {
    panic!("NO ARGUMENTS SPECIFIED!")
  }
}

fn decrypt(options: &Opt) {
  let input_key = decode(&options.key).unwrap();

  let input_path = Path::new(&options.file);
  let filename = input_path.file_name().unwrap();
  println!("DECRYPTING {:?} WITH THE KEY {}", filename, &options.key);
  let mut new_path = input_path.to_path_buf();
  new_path.set_extension("EX");
  println!("INPUT FILE SAVED AS {:?}", new_path);
  match fs::rename(input_path, &new_path) {
      Err(err) => println!("{}\n{:?}", new_path.display(), err),
      Ok(_) => {},
  }
  
  let mut file = File::open(new_path).unwrap();

  // Open the destination file for writing
  let mut dest = File::create(input_path).unwrap(); // Replace with your desired destination file path

  let key = GenericArray::clone_from_slice(&input_key);

  // Initialize cipher
  let cipher = Aes256Dec::new(&key);

  // Buffer for storing the file data
  let mut buffer = [0u8; 16];

  // Process the file in chunks of 16 bytes
  while let Ok(size) = file.read(&mut buffer) {
      if size == 0 {
          break;
      }

      // Create a mutable block from the buffer
      let mut block = GenericArray::from_mut_slice(&mut buffer[..size]);

      // Decrypt the block
      cipher.decrypt_block(&mut block);

      // Write the decrypted block to the destination file
      dest.write_all(&block[..size]).unwrap();
  }
}

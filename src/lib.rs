//  // from 'bs58' 
// // import * as prompt from 'prompt-sync 
// // use std::io::stdin;
// use std::io;

// #[test] 
// fn base58_to_wallet() { 
// println!("Enter your name:"); 
// let stdin = io::stdin(); 
// let base58 = stdin.lock().lines().next().unwrap().unwrap(); // gdtKSTXYULQNx87fdD3YgXkzVeyFeqwtxHm6WdEb5a9YJRnHse7GQr7t5pbepsyvUCk7VvksUGhPt4SZ8JHVSkt 
// let wallet = bs58::decode(base58).into_vec().unwrap(); println!("{:?}", wallet); 
// } 
// #[test] 
// fn wallet_to_base58() { 
// let wallet: Vec<u8> = 
// vec![34,46,55,124,141,190,24,204,134,91,70,184,161,181,44,122,15,172,63,62,153,150,99,255,202,89,105,77,41,89,253,130,27,195,134,14,66,75,242,7,132,234,160,203,109,195,116,251,144,44,28,56,231,114,50,131,185,168,138,61,35,98,78,53]; 
// let base58 = bs58::encode(wallet).into_string(); 
// println!("{:?}", base58); 
// } 

use std::io::{self, BufRead}; // Import proper I/O utilities
 // Base58 library for encoding and decoding

// Function to convert Base58 string to wallet (byte array)
#[test]
fn base58_to_wallet() {
    // Prompt user for Base58 input
    println!("Enter your Base58 encoded wallet:");

    let stdin = io::stdin();
    let base58 = stdin.lock().lines().next().unwrap().unwrap(); // Read input from the user

    // Decode Base58 string to a byte vector (wallet)
    match bs58::decode(base58).into_vec() {
        Ok(wallet) => println!("Decoded wallet (byte array): {:?}", wallet),
        Err(e) => println!("Failed to decode Base58: {:?}", e), // Handle decoding error
    }
}

// Function to convert a wallet (byte array) to Base58 string
#[test]
fn wallet_to_base58() {
    // Example byte vector (wallet)
    let wallet: Vec<u8> = vec![
        
    ];

    // Encode the byte vector (wallet) to a Base58 string
    let base58 = bs58::encode(wallet).into_string();
    println!("Encoded Base58 wallet: {:?}", base58);
}

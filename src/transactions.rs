use std::str::FromStr;

use secp256k1::{ecdsa::{Signature}, Message, PublicKey, Secp256k1, SecretKey};


// Key Requirements
/*
1. Only the right owner can spend a coin
2. The recipient can verify the legitimacy of the transaction
3. Once a transaction is written it cannot be altered this preventing fraud */
pub struct Transaction {
    sender: String,
    recipient: String,
    amount: f32,
    signature: String
}

impl Transaction {
    // TODO: Implement Result and Error Here
    // TODO: How often will this be used - could you make the Engine instance global
    fn try_sign(&mut self, secret_key: &SecretKey){
        // Create new engine instance
        let context = Secp256k1::new();
        // Create a byte array representing the message (vector in rust world)
        let message = self.create_message();
        // Sign ecdsa constructs a signature from a message, creates a digest from the message containing the sender, recipient and the amount using the secret key
        let signature = context.sign_ecdsa(&Message::from_slice(&message).expect("Could not parse message"), secret_key);
        // stores signature
        self.signature = signature.to_string();
    }

    fn create_message(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(self.sender.as_bytes());
        bytes.extend(self.recipient.as_bytes());
        bytes.extend(self.amount.to_le_bytes());
        bytes
    }

    fn verify_transaction(&self, public_key: &PublicKey) -> bool {
        // Create Sepc256k1
        let context = Secp256k1::new();
        // Create message from transaction
        let message = self.create_message();
        // Decode signature from string into Signature type
        let signature = Signature::from_str(&self.signature).expect("Error creating signature");
        
        context.verify_ecdsa(&Message::from_slice(&message).expect("Could not parse message"), &signature, public_key).is_ok()
    }
}

// What is ecdsa
// How does it work
// Are digital signatures used in ssh is it the same?
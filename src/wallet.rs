
use secp256k1::rand::rngs::OsRng;
use secp256k1::{Secp256k1,PublicKey, SecretKey};

struct Wallet {
    name: String,
    public_key: PublicKey,
    private_key: SecretKey,
    balances: Vec<String>
}

impl Wallet {
    pub fn new(username: String) -> Self {
        let (secret_key, public_key) =  Wallet::generate_public_and_private_key_pair();
        Self {
            name: username,
            public_key: public_key,
            private_key: secret_key,
            balances: Vec::new()
        }   
    }

    fn generate_public_and_private_key_pair() -> (SecretKey, PublicKey) {
        let secp = Secp256k1::new();
        // Random number generator that receives randomness from the OS.
        // https://stackoverflow.com/questions/71048413/rng-getting-randomness-from-operating-system-vs-crypto-rngs
        // What is this doing?
        /*
        Generate keypair uses OsRng
        OsRng: Represents a random number generator that sources randomness from the operating system. It is part of the rand crate in Rust.
        OsRng ensures that the randomness used for generating the secret key is secure and derived from a source of true entropy provided by the operating system (e.g., /dev/urandom on Unix-based systems or the Windows Cryptographic API).
         */
        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
        (secret_key, public_key)
    }

    fn get_balances(&self) -> Vec<String> {
        self.balances.clone()
    }

    fn sign_transaction () {
        // Generates new transactions
        // Hashes transactions
        todo!()
    }


    fn verify_new_transaction(){
        // verifies incoming transaction signature from you to someone else?
        todo!()
    }
}
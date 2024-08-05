use serde::{de::DeserializeOwned, Deserializer};
use risc0_zkvm::{guest::sha::Impl, sha::Sha256, Receipt};

// fn f() -> Result<[u8; 32], risc0_zkvm::serde::Error>{
//     let a = [10; 32].as_slice();
//     let mut b = risc0_zkvm::serde::Deserializer::new(a);
//     <[u8; 32] as DeserializeOwned>::deserialize(&mut b)
// }

// fn g(){
//     let a = f();
// }

fn f<T: DeserializeOwned>() -> Result<T, risc0_zkvm::serde::Error>{
    let a = [10; 32].as_slice();
    let mut b = risc0_zkvm::serde::Deserializer::new(a);
    T::deserialize(&mut b)
}

fn g(){
    let a = f::<[u8; 32]>();
}

fn main(){
    g();
}
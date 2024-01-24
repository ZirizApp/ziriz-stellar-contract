use soroban_sdk::{contracttype, String, Address};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Metadata {
  pub short_description_uri: String,  // IPFS hash or URL
  pub long_description_uri: String,   // IPFS hash or URL
  pub data_file_uri: String, 
  pub symbol: String,        
  pub issuer: Address,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Series {
  pub creator: Address,
  pub metadata: Metadata,
  pub price: u128,
  pub artist_cut: u128,
  pub fan_cut: u128,
  pub sales: u128,
}
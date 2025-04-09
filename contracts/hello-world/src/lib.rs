#![no_std]
// nft ticketing platform 
// for privac y reasons, the contract is not public
// but the test is public
// This contract is a simple "Hello, World!" example for the Soroban SDK.
// It demonstrates how to create a contract that returns a greeting message.
use soroban_sdk::{contract, contracterror, contractimpl, log, Address, String, Env, Symbol};


#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    // This error is returned when the input string is empty.
    NotOwner = 1,
    NoOwner = 2,
}

#[contract]
pub struct MyFirstNftTicketingContract;

#[contractimpl]
impl MyFirstNftTicketingContract {
    // This function takes a name as input and returns a greeting message.
    pub fn initialize(env: Env, admin: Address, name: String, symbol: Symbol) {
        // above initialize 
        log!(&env, "admin: {}, symbol:", admin, name, symbol);
    }

    //Mint a seat token 
    pub fn mint(env: Env, admin: Address, seat: u32, owner: Address) -> u32 {
        // check if the seat is already token 
        if env.storage().persistent().has(&seat) {
            panic!("this seat is already token");
        }
        env.storage().persistent().set(&seat, &owner);
        seat
    }

    // owner of the ticket 
    pub fn owner_of(env: Env, seat: u32) -> Option<Address> {
        // check if the seat is already token 
        env.storage().persistent().get(&seat)
    }

    // transfer address "from to address "to""
    pub fn transfer(env: Env, seat: u32, from: Address, to: Address) -> Result<u32, Error> {
        let current_owner: Option<Address> = env.storage().persistent().get(&seat);
        log!(&env, "check current_owner: {}", current_owner);
        
        match current_owner {
            Some(owner) => {
                if owner != from {
                    return Err(Error::NotOwner);
                }
                log!(&env, "Valid owner, proceeding with transfer");
            },
            None => return Err(Error::NoOwner),
        }
        env.storage().persistent().set(&seat, &to);
        Ok(seat)
    }
}


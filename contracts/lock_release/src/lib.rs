#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, token, xdr::ScErrorCode, xdr::ScErrorType, Address,
    Bytes, Env, Error, String,
};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Init,
    Owner,
    Admin,
    LockData,
    Config,
    ReentrancyGuard,
    Paused, 
}

#[derive(Clone)]
#[contracttype]
pub struct LockData {
    pub user_address: Address,
    pub dest_token: String,
    pub from_token: Address,
    pub in_amount: i128,
    pub swaped_amount: i128,
    pub recipient_address: String,
    pub dest_chain: Bytes,
}

#[derive(Clone)]
#[contracttype]
pub struct AdminData {
    pub admin_address: Address,
}

#[derive(Clone)]
#[contracttype]
pub struct Config {
    pub fee_percentage: i128,
}

#[contract]
pub struct LockAndReleaseContract;

fn check_and_set_reentrancy_guard(env: &Env) {
    if env.storage().instance().has(&DataKey::ReentrancyGuard) {
        env.panic_with_error(Error::from_type_and_code(
            ScErrorType::Contract,
            ScErrorCode::InvalidAction,
        ));
    }
    env.storage().instance().set(&DataKey::ReentrancyGuard, &());
}

fn clear_reentrancy_guard(env: &Env) {
    env.storage().instance().remove(&DataKey::ReentrancyGuard);
}

fn check_if_paused(env: &Env) {
    if env.storage().instance().has(&DataKey::Paused) {
        env.panic_with_error(Error::from_type_and_code(
            ScErrorType::Contract,
            ScErrorCode::InvalidAction,
        ));
    }
}

#[contractimpl]
impl LockAndReleaseContract {
    pub fn initialize(env: Env, owner: Address, fee_percentage: i128) {
        if env.storage().instance().has(&DataKey::Init) {
            env.panic_with_error(Error::from_type_and_code(
                ScErrorType::Contract,
                ScErrorCode::ExistingValue,
            ));
        }

        env.storage().instance().set(&DataKey::Owner, &owner);
        env.storage().instance().set(&DataKey::Config, &Config { fee_percentage });
        env.storage().instance().set(&DataKey::Init, &());
    }

    pub fn add_admin(env: Env, admin: Address) {
        check_if_paused(&env);
        let owner: Address = env.storage().instance().get(&DataKey::Owner).unwrap();
        owner.require_auth();
        

        if env.storage().instance().has(&DataKey::Admin) {  
            env.panic_with_error(Error::from_type_and_code(
                ScErrorType::Contract,
                ScErrorCode::InvalidAction,
            ));
        }

        env.storage().instance().set(
            &DataKey::Admin,
            &AdminData {
                admin_address: admin.clone(),
            },
        );

        let topics = ("AdminAddedEvent", admin.clone());
        env.events().publish(topics, AdminData { admin_address: admin });
    }

    pub fn remove_admin(env: Env) {
        check_if_paused(&env);
        let owner: Address = env.storage().instance().get(&DataKey::Owner).unwrap();
        owner.require_auth();

        if !env.storage().instance().has(&DataKey::Admin) {
            env.panic_with_error(Error::from_type_and_code(
                ScErrorType::Contract,
                ScErrorCode::MissingValue,
            ));
        }

        env.storage().instance().remove(&DataKey::Admin);

        let topics = ("AdminRemovedEvent", ());
        env.events().publish(topics, ());
    }

    pub fn pause(env: Env) {
        let owner: Address = env.storage().instance().get(&DataKey::Owner).unwrap();
        owner.require_auth(); 

        if env.storage().instance().has(&DataKey::Paused) {
            env.panic_with_error(Error::from_type_and_code(
                ScErrorType::Contract,
                ScErrorCode::ExistingValue,
            ));
        }

        // Set the contract state to paused
        env.storage().instance().set(&DataKey::Paused, &());

        let topics = ("ContractPausedEvent", ());
        env.events().publish(topics, ());
    }

    pub fn unpause(env: Env) {
        let owner: Address = env.storage().instance().get(&DataKey::Owner).unwrap();
        owner.require_auth(); 

        if !env.storage().instance().has(&DataKey::Paused) {
            env.panic_with_error(Error::from_type_and_code(
                ScErrorType::Contract,
                ScErrorCode::MissingValue,
            ));
        }

        // Set the contract state to unpaused
        env.storage().instance().remove(&DataKey::Paused);

        let topics = ("ContractUnpausedEvent", ());
        env.events().publish(topics, ());
    }

    pub fn lock(
        env: Env,
        user_address: Address,
        from_token: Address,
        dest_token: String,
        in_amount: i128,
        dest_chain: Bytes,
        recipient_address: String,
    ) {
        // Check if contract is paused before proceeding
        check_if_paused(&env);

        // Set re-entrancy guard
        check_and_set_reentrancy_guard(&env);
        
        // Authorization and input validation
        user_address.require_auth();
        if in_amount < 1 {
            env.panic_with_error(Error::from_type_and_code(
                ScErrorType::Contract,
                ScErrorCode::InvalidAction,
            ));
        }

        // Check if an admin exists
        if !env.storage().instance().has(&DataKey::Admin) {
            env.panic_with_error(Error::from_type_and_code(
                ScErrorType::Contract,
                ScErrorCode::MissingValue,
            ));
        }

        // Verify user's balance before proceeding
        let user_balance = token::Client::new(&env, &from_token).balance(&user_address);
        if user_balance < in_amount {
            env.panic_with_error(Error::from_type_and_code(
                ScErrorType::Contract,
                ScErrorCode::InvalidAction,
            ));
        }
        
        // Fee and swap calculations
        let config: Config = env.storage().instance().get(&DataKey::Config).unwrap();
        let fee = in_amount * config.fee_percentage / 100;
        let swaped_amount = in_amount - fee;

        // Ensure valid swap amount after fee
        if swaped_amount < 1 {
            env.panic_with_error(Error::from_type_and_code(
                ScErrorType::Contract,
                ScErrorCode::InvalidAction,
            ));
        }

        // Update state before external interactions
        env.storage().instance().set(
            &DataKey::LockData,
            &LockData {
                user_address: user_address.clone(),
                dest_token: dest_token.clone(),
                from_token: from_token.clone(),
                in_amount,
                swaped_amount,
                recipient_address: recipient_address.clone(),
                dest_chain: dest_chain.clone(),
            },
        );

        // Perform token transfer (interaction with external contract)
        token::Client::new(&env, &from_token)
            .transfer(&user_address, &env.current_contract_address(), &in_amount);
        
        // Transfer fee to admin
        let admin_data: AdminData = env.storage().instance().get(&DataKey::Admin).unwrap();
        token::Client::new(&env, &from_token)
            .transfer(&env.current_contract_address(), &admin_data.admin_address, &swaped_amount);

        // Publish lock event
        let topics = (
            "LockEvent",
            user_address.clone(),
            dest_token.clone(),
            in_amount,
            swaped_amount,
        );
        env.events().publish(
            topics,
            LockData {
                user_address,
                dest_token,
                from_token,
                in_amount,
                swaped_amount,
                recipient_address,
                dest_chain,
            },
        );

        // Clear re-entrancy guard
        clear_reentrancy_guard(&env);
    }

    pub fn release(env: Env, amount: i128, user: Address, destination_token: Address) {
        // Check if contract is paused before proceeding
        check_if_paused(&env);

        // Set re-entrancy guard
        check_and_set_reentrancy_guard(&env);
        
        // Admin authorization
        let admin_data: AdminData = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin_data.admin_address.require_auth();
        
        // Check admin balance
        let admin_balance = token::Client::new(&env, &destination_token).balance(&admin_data.admin_address);
        if admin_balance < amount {
            env.panic_with_error(Error::from_type_and_code(
                ScErrorType::Contract,
                ScErrorCode::InvalidAction,
            ));
        }

        // Perform token transfer to the user
        token::Client::new(&env, &destination_token).transfer(&admin_data.admin_address, &user, &amount);
        
        // Publish release event
        let topics = ("ReleaseEvent", user.clone(), destination_token.clone(), amount);
        env.events().publish(topics, ());
        
        // Clear re-entrancy guard
        clear_reentrancy_guard(&env);
    }
}

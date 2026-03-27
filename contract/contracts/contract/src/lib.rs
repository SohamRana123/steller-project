#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, token};

// Define the data structure for a subscription
#[contracttype]
#[derive(Clone)]
pub struct Subscription {
    pub provider: Address,
    pub token: Address,
    pub amount: i128,
    pub interval: u64,        // Time between payments in seconds
    pub last_payment: u64,    // Timestamp of the last successful charge
}

#[contract]
pub struct CryptoSubscriptionContract;

#[contractimpl]
impl CryptoSubscriptionContract {
    /// Initializes a new subscription.
    /// The subscriber must separately grant an allowance to this contract address on the Token.
    pub fn subscribe(
        env: Env,
        subscriber: Address,
        provider: Address,
        token: Address,
        amount: i128,
        interval: u64,
    ) {
        // Ensure the subscriber actually authorized this action
        subscriber.require_auth();

        let current_time = env.ledger().timestamp();

        let sub = Subscription {
            provider,
            token,
            amount,
            interval,
            last_payment: current_time, // Starts the clock from the moment of subscription
        };

        // Store the subscription in persistent storage, keyed by the subscriber's address
        env.storage().persistent().set(&subscriber, &sub);
    }

    /// Pulls funds from the subscriber to the provider.
    /// Can be called by anyone, but will only succeed if the interval has passed.
    pub fn charge(env: Env, subscriber: Address) {
        // Retrieve the subscription details
        let mut sub: Subscription = env.storage().persistent().get(&subscriber).expect("No active subscription found.");
        
        let current_time = env.ledger().timestamp();
        
        // Enforce the time interval constraint
        if current_time < sub.last_payment + sub.interval {
            panic!("Subscription interval has not elapsed yet.");
        }

        // Initialize the token client
        let token_client = token::Client::new(&env, &sub.token);
        
        // Pull the funds using the allowance previously granted by the subscriber
        token_client.transfer_from(
            &env.current_contract_address(),
            &subscriber,
            &sub.provider,
            &sub.amount,
        );

        // Update the last payment timestamp and save back to storage
        sub.last_payment = current_time;
        env.storage().persistent().set(&subscriber, &sub);
    }

    /// Cancels the subscription
    pub fn cancel(env: Env, subscriber: Address) {
        subscriber.require_auth();
        env.storage().persistent().remove(&subscriber);
    }
}
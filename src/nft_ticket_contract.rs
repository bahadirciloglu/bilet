use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, Promise};
use near_sdk::collections::UnorderedMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

#[derive(BorshDeserialize, BorshSerialize)]
pub enum StorageKey {
    Tickets,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
pub struct Ticket {
    pub ticket_id: u64,
    pub event_id: u64,
    pub owner: AccountId,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct NFTTicketContract {
    pub tickets: UnorderedMap<u64, Ticket>,
    pub ticket_counter: u64,
}

#[near_bindgen]
impl NFTTicketContract {
    #[init]
    pub fn new() -> Self {
        Self {
            tickets: UnorderedMap::new(StorageKey::Tickets.try_to_vec().unwrap()),
            ticket_counter: 0,
        }
    }

    // Mint a new NFT ticket for an event
    pub fn mint_ticket(&mut self, event_id: u64, owner: AccountId) -> u64 {
        let ticket_id = self.ticket_counter;
        let ticket = Ticket {
            ticket_id,
            event_id,
            owner: owner.clone(),
        };
        self.tickets.insert(&ticket_id, &ticket);
        self.ticket_counter += 1;
        env::log_str(&format!("Minted ticket with ID {} for event {}", ticket_id, event_id));
        ticket_id
    }

    // Get a ticket by its ID
    pub fn get_ticket(&self, ticket_id: u64) -> Option<Ticket> {
        self.tickets.get(&ticket_id)
    }

    // Transfer ticket ownership to another user
    pub fn transfer_ticket(&mut self, ticket_id: u64, new_owner: AccountId) {
        let mut ticket = self.tickets.get(&ticket_id).expect("Ticket not found");
        assert_eq!(env::predecessor_account_id(), ticket.owner, "Only the owner can transfer the ticket");
        ticket.owner = new_owner.clone();
        self.tickets.insert(&ticket_id, &ticket);
        env::log_str(&format!("Transferred ticket {} to {}", ticket_id, new_owner));
    }

    // Verify ticket ownership
    pub fn verify_ticket(&self, ticket_id: u64, owner: AccountId) -> bool {
        if let Some(ticket) = self.tickets.get(&ticket_id) {
            ticket.owner == owner
        } else {
            false
        }
    }
}

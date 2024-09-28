use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};
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
pub struct QRCodeVerificationContract {
    pub tickets: UnorderedMap<u64, Ticket>,
}

#[near_bindgen]
impl QRCodeVerificationContract {
    #[init]
    pub fn new() -> Self {
        Self {
            tickets: UnorderedMap::new(StorageKey::Tickets.try_to_vec().unwrap()),
        }
    }

    // Function to generate a unique verification string (for frontend QR code generation)
    pub fn generate_qr_code(&self, ticket_id: u64) -> String {
        let ticket = self.tickets.get(&ticket_id).expect("Ticket not found");

        // Generate a simple string that can be used to create a QR code
        let qr_code_string = format!(
            "https://my-event-platform.com/verify/{}?owner={}&event={}",
            ticket.ticket_id, ticket.owner, ticket.event_id
        );

        env::log_str(&format!("QR Code generated: {}", qr_code_string));
        qr_code_string
    }

    // Function to verify ownership of the ticket using the generated string (or QR code)
    pub fn verify_qr_code(&self, ticket_id: u64, owner: AccountId) -> bool {
        if let Some(ticket) = self.tickets.get(&ticket_id) {
            if ticket.owner == owner {
                env::log_str(&format!("Ticket {} is valid for owner {}", ticket_id, owner));
                true
            } else {
                env::log_str("Invalid ticket owner");
                false
            }
        } else {
            env::log_str("Ticket not found");
            false
        }
    }

    // Optional: Simulate minting a ticket for testing purposes
    pub fn mint_ticket(&mut self, ticket_id: u64, event_id: u64, owner: AccountId) {
        let ticket = Ticket {
            ticket_id,
            event_id,
            owner: owner.clone(),
        };
        self.tickets.insert(&ticket_id, &ticket);
        env::log_str(&format!("Minted ticket {} for event {}", ticket_id, event_id));
    }
}

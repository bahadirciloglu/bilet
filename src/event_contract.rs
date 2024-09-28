use near_sdk::{env, near_bindgen, borsh::{self, BorshDeserialize, BorshSerialize}, AccountId};
use std::collections::HashMap;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Event {
    pub event_id: u64,
    pub available_tickets: u64,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Ticket {
    pub ticket_id: u64,
    pub owner: AccountId,
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct EventContract {
    pub events: HashMap<u64, Event>,
    pub tickets: HashMap<u64, Ticket>,
    pub next_ticket_id: u64,
}

#[near_bindgen]
impl EventContract {
    // Function to mint tickets for a specific event
    pub fn mint_ticket(&mut self, event_id: u64) -> Option<Ticket> {
        // Find the event by event_id
        let mut event = self.events.get_mut(&event_id).expect("Event not found");

        // Check if tickets are still available
        assert!(event.available_tickets > 0, "No more tickets available");

        // Decrease available tickets
        event.available_tickets -= 1;

        // Generate a new ticket
        let ticket_id = self.next_ticket_id;
        self.next_ticket_id += 1;

        let owner = env::predecessor_account_id();
        let ticket = Ticket {
            ticket_id,
            owner: owner.clone(),
        };

        // Store the ticket in the contract
        self.tickets.insert(ticket_id, ticket.clone());

        // Return the newly minted ticket
        Some(ticket)
    }

    // Function to add events (for testing or future use)
    pub fn add_event(&mut self, event_id: u64, available_tickets: u64) {
        let event = Event {
            event_id,
            available_tickets,
        };
        self.events.insert(event_id, event);
    }
}

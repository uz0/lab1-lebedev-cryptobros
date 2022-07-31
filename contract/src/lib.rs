use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, env};
use near_sdk::collections::UnorderedMap;
use near_sdk::env::is_valid_account_id;
use near_sdk::serde::{Serialize};

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Cryptobros,
    Cryptobro { account_id: AccountId },
}

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub enum BrosStatus {
    Following,
    FollowedBy,
    Bros,
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Cryptobros {
    bros: UnorderedMap<AccountId, UnorderedMap<AccountId, BrosStatus>>,
}

#[near_bindgen]
impl Cryptobros {
    #[init]
    pub fn new() -> Self {
        Self {
            bros: UnorderedMap::new(StorageKey::Cryptobros),
        }
    }

    pub fn get_bros(&self, account_id: String) -> HashMap<AccountId, BrosStatus> {
        assert!(is_valid_account_id(account_id.as_bytes()), "Specify correct account ID");

        let connections = self.bros.get(&AccountId::new_unchecked(account_id)).unwrap();

        let keys = connections.keys_as_vector();
        keys.iter().fold(HashMap::new(), |mut hashmap, key| {
            hashmap.insert(key.clone(), connections.get(&key).unwrap());
            hashmap
        })
    }

    fn get_or_create_connections(&mut self, account_id: &AccountId) -> UnorderedMap<AccountId, BrosStatus> {
        self.bros.get(&account_id).unwrap_or_else(|| {
            let new_unordered_map = UnorderedMap::new(StorageKey::Cryptobro { account_id: account_id.clone() });

            self.bros.insert(&account_id, &new_unordered_map);

            new_unordered_map
        })
    }

    pub fn add_bro(&mut self, account_id: String) {
        assert!(is_valid_account_id(account_id.as_bytes()), "Specify correct account ID");

        let caller_account_id = env::predecessor_account_id();
        let target_account_id: AccountId = AccountId::new_unchecked(account_id.clone());

        assert!(caller_account_id != target_account_id, "No need to add yourself as a bro, bro");

        let mut caller_connections = self.get_or_create_connections(&caller_account_id);
        let mut target_connections = self.get_or_create_connections(&target_account_id);

        let caller_target_connection = caller_connections.get(&target_account_id);
        let new_statuses: Option<(BrosStatus, BrosStatus)> = match caller_target_connection {
            None => Some((BrosStatus::Following, BrosStatus::FollowedBy)),
            Some(BrosStatus::FollowedBy) => Some((BrosStatus::Bros, BrosStatus::Bros)),
            _ => None,
        };

        match new_statuses {
            Some((caller_target_status, target_caller_status)) => {
                caller_connections.insert(&target_account_id, &caller_target_status);
                self.bros.insert(&caller_account_id, &caller_connections);

                target_connections.insert(&caller_account_id, &target_caller_status);
                self.bros.insert(&target_account_id, &target_connections);
            }
            _ => {}
        }
    }

    pub fn remove_bro(&mut self, account_id: String) {
        assert!(is_valid_account_id(account_id.as_bytes()), "Specify correct account ID");

        let caller_account_id = env::predecessor_account_id();
        let target_account_id: AccountId = AccountId::new_unchecked(account_id.clone());

        let mut caller_connections = self.bros.get(&caller_account_id).unwrap();
        let mut target_connections = self.bros.get(&target_account_id).unwrap();

        match caller_connections.get(&target_account_id) {
            Some(BrosStatus::Following) => {
                caller_connections.remove(&target_account_id);
                if caller_connections.is_empty() {
                    self.bros.remove(&caller_account_id);
                } else {
                    self.bros.insert(&caller_account_id, &caller_connections);
                }

                target_connections.remove(&caller_account_id);
                if target_connections.is_empty() {
                    self.bros.remove(&target_account_id);
                } else {
                    self.bros.insert(&target_account_id, &target_connections);
                }
            },
            Some(BrosStatus::Bros) => {
                caller_connections.insert(&target_account_id, &BrosStatus::FollowedBy);
                self.bros.insert(&caller_account_id, &caller_connections);

                target_connections.insert(&caller_account_id, &BrosStatus::Following);
                self.bros.insert(&target_account_id, &target_connections);
            },
            _ => {},
        };
    }
}

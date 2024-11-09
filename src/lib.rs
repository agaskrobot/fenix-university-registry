use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, require, BorshStorageKey};
use schemars::JsonSchema;

/// Enum for managing different storage keys used within the smart contract.
#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKey {
    /// Storage key for mapping universities by account ID.
    UniversitiesAccounts,
    /// Storage key for mapping universities by their names.
    UniversitiesByName,
}

/// Struct representing a university with a name and associated account ID.
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Clone, Debug, JsonSchema, PartialEq)]
pub struct University {
    /// The name of the university.
    pub name: String,
    /// The unique account ID associated with the university.
    pub account_id: String,
}

/// Main smart contract struct for managing university registration and lookups.
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct UniversityRegistry {
    /// A map of universities keyed by account ID.
    universities_accounts: UnorderedMap<String, University>,
    /// A map of universities grouped by name.
    universities_by_name: UnorderedMap<String, Vec<University>>,
}

impl Default for UniversityRegistry {
    fn default() -> Self {
        Self {
            universities_accounts: UnorderedMap::new(StorageKey::UniversitiesAccounts),
            universities_by_name: UnorderedMap::new(StorageKey::UniversitiesByName),
        }
    }
}

#[near_bindgen]
impl UniversityRegistry {
    /// Adds a new university to the registry.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the university.
    /// * `account_id` - The unique account ID of the university.
    ///
    /// # Returns
    ///
    /// Returns the newly added `University` object.
    ///
    /// # Panics
    ///
    /// Panics if the caller is not the contract owner or if the account ID already exists in the registry.
    pub fn add_university(&mut self, name: String, account_id: String) -> University {
        require!(
            env::signer_account_id() == env::current_account_id(),
            "Permission denied"
        );
        require!(
            self.universities_accounts.get(&account_id).is_none(),
            "Account already exists"
        );

        let university = University { name, account_id };

        self.universities_accounts
            .insert(&university.account_id, &university);
        self.add_university_by_name(university.clone());

        university
    }

    /// Internal helper function to add a university to the `universities_by_name` map.
    ///
    /// # Arguments
    ///
    /// * `university` - A `University` object to add to the name map.
    fn add_university_by_name(&mut self, university: University) {
        match self.universities_by_name.get(&university.name) {
            None => {
                self.universities_by_name
                    .insert(&university.name, &vec![university.clone()]);
            }
            Some(mut universities_by_name) => {
                universities_by_name.push(university.clone());
                self.universities_by_name
                    .insert(&university.name, &universities_by_name);
            }
        };
    }

    /// Retrieves all universities currently stored in the registry.
    ///
    /// # Returns
    ///
    /// Returns a vector of tuples, where each tuple contains an account ID and the associated `University` struct.
    pub fn get_all_universities(&self) -> Vec<(String, University)> {
        self.universities_accounts.to_vec()
    }

    /// Retrieves universities by a given name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the universities to retrieve.
    ///
    /// # Returns
    ///
    /// Returns a vector of `University` objects that match the given name.
    pub fn get_universities_by_name(self, name: String) -> Vec<University> {
        match self.universities_by_name.get(&name) {
            None => Vec::<University>::new(),
            Some(universities_by_name) => universities_by_name,
        }
    }

    /// Retrieves a university by its account ID.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The account ID of the university to retrieve.
    ///
    /// # Returns
    ///
    /// Returns an `Option<University>`. `Some(University)` if found, or `None` if not found.
    pub fn get_university_by_account_id(self, account_id: String) -> Option<University> {
        self.universities_accounts.get(&account_id)
    }
}

/// Unit tests for the `UniversityRegistry` contract.
#[cfg(test)]
mod tests {
    use near_sdk::{test_utils::VMContextBuilder, testing_env};

    use super::*;

    /// Test adding a university and verifying that it is stored correctly.
    #[test]
    fn add_university() {
        let mut contract = UniversityRegistry {
            universities_accounts: UnorderedMap::new(StorageKey::UniversitiesAccounts),
            universities_by_name: UnorderedMap::new(StorageKey::UniversitiesByName),
        };
        set_context_as_admin();

        contract.add_university("UMA".to_string(), "uni_id".parse().unwrap());
        let university = contract
            .get_university_by_account_id("uni_id".parse().unwrap())
            .unwrap();

        assert_eq!("uni_id".to_string(), university.account_id);
        assert_eq!("UMA".to_string(), university.name);
    }

    /// Test adding a university by name and verifying that it is grouped correctly in the name map.
    #[test]
    fn add_university_by_name() {
        let mut contract = UniversityRegistry {
            universities_accounts: UnorderedMap::new(StorageKey::UniversitiesAccounts),
            universities_by_name: UnorderedMap::new(StorageKey::UniversitiesByName),
        };
        set_context_as_admin();

        contract.add_university_by_name(University {
            name: "UMA".to_string(),
            account_id: "uni_id".parse().unwrap(),
        });

        let university_by_name = contract.get_universities_by_name("UMA".to_string());
        assert_eq!("uni_id".to_string(), university_by_name[0].account_id);
        assert_eq!("UMA".to_string(), university_by_name[0].name);
    }

    /// Test that adding a university without admin permissions panics as expected.
    #[test]
    #[should_panic]
    fn panics_on_permissions() {
        let mut contract = UniversityRegistry {
            universities_accounts: UnorderedMap::new(StorageKey::UniversitiesAccounts),
            universities_by_name: UnorderedMap::new(StorageKey::UniversitiesByName),
        };
        set_context_as_user();

        contract.add_university("UMA".to_string(), "uni_id".parse().unwrap());
    }

    /// Test that adding a duplicate university account ID panics as expected.
    #[test]
    #[should_panic]
    fn panics_on_duplicate() {
        let mut contract = UniversityRegistry {
            universities_accounts: UnorderedMap::new(StorageKey::UniversitiesAccounts),
            universities_by_name: UnorderedMap::new(StorageKey::UniversitiesByName),
        };
        set_context_as_admin();

        contract.add_university("UMA".to_string(), "uni_id".parse().unwrap());
        contract.add_university("UMA".to_string(), "uni_id".parse().unwrap());
    }

    /// Sets the testing environment context as an admin account.
    fn set_context_as_admin() {
        let mut builder = VMContextBuilder::new();
        builder.current_account_id("admin".parse().unwrap());
        builder.signer_account_id("admin".parse().unwrap());
        testing_env!(builder.build());
    }

    /// Sets the testing environment context as a regular user account.
    fn set_context_as_user() {
        let mut builder = VMContextBuilder::new();
        builder.current_account_id("admin".parse().unwrap());
        builder.signer_account_id("user".parse().unwrap());
        testing_env!(builder.build());
    }
}

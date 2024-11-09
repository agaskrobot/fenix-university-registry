use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, require, BorshStorageKey};
use schemars::JsonSchema;

#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKey {
    UniversitiesAccounts,
    UniversitiesByName,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Clone, Debug, JsonSchema, PartialEq)]
pub struct University {
    pub name: String,
    pub account_id: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct UniversityRegistry {
    universities_accounts: UnorderedMap<String, University>,
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
    pub fn add_university(&mut self, name: String, account_id: String) -> University {
        require!(
            env::signer_account_id() == env::current_account_id(),
            "Permission denied"
        );
        require!(
            self.universities_accounts.get(&account_id).is_none(),
            "Account already exist"
        );

        let university = University { name, account_id };

        self.universities_accounts
            .insert(&university.account_id, &university);
        self.add_university_by_name(university.clone());

        university
    }

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

    pub fn get_all_universities(&self) -> Vec<(String, University)> {
        self.universities_accounts.to_vec()
    }

    pub fn get_universities_by_name(self, name: String) -> Vec<University> {
        match self.universities_by_name.get(&name) {
            None => Vec::<University>::new(),
            Some(universities_by_name) => universities_by_name,
        }
    }

    pub fn get_university_by_account_id(self, account_id: String) -> Option<University> {
        self.universities_accounts.get(&account_id)
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be: `cargo test`
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use near_sdk::{test_utils::VMContextBuilder, testing_env};

    use super::*;

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

    fn set_context_as_admin() {
        let mut builder = VMContextBuilder::new();
        builder.current_account_id("admin".parse().unwrap());
        builder.signer_account_id("admin".parse().unwrap());
        testing_env!(builder.build());
    }

    fn set_context_as_user() {
        let mut builder = VMContextBuilder::new();
        builder.current_account_id("admin".parse().unwrap());
        builder.signer_account_id("user".parse().unwrap());
        testing_env!(builder.build());
    }
}

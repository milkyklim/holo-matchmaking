#![feature(try_from, proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    AGENT_ADDRESS, // api::AGENT_ADDRESS,
    EntryValidationData,
};
use hdk::holochain_core_types::{
    entry::Entry,
    dna::entry_types::Sharing,
    link::LinkMatch,
};

use hdk::holochain_json_api::{
    json::JsonString,
    error::JsonError
};

use hdk::holochain_persistence_api::{
    cas::content::{Address, AddressableContent},
};

use hdk_proc_macros::zome;

// see https://developer.holochain.org/api/0.0.18-alpha1/hdk/ for info on using the hdk library

// This is a sample zome that defines an entry type "MyEntry" that can be committed to the
// agent's chain via the exposed function create_my_entry

#[derive(Serialize, Deserialize, Debug, DefaultJson,Clone)]
pub struct GameProposal {
    agent: Address,
    message: String,
}

#[zome]
mod my_zome {

    #[genesis]
    fn genesis() {
        Ok(())
    }

    #[entry_def]
     fn game_proposal_entry_def() -> ValidatingEntryType {
        entry!(
            name: "game_proposal",
            description: "this is a same entry representing a proposal to play a game",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | validation_data: hdk::EntryValidationData<GameProposal>| {
                match validation_data {
                    EntryValidationData::Create {entry, validation_data} => {
                        
                        let proposal = GameProposal::from(entry);
                        if validation_data.sources().contains(&proposal.agent) {
                            Ok(())
                        } else {
                            Err("Cannot author a proposal from another agent".into())
                        }
                        
                    },
                    _ => {
                        Err("Updating or deleting proposal is not allowed".into())
                    }
                }
            }
        )
    }

    #[entry_def]
    fn anchor_entry_def() -> ValidatingEntryType {
        entry!{
            name: "anchor",
            description: "central location to place links",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_validation_data: hdk::EntryValidationData<String>| {
                Ok(())
            },
            links: [
                to!(
                    "game_proposal", // must match target entry type name
                    link_type: "has_proposal", // can be arbitrary
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: |_validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                )
            ]
        }
    }

    #[zome_fn("hc_public")]
    fn create_proposal(message: String) -> ZomeApiResult<Address> {
        let proposal = GameProposal {
            agent: AGENT_ADDRESS.clone(),
            message: message,
        };

        let entry = Entry::App(
            "game_proposal".into(),
            proposal.into()
        );

        let proposal_address = hdk::commit_entry(&entry)?;

        let anchor_entry = Entry::App(
            "anchor".into(),
            "proposals".into(), // this is the entry data
        );

        let anchor_address = hdk::commit_entry(&anchor_entry)?;

        hdk::link_entries(
            &anchor_address, 
            &proposal_address, 
            "has_proposal",
            "")?;

        Ok(proposal_address)
    }


    #[zome_fn("hc_public")]
    fn get_proposals() -> ZomeApiResult<Vec<GameProposal>> {
        let anchor_address = Entry::App(
            "anchor".into(),
            "proposals".into(), // this is the entry data
        ).address();
        
        hdk::utils::get_links_and_load_type(
            &anchor_address, 
            LinkMatch::Exactly("has_proposal"),
            LinkMatch::Any
        )

    }
}

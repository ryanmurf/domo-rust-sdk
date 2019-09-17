use domo::public::dataset::{DataSet, Policy};
use domo::public::Client;

use std::fs;
use std::path::PathBuf;

use structopt::StructOpt;

use super::util;

/// Wraps the dataset api
#[derive(StructOpt, Debug)]
pub enum DataSetCommand {
    /// Get a list of DataSets in your Domo instance.
    #[structopt(name = "list")]
    List {
        #[structopt(short = "l", long = "limit")]
        limit: Option<u32>,
        #[structopt(short = "o", long = "offset")]
        offset: Option<u32>,
    },

    /// Get a list of all DataSets in your Domo instance.
    #[structopt(name = "list-all")]
    ListAll {},

    /// Create a new dataset
    #[structopt(name = "create")]
    Create {},

    /// Retrieves the details of an existing DataSet.
    #[structopt(name = "retrieve")]
    Retrieve { id: String },

    /// Update a dataset
    #[structopt(name = "update")]
    Update { id: String },

    /// Permanently deletes a DataSet from your Domo instance. This can be done for all DataSets, not just those created through the API.
    #[structopt(name = "delete")]
    Delete { id: String },

    /// Import data into a DataSet in your Domo instance. This request will replace the data currently in the DataSet.
    #[structopt(name = "import")]
    Import {
        /// A csv file that will replace all of the data in this dataset
        #[structopt(parse(from_os_str))]
        file: PathBuf,
        /// The dataset to import the data into
        id: String,
    },

    /// Export data from a DataSet in your Domo instance.
    #[structopt(name = "export")]
    Export { id: String },

    /// Returns data from the DataSet based on your SQL query.
    #[structopt(name = "query")]
    Query { id: String, sql: String },

    /// List the Personalized Data Permission (PDP) policies for a specified DataSet.
    ListPolicies { id: String },

    /// Create a PDP policy for user and or group access to data within a DataSet.
    /// Users and groups must exist before creating PDP policy.
    CreatePolicy { id: String },

    /// Retrieve a policy from a DataSet within Domo. A DataSet is required for a PDP policy to exist.
    RetrievePolicy { id: String, policy_id: u32 },

    /// Update the specific PDP policy for a DataSet by providing values to parameters passed.
    UpdatePolicy { id: String, policy_id: u32 },

    /// Permanently deletes a PDP policy on a DataSet in your Domo instance.
    DeletePolicy { id: String, policy_id: u32 },
}

pub fn execute(dc: Client, e: &str, t: Option<String>, command: DataSetCommand) {
    match command {
        DataSetCommand::List { limit, offset } => {
            let r = dc.get_datasets(limit, offset).unwrap();
            util::vec_obj_template_output(r, t);
        }
        DataSetCommand::ListAll {} => {
            let mut offset = 0_u32;
            let mut r: Vec<DataSet> = Vec::new();
            loop {
                let mut ret = dc.get_datasets(Some(50), Some(offset)).unwrap();
                let mut b = false;
                if ret.len() < 50 {
                    b = true;
                }
                //Either way slurp all the elements into the aggregator
                r.append(&mut ret);
                offset += 50;
                if b {
                    break;
                }
            }
            util::vec_obj_template_output(r, t);
        }
        DataSetCommand::Create {} => {
            let r = DataSet::template();
            let r = util::edit_obj(e, r, "").unwrap();
            let r = dc.post_dataset(r).unwrap();
            util::obj_template_output(r, t);
        }
        DataSetCommand::Retrieve { id } => {
            let r = dc.get_dataset(&id).unwrap();
            util::obj_template_output(r, t);
        }
        DataSetCommand::Update { id } => {
            let r = dc.get_dataset(&id).unwrap();
            let r = util::edit_obj(e, r, "").unwrap();
            let r = dc.put_dataset(&id, r).unwrap();
            util::obj_template_output(r, t);
        }
        DataSetCommand::Delete { id } => {
            dc.delete_dataset(&id).unwrap();
        }
        DataSetCommand::Import { file, id } => {
            let csv = fs::read_to_string(file).unwrap();
            dc.put_dataset_data(&id, csv).unwrap();
        }
        DataSetCommand::Export { id } => {
            let r = dc.get_dataset_data(&id).unwrap();
            util::csv_template_output(r, t);
        }
        DataSetCommand::Query { id, sql } => {
            let r = dc.post_dataset_query(&id, &sql).unwrap();
            util::query_template_output(r, t);
        }
        DataSetCommand::ListPolicies { id } => {
            let r = dc.get_dataset_policies(&id).unwrap();
            util::vec_obj_template_output(r, t);
        }
        DataSetCommand::CreatePolicy { id } => {
            let r = Policy::template();
            let r = util::edit_obj(e, r, "").unwrap();
            let r = dc.post_dataset_policy(&id, r).unwrap();
            util::obj_template_output(r, t);
        }
        DataSetCommand::RetrievePolicy { id, policy_id } => {
            let r = dc.get_dataset_policy(&id, policy_id).unwrap();
            util::obj_template_output(r, t);
        }
        DataSetCommand::UpdatePolicy { id, policy_id } => {
            let r = dc.get_dataset_policy(&id, policy_id).unwrap();
            let r = util::edit_obj(e, r, "").unwrap();
            let r = dc.put_dataset_policy(&id, policy_id, r).unwrap();
            util::obj_template_output(r, t);
        }
        DataSetCommand::DeletePolicy { id, policy_id } => {
            dc.delete_dataset_policy(&id, policy_id).unwrap();
        }
    }
}

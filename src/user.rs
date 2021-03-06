use domo::public::user::User;
use domo::public::Client;

use structopt::StructOpt;

use super::util;

/// Wraps the user api
#[derive(StructOpt, Debug)]
pub enum UserCommand {
    /// Get a list of users.
    #[structopt(name = "list")]
    List {
        #[structopt(short = "l", long = "limit")]
        limit: Option<u32>,
        #[structopt(short = "o", long = "offset")]
        offset: Option<u32>,
    },

    /// Get a list of all users.
    #[structopt(name = "list-all")]
    ListAll {},

    /// Create a new user
    #[structopt(name = "create")]
    Create {},

    /// Retrieves the details of an existing user.
    #[structopt(name = "retrieve")]
    Retrieve { user_id: String },

    /// Update a user
    #[structopt(name = "update")]
    Update { user_id: String },

    /// Permanently deletes a user from your Domo instance
    #[structopt(name = "delete")]
    Delete { user_id: String },
}

pub async fn execute(dc: Client, editor: &str, template: Option<String>, command: UserCommand) {
    match command {
        UserCommand::List { limit, offset } => {
            let r = dc.get_users(limit, offset).await.unwrap();
            util::vec_obj_template_output(r, template);
        }
        UserCommand::ListAll {} => {
            let mut offset = 0_u32;
            let mut r: Vec<User> = Vec::new();
            loop {
                let mut ret = dc.get_users(Some(50), Some(offset)).await.unwrap();
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
            util::vec_obj_template_output(r, template);
        }
        UserCommand::Create {} => {
            let r = User::template();
            let r = util::edit_obj(editor, r, "").unwrap();
            let r = dc.post_user(r).await.unwrap();
            util::obj_template_output(r, template);
        }
        UserCommand::Retrieve { user_id } => {
            let r = dc.get_user(&user_id).await.unwrap();
            util::obj_template_output(r, template);
        }
        UserCommand::Update { user_id } => {
            let r = dc.get_user(&user_id).await.unwrap();
            let r = util::edit_obj(editor, r, "").unwrap();
            let r = dc.put_user(&user_id, r).await.unwrap();
            util::obj_template_output(r, template);
        }
        UserCommand::Delete { user_id } => {
            dc.delete_user(&user_id).await.unwrap();
        }
    }
}

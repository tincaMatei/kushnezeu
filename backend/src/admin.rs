extern crate diesel;
extern crate backend;
#[macro_use]
extern crate diesel_migrations;

use self::backend::*;
use self::models::*;
use self::database::DatabaseServer;
use std::env;

use bcrypt::{hash, DEFAULT_COST};

pub mod database;

fn display_instructions() {
    eprintln!("Usage: bacdb-admin COMMAND ARGUMENTS\n");
    eprintln!("           COMMANDS                                 INFORMATION");
    eprintln!("    add-user USERNAME PASSWORD           add an user with the given username");
    eprintln!("                                         and password\n");
    eprintln!("    add-group GROUPNAME                  add a group with the given name\n");
    eprintln!("    add-privillege USERNAME GROUPNAME    add rights to a user, for the");
    eprintln!("    RIGHTS                               given group\n");
    std::process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        display_instructions();
    }
    
    let server = DatabaseServer::start_database();
    match args[1].as_str() {
    "add-user" => {
        if args.len() != 4 {
            display_instructions();
        }
    
        let (username, password) = (args[2].to_lowercase(), hash(&args[3], DEFAULT_COST).unwrap());
        let user = NewUser {
            username,
            password,
        };
        server.add_account(&user);
    }
    "add-group" => {
        if args.len() != 3 {
            display_instructions();
        }
        let groupname = args[2].clone();
        let group: Group = Group {
            name: groupname,
        };
        
        server.add_group(&group);
    }
    "add-privillege" => {
        if args.len() != 5 {
            display_instructions();
        }

        let (username, groupname, rights) = (args[2].to_lowercase(), args[3].clone(), args[4].clone());
        let added_privillege = PrivillegeByUsername {
            username, 
            groupname, 
            rights,
        };
        
        let user = server.get_user_by_username(added_privillege.username);
        let user = if let Some(x) = user {
            x
        } else {
            eprintln!("Failed to load the user");
            std::process::exit(1);
        };
        
        if added_privillege.rights.len() != 4 {
            display_instructions();
        }
        for i in 0..4 {
            if added_privillege.rights.as_bytes()[i] != b'_' &&
               added_privillege.rights.as_bytes()[i] != b"RWXX"[i] {
                display_instructions();
            }
        }

        server.add_privillege(user.id, added_privillege.groupname, added_privillege.rights);
    }
    _ => {
        display_instructions();
    }
    };
}


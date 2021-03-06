use std::path::PathBuf;
use aliases::factories::AliasFactory;
use aliases::collections::Aliases;
use aliases::repositories::UserRepository;
use aliases::models::User;

pub struct AliasRepository;

impl AliasRepository {

    pub fn find_for_directory(directory: &String) -> Result<Aliases, &'static str> {
        let mut aliases = Aliases::new(vec![]); // TODO should be able to use map or inject here
        for user in Self::available_users().iter() {
            match Self::directory_aliases_for_user(directory, &user) {
                None => { },
                Some(user_aliases) => { aliases = aliases.merge(user_aliases); }
            }
        }
        Ok(aliases)
    }

    //--------- private ------------//

    fn available_users() -> Vec<User> {
        UserRepository::enabled()
    }

    fn directory_aliases_for_user(directory: &String, user: &User) -> Option<Aliases> {
        let aliases_file = PathBuf::from(directory).join(&user.filename()); // TODO use the user's file name
        match AliasFactory::create_from_file(aliases_file) {
            Err(_) => { None },
            Ok(aliases) => { Some(aliases) }
        }
    }


}

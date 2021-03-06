use crate::exit_with_error;
use crate::repository::Repository;
use alfred::{Item, ItemBuilder};
use std::io::stdout;
use std::process::exit;

pub struct Update<'a> {
    _lifetime: &'a str,
}

impl<'a> Update<'a> {
    pub fn item() -> Item<'a> {
        ItemBuilder::new("Update .gitignore templates")
            .subtitle("Download the latest templates from github/gitignore")
            .autocomplete("--update")
            .valid(false)
            .into_item()
    }

    pub fn perform(repository: &Repository) -> ! {
        match repository.update() {
            Ok(_) => {
                alfred::json::write_items(
                    stdout(),
                    &[ItemBuilder::new("Successfully updated the templates")
                        .subtitle("Press Enter to start building a .gitignore file")
                        .autocomplete("")
                        .valid(false)
                        .into_item()],
                )
                .unwrap();

                exit(0);
            }
            Err(error) => exit_with_error(&error),
        }
    }
}

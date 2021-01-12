# Oppai
This project is dead. I'm instead writing a JAV plugin for Jellyfin.

# Requirements
* Rust
* Ruby

# How To Run
    $ cat init_db.sql | sqlite3 database.sqlite
    $ ./create_details PATH_TO_JAV_FOLDER # Will take a while
    $ cp settings.example.toml settings.toml
    # Replace PATH in settings.toml with PATH_TO_JAV_FOLDER
    $ cargo run --release
    # Click 'Scan Videos'. Will take a while while scanning each actress.

# Screenshots
![Main Page Screenshot](screenshots/main-page.jpg)

# License
Licensed under AGPL-3.0-only

# Rust Discord RPC Wrapper
This Rust project provides a wrapper for Discord Rich Presence to be updated via a C function, and is intended to be used from a dllbind within UDK for the RenegadeX/Firestorm game.

## Public API
```rust
#[no_mangle]
pub extern "C" fn UpdateDiscordRPC(
    // C-Style terminated wide string of the server name
    server_name_ptr: *const u16,
    // C-Style terminated wide string of the level name
    level_name_ptr: *const u16,
    // Player count on this server
    player_count: u32,
    // Maximum players supported on this server
    max_players: u32,
    // Team number: 0 for GDI, 1 for NOD, 2 for Black Hand
    team_num: u32,
    // Time elapsed since the start of the current round
    time_elapsed: u32,
    // Time remaining untill the end of the round
    time_remaining: u32,
    // Boolean specifying the game-type: false for RenegadeX, true for Firestorm
    is_firestorm: u32,
    // C-Style terminated wide string of the image name
    image_name_ptr: *const u16
);
```
All fields are input parameters.

## Build Requirements
To build this library, you'll need to have the following tools installed on your system:

 * Rust (latest version)
 * cross (for cross-compilation and ease of compilation)
 * docker (required by cross)


## Building
To build the library, you can run the following command:

```bash
cross build --release --target=x86_64-pc-windows-gnu
cross build --release --target=i686-pc-windows-gnu
```
This will generate the discord.dll file in the target/release directory.

## Contributing
Contributions are welcome! If you want to contribute to this project, please fork the repository, make your changes and submit a pull request.

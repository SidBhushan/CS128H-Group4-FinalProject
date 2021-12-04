# CS128H Group 4 FinalProject
## Running instructions

To run this application, it should first be downloaded to the computer using it. This is done through git, as all of the code is hosted in this repository. This can be done using the `git clone` command (using the HTTPS or SSH locations available under the "Code" tab in GitHub).

This application has no external dependencies other than Rust crates, so it can be ran immediately using `cargo run`. This will download the crates if needed, compile the code, and run it.

Do note, however, that in order to actually use the application, it is necessary to open two terminals on the same machine; it is possible the application will across machines on the same network, but I have no way of testing this. Running `cargo run` in both terminals will start separate instances of the application.

Once the application is started, it can then be used to chat between two users. First, a user must add the other client by using the `/connect` command. This is used by typing `/connect [ip] [port]` into the terminal. The ip and port are of the other user. This command must be repeated for both users to establish a mutual connection.

Sending messages can then be done just by typing them into the terminal; messages from the other user will be printed onto the terminal, along with the sender's username. This username can be changed with the `/set_username` command.

To send files, the `/send_file` command is used. This is done by typing `/send_file [path] [file_name]` into the terminal, where `path` is the path to the file to send, and `file_name` is the name to save the file as. Files are saved to the `./files` directory on the receiving machine.

To use encryption, both users should use the `/set_key` command to set their encryption key to the *same* string. If they are different, all messages will be unreadable (and they will just show as `Encrypted Message`). The `/get_key` command will print the current encryption key as a byte array and can be used to verify that the keys are the same. The `/remove_encryption` command disables the encryption and goes back to sending messages in plaintext.
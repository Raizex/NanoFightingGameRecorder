# Introduction
This is a program, in simple terms, allows a client to be able to pair to a raspberry pi and then send commands such as start and stop recording. It is an easy way to start screen recording and you will have the option to just download the client android application for a feasible GUI.

# Hardware Setup
TBA

# Dependencies
In order for you to run the program, I would recommend opening the Cargo.Toml file to see what dependencies are required. You only need to download a few of these onto your computer to run the program. These packages can be installed by using [Scoop](https://scoop.sh/), which is what I used for my windows machine. But, feel free to use another installer like Chocolatey or after you install Cargo you can use that as well.

- **Rust**
  - This is a no brainer as the program was 99% written in Rust lang, but I still want to put it here to be professional. Now, when installing rust from the official rust book website, it will automatically install Cargo for you as well. Cargo will be used to build our project and to also import packages.
    - [Rust Installation](https://doc.rust-lang.org/cargo/getting-started/installation.html)

- **OpenSsl**
  - Installing [OpenSsl](https://github.com/sfackler/rust-openssl#windows) will be required for hosting a secure, encrypted server for the raspberry pi.
  
- **Pkg-Config**
  - This package will be needed in order for OpenSsl to be found by Cargo Build. [Pkg-Config](https://crates.io/crates/pkg-config)

- **mkcert**
  - Installing [mkcert](https://github.com/actix/examples/tree/master/security/rustls) will allow you to generate your own key and certificate for the SSL connection.

# Getting Started
After you install the necessary packages and also explored the Cargo.toml file, we are ready to run the Rest Api which will be setup on the Raspberry Pi as a host device.

**Environment**
First thing you want to do is take a look at the .env file that should be under the api directory. This will have your server and your port configurations. Feel free to change them if you want to, but keeping it as default will be just fine.

**File Guide (For Developers)**

- **main.rs**
  - This is the main file where the api will be running. It will also have your Ssl instantiation and declarations for the server.

- **models.rs**
  - The models page will have your Structs for the Status of the server, Host, and Client.

- **utils.rs**
  - Utils page has some random functions that are used to make my life easier such as a random number generator and also a miliseconds to minutes/seconds converter.

- **config.rs**
  - This is the server configuration page which communicates directly with the dotenv module.

- **client_handlers.rs**
  - These are the handlers for the client which I would advise for you to explore yourself to really understand what goes on underneath the hood of the api.

# Running the API
Here are the steps that you need to take to run the api

1. Make sure that your terminal is pointing to the api directory. For Example: `C:\Users\chiva\OneDrive\Desktop\FightingGameRecorder\JetsonNanoProj\NanoFightingGameRecorder\api>`
2. Run the Cargo Build command in the terminal to build the project. This may take a few minutes `cargo build`
3. Now cargo run will run the api `cargo run`
4. Server should be up!

# Client Testing
Now, in order to pair, start, and stop recording, we need to use either [curl](https://www.educative.io/edpresso/how-to-perform-a-post-request-using-curl) commands to send requests or we can use this [api testing website](https://reqbin.com/). By clicking on the hyperlinks above, you will be directed to a curl tutorial on how to run request commands, and also an api testing website.

**Available Routes**
`http://127.0.0.1:7878/nano/pair` GET-Request for pairing device.
The response will be your pair key.
Save this key! (Ctrl-C)

`http://127.0.0.1:7878/nano/start` Post Request using JSON content type and your pair key that was given to you
JSON Content Ex.: `{"key": "{pair_key}"}`

`http://127.0.0.1:7878/nano/stop` Post Request with same content as above to stop recording

`http://127.0.0.1:7878/nano/unpair` Post Request with the same JSON content as before, which will cause you to lose your pair_key and disconnect.

**This version is still incomplete and we are in the process of constructing an Android Application for the client to make things easier**


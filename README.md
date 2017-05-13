# r(emote)p(aste)

Allows 'copying and pasting' between terminal sessions on any internet connected device by using hastebin as a conduit. No guarantees are made about the integrity or security of the files after being sent through ```rp```. Every paste is public and unencrypted; do not use ```rp``` with confidential information.

You can host your own hastebin server on a personal network, look here for more details: [seejohnrun/haste-server](https://github.com/seejohnrun/haste-server). Simply change the urls in ```src/main.rs``` to point to your sever before compiling.

## Installation

```
git clone https://github.com/z2oh/rp.git
cd rp
cargo build --release
```

Then copy ```./target/release/rp``` to some location in your ```$PATH```.
You must have ```cargo``` installed (and a Rust compiler) in order to compile ```rp```.

## Usage

To upload a paste:

```
rp < file
```
or
```
cat file | rp
```
or
```
rp
enter any text to paste then press ctrl-d to send EOF
```
All of these will print out a 10 character key.

This ten character key can be used anywhere to download the contents of the paste by running the following command
```
rp <key>
```
which will output the contents of the paste.
```
rp <key> > file
```
will print the contents to ```file```.
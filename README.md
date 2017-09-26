
Just playing with servo.

I like browsers, specially the inner bits. I wish servo eventually turns into a browser builder library, that allows us to switch out certain crates with our modified versions, or take out the bits we dont want, or just do insane things because **science!!!**.

## Updating servo

1. servo/ is a git submodule, update it to the desired revision
2. copy servo/Cargo.lock to the root and tweak it if needed

## Known issues

- mozjs bindings fail to build with a python 'SyntaxError', this means mozjs is using the wrong python version (it assumes python is python2)
- servo won't build against libressl 2.5.5, there is an open issue upstrea, you can work around it by updating the openssl crate version in Cargo.lock
- replacing git crates can be tricky, check https://github.com/servo/servo/blob/master/docs/HACKING_QUICKSTART.md#working-on-a-crate for the `mach cargo pkgid` command to get the correct id for the Cargo.toml replace entry

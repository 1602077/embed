# embed

A vector embeddings generator hosted over gRPC.

__CAUTION__: I am still teaching myself Rust, so please bear this is mind. 
Things may not be up to scratch or unbeknownst to me I may be committing a multitude of sins 
because at this point I a) frankly don't know any better and b) haven't been exposed to what good looks like.


# Cross-compilation on mac
```bash
# install musl-gcc
brew install FiloSottile/musl-cross/musl-cross
sudo ln -s /opt/homebrew/bin/x86_64-linux-musl-cc /usr/local/bin/musl-gcc
```

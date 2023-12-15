bindings to [Monkey's Audio SDK](https://www.monkeysaudio.com/developers.html).

Monkey's Audio is a lossless compression algorithm designed for maxmimum space-savings.
This crate has contains the source of the SDK itself -- [libmac-sys/MAC_1029_SDK](libmac-sys/MAC_1029_SDK)
-- and libmac-sys which are *(will be)* the Rust bindings. 

Right now libmac-sys is just me testing things. The crate is still early days but I wanted
to show a friend some code from it and this is just easy. I have to go for a drive and
do laundry, so it'll be a few hours before it makes sense in here :)

# notes
- patches `Source/Shared/CharacterHelper.cpp` to fix the UTF-16 handling. Thank you, lak, for
  helping me debug this and gently guide my implementation.
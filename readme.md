bindings to [Monkey's Audio SDK](https://www.monkeysaudio.com/developers.html).

Monkey's Audio is a lossless compression algorithm designed for maxmimum space-savings.
This crate contains a patched version of the SDK Source
-- [libmac-sys/MAC_1029_SDK](libmac-sys/MAC_1029_SDK) -- and libmac-sys which are the Rust bindings.

# notes
- patches [`CharacterHelper.cpp`][charhelp-path] to fix the UTF-16 handling. Thank you, lak, for
  helping me debug this and gently guide my implementation.

[charhelp-path]: libmac-sys/MAC_1029_SDK/Source/Shared/CharacterHelper.cpp

# license
MAC SDK is Copyright © 2000-2024 by Matthew T. Ashland. (BSD 3-clause)

bindings are Copyright 2023 gennyble <gen@nyble.dev> (ISC)
# Simple Caesar shift implementation in Rust

Supports multiple alphabets and language detection, but only for a very select few articles. Defaults to the Norwegian alphabet, also when language detection fails due to a low probability score to any language. For security, the program does not alert you if you do not get the requested alphabet, or if any of the characters do not exist in said alphabet.  

### Requirements:

Cargo and rustc. That's about it. 

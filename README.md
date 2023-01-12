# Fork of rpassword
[origin README.md](README_ORIGIN.md)

## Purpose of fork
This fork aims at replace the rtoolbox dependency with crates like zeroize.    
HOWEVER, codes like cross platform support & test are removed caused i don't need them and is not able to maintain.

## Uasge
```rust
let password = rpassword::read_password().unwrap();
println!("Your password is {}", password);

// It is recommended to secure clear your password if it's not needed anymore. 
// You can use crates like zeroize or secstr to achive this.
use zeroize::Zeroize;
password.zeroize();
```

## License
The modified / added code is license to public domain. Code from original repository follow their own licence. see [LICENSE-APACHE](LICENSE-APACHE).

## Declarations
- THE MODIFIED / ADDED CODE IS PROVIDED "AS IS", WITHOUT ANY WARRANTY.
- The fork owner MAY or MAY NOT make any pull request to original repository.
- The fork owner will NOT publish any crate of the fork.
- The fork owner MAY NOT wirte test for modified / added code.
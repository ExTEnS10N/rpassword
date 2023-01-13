# Fork of rpassword
[origin README.md](README_ORIGIN.md)

## Purpose of fork
This fork has already refactored the code to meet my own needs, and maybe still useful for anybody else.
Howerver, codes like cross platform support and test are removed, since i can't maintain them myself.

## Uasge
Simple use:
```rust
let password = rpassword::read_password().unwrap();
println!("Your password is {}", password);

// It is recommended to secure clear your password if it's not needed anymore. 
// You can use crates like zeroize or secstr to achive this.
use zeroize::Zeroize;
password.zeroize();
```

With prompt, retry and securely consume the password:
```rust
use std::io::{Error, ErrorKind};
let res = rpassword::ask_password("Enter your password:", |password: &str| {
    // consume your password here, and make sure the code SHALL NOT PANIC here!

    if password.isEmpty() { // if password is wrong, return PermissionDenied to tell user to try again
        println!("Wrong password, please try again.");
        // This will cause password to be ask again automatically, and the wrong one will be securely zeroize too.
        Err(Error::from(ErrorKind::PermissionDenied)) 
    }

    Ok(()) // You can return any other thing you need other than the unit type ().
});
//Password has already securely zeroized here, no matter whether it is wrong or not, if nothing panic above.

match res {
    // if user has retried for 3 times, ask_password will not continue retrying and return this error.
    Err(error) if error.kind() == ErrorKind::PermissionDenied => { 
        panic!(); // now you can panic!() if needed.
    },
    Err(error) => {
        // Any other error will cause ask_password to exit without retry.
    }
    _ => {}
};
```
Notice that ask_password won't retry again 

## License
The modified / added code is license to public domain. Code from original repository follow their own licence. see [LICENSE-APACHE](LICENSE-APACHE).

## Declarations
- THE MODIFIED / ADDED CODE IS PROVIDED "AS IS", WITHOUT ANY WARRANTY.
- The fork owner will NOT make any pull request to original repository.
- The fork owner will NOT publish any crate of the fork.
- The fork owner will NOT test the modified / added code.
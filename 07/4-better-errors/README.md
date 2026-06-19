# Better errors

## Overview

This is the fourth implementation of the 'panic-to-result' macro from chapter 7.
We are now using [`syn::Error`](https://docs.rs/syn/latest/syn/struct.Error.html) instead of `String` for better error handling and
compile time warnings.

Search for [`syn::Error`](https://docs.rs/syn/latest/syn/struct.Error.html) in the source code.

# Background
The purpose of this crate is to provide wrapper functions for interaction with UEFI environemnt. This will also server as dependency for Rust std in my work to port Rust std for UEFI.

# Design Principles
1. Mostly just functions: The structure of this crate will be more like an API gateway.
2. No pointer caching: It seems in some cases, the internal pointers in the SystemTable can be modified by the system. Thus, none of the pointers, not even the SystemTable will be stored in this crate. It just provides an interface to operate on the SystemTable.
3. Safety: Functions where it is possible to provide safety gaurentee (at least as much safety as possible in UEFI) should be safe to call. However, most functions where there is a possiblity of erros should return Error rather than panicing. All pointers should be checked before being operated on.

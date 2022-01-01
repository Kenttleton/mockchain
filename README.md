# Mockchain

Mocking blockchain tech from scratch for learning purposes. The use of libraries is very minimal. If you are looking to build an end product I recommend the Rust [Web3](https://docs.rs/web3/latest/web3/) library for blockchain development in Rust. It is not advised to reinvent the wheel as has been done here.

**DISCLAIMER**: This project is not intended for production use as it is vulnerable to attack by bad actors and lacks many security features that should be implemented for production. Lack of security in design was intentional for observation of communication ease. Some implementations will have security features as an example but without enough adoption the network will fall victim to 51% attacks or failure of decentralization. Expect instability in the services created here.

Docker and Compose are used to spin up copies of the service to create a network of nodes. The plan for this repository is to implement different methods that have been created over the years. Most of the codebase is written in Rust with some Javascript, CSS, and HTML for web interfaces.

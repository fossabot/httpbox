# httpbox
**[WIP]** Command line HTTP client that aims to be as feature rich as you your shell scripts will like

**WARNING:** This is me, finally finding the time to learn a bit of Rust and _Rust-best-practices_.
If I'm doing anything wrong, please make sure you yell at me... gently. :)

## TODO / Desirable features list

* ~~verbosity control~~
* ~~quiet mode (nothing written on STD_OUT if there are no errors)~~
* ~~help and version flags~~
* ~~meta information for --help coming from Cargo.toml (i.e. single source for binary metadata)~~
* ~~support for all http methods~~
* ~~default output to stdout~~
* disable gzip
* provide customer root certificate
* provide headers
* provide User Agent (specialized header)
* provide Cookies (specialized header)
* provide User Agent strings shorthands (i.e. 'macosx-chrome-60' or 'windows-edge')
* able to output to a given file
* multiple concurrent requests to download a file (if supported by target server)
* determine max redirection
* use IPv4 protocol (default)
* use IPv6 protocol
* skip HTTPS certificate verification
* skip download if output file already exists
* proxy configuration
* timeouts (connection / read)
* progress indicator (none, simple, advanced)
* search for mirros (RESEARCH: how do other clients like 'axel' do this?)

## License

[Apache License 2.0](./LICENSE) ([official page](https://www.apache.org/licenses/LICENSE-2.0))

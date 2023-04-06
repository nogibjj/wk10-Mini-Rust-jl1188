# wk10-mini-rust-jl1188:
A Rust Mini Project command-line tool that performs wildcard matching on two given strings. 
'?' Matches any single character.
'*' Matches any sequence of characters (including the empty sequence). 

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* [rust-new-project-template](https://github.com/noahgift/rust-new-project-template)
* [rust-marco-polo-example](https://github.com/noahgift/rust-mlops-template/tree/main/MarcoPolo)

## Usage
<code>cargo run -- match --s "aa" --p "a"</code>
 
 The command line tool uses the subcommand "<code>match</code>" and takes in two arguments, a first <code>String</code> named "<code>s</code>," and a second <code>String</code> named "<code>p</code>" for pattern. The result should be outputted as "Given strings match result: false". 


```
@selinaes ➜ /workspaces/wk10-Mini-Rust-jl1188/wildcard (main) $ cargo run -- match --s "aa" --p "a"
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/wildcard match --s aa --p a`
Given strings match result: false
@selinaes ➜ /workspaces/wk10-Mini-Rust-jl1188/wildcard (main) $ cargo run -- match --s "aa" --p "*"
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/wildcard match --s aa --p '*'`
Given strings match result: true
@selinaes ➜ /workspaces/wk10-Mini-Rust-jl1188/wildcard (main) $ cargo run -- match --s "cb" --p "?a"
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/wildcard match --s cb --p '?a'`
Given strings match result: false
```
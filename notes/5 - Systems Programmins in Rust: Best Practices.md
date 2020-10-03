# 5.1 Creating a Library and Organizing it into Modules
- myproject
    - src
        - bin
            - bar.rs 
            - baz.rs  <=== extra binary
        - lib.rs
        - mymodule
            - mysubmodule.rs
        - mymodule.rs
* this project doesn't need an extra binary
* create a stats submodule

# 5.2 Writing and Running Tests on Your Code
* same stuff as covered in Rust book

# 5.3 Documenting Your Code
* use `//!` to document the enclosing item 
    * only way to document a module since you can't place a comment outside a module
* `///` to document the next item right below it
* see your lib docs: `cargo doc --no-deps --open`

# 5.4 Understanding the Ins and Outs of Semantic Versioning
* major.minor.patch
* rust rule: <1.0.0 (0.minor.patch)
    * changing the minor number means a breaking change
    * changing patch numbers means whatever you want
* ruse rule: >= 1.0.0
    * same as normal semvar rules

# 5.5 Publishing Your Project as a Crate on crates.io
* check TOML file for fields to fills out before publishing
* create account on crates.io
* `cargo publish` to publish
* documentation is auto generated and published to docs.rs
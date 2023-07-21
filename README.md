# Arrowhead!
## An Obsidian Static Site Generator
### Written in Rust

### What this is 

This is a simple Obsidian static site generator written in Rust. In order to use this, just dump your obsidian vault into the `obsidian_project` directory. For any page specific styles, modify the `hash_map` in `obsidian_styles`.

### How to run

`cd` into the root project directory and `cargo run`. You should have rust installed. I haven't yet made the deployment code. I'm still looking for free hosting for Rust (which may have to include Docker).

### How to use it 

This is intentionally as simple a project as possible, so that people can take their Obsidian files and start making interactive online stories. Pictures in Obsidian folders are also automatically imported into the web pages. The pipe line is functional and first reads the files int `obsidian_project` directory and then puts images in `obsidian_img` and the text into `obsidian_html`. Files in `obsidian_html` are then modified by the `parse_file` functional pipeline. Most of the pipeline is simple `regex` find and replace commands. If you want to add another function into the pipeline it should be a simple matter of modifying `parse_file` to include a new function and then including that function in the `impl Parse`.

Routing is done using Rust Warp, which is a straightforward routing mechanism and is found at the bottom of the `main.rs` file. There are are also included web pages that are handled as single vue components that can be found in the `vue` folder. 

### Why I like it

I like it because it allows me to write a story that I've been working on, and organizing my thoughts in. If I can build out the site (with pictures even~) then I can make an interactive story website.

I didn't see a framework online that would be easy to use to turn Obsidian projects into online stories although it may be out there somewhere. This project is made as simple as possible so that anyone should be able to understand and modify the code.

### Other ideas

### A database maybe?

I don't know if I'll include a backend in the project. If so, it may be possible to do neat things like have someone write a key word in Obsidian that could then be used as a lookup in a Rust backend. For example someone may write in an Obsidian project `and then [x] number of elves road into Mordor` where `[x]` is the number of visitors to the site. This would be a sort of reverse templating function for non-secure or shared data. Given that the files are being parsed there could be even shareable fields such that someone could write `the Orcs wielded [weilded: halberds]` where someone could then write shareable data to their local database as a lookup for others (provided that databases were ever shared). I don't know that I want to turn this into an Active Record clone or a social network, but adding a database has some potential.

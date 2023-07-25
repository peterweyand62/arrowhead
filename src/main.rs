#![deny(warnings)]
use warp::{Filter};
use serde_json::json;
use std::path::Path;
use std::io;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use serde_json::Value;
use std::os::unix::fs::PermissionsExt;
use regex::Regex;
use serde_derive::{Serialize, Deserialize};
use warp_page::obsidian_styles::styles::specpag;
use warp_page::rust_state::state;
use scraper::{Html, Selector};
// use html5ever::{parse_document};
// use select::node::Node;
// use std::error::Error;
// use warp::reply::json;
// use warp::reply::Json;
// use warp::Reply;
// use serde::{Serialize, Serializer};
// use warp::Rejection;
// use warp::reply::Response;
// use warp::hyper::Body;
// use warp::http::StatusCode;

fn is_an_image(name: String) -> bool {
    println!("inside is_an_image and value of name: {:?}", name);
    let image_formats = vec![".jpg", ".jpeg", ".gif", ".pdf", ".svg", ".tiff", ".webp", ".png"];
    let mut is_an_image = false;
    for ext in image_formats{
        is_an_image = name.contains(ext);
        if is_an_image {
            break;
        }
    }
    is_an_image
}

struct Parse {
    contents: String
}

impl Parse {

    fn specific_page_styling(&self, entry_path: String) -> Self {
        println!("inside specific_page_styling and value of entry_path: {:?}", entry_path.clone());
        let mut p = Parse { contents: self.contents.clone() };
        for (key, value) in specpag().iter() {
            if entry_path.clone() == key.to_string() {
                let style_value = "<div style = '".to_string( )+ &value.clone() + "'>";
                let new_contents = style_value.to_string()+&self.contents+"</div>";
                println!("value of new_contents in key, value: {:?}", new_contents);
                p = Parse{contents: new_contents};
            }
        }
        p
    }

    fn carriage_return(&self) -> Self {
        let new_contents = self.contents.replace("\n", "<br/>");
        let p = Parse{contents: new_contents};
        p
    }

    #[allow(dead_code)]
    fn add_h2_to_page(&mut self) -> Self {
        let p = Parse { contents: self.contents.clone() };
        let re = Regex::new(r#"<br.>##\s+(.+)<br.>"#).unwrap();
        let text = p.contents.clone();
        let mut cleantext = p.contents.clone();
        for capture in re.captures_iter(&text) {
            let uncleaned_capture = &capture[0].to_string();
            let boldstring = format!(
                "<h2>{}</h2>",
                &capture[0].to_string().replace("##", "")
            );
            cleantext = cleantext.replace(uncleaned_capture, &boldstring);
        }
        let p = Parse { contents: cleantext };
        p
    }

    #[allow(dead_code)]
    fn add_h1_to_page(&mut self) -> Self {
        let p = Parse { contents: self.contents.clone() };
        let re = Regex::new(r#"<br.>\s*#([^<]+)<br.>"#).unwrap();
        let text = p.contents.clone();
        let mut cleantext = p.contents.clone();
        for capture in re.captures_iter(&text) {
            let uncleaned_capture = &capture[0].to_string();
            let boldstring = format!(
                "<h1>{}</h1>",
                &capture[0].to_string().replace("#", "")
            );
            cleantext = cleantext.replace(uncleaned_capture, &boldstring);
        }
        let p = Parse { contents: cleantext };
        p
    }

    #[allow(dead_code)]
    fn add_bold_text_to_page(&mut self) -> Self {
        let p = Parse { contents: self.contents.clone() };
        let re = Regex::new(r"(\*\*)(.*?)(\*\*)").unwrap();
        let text = p.contents.clone();
        let mut cleantext = p.contents.clone();
        for capture in re.captures_iter(&text) {
            let uncleaned_capture = &capture[0].to_string();
            let boldstring = format!(
                "<span style='font-weight: bold;'>{}</span>",
                &capture[0].to_string().replace("**", "")
            );
            cleantext = cleantext.replace(uncleaned_capture, &boldstring);
        }
        let p = Parse { contents: cleantext };
        p
    }

    #[allow(dead_code)]
    fn add_italic_text_to_page(&mut self) -> Self {
        let p = Parse { contents: self.contents.clone() };
        let re = Regex::new(r"\*(.*?)\*").unwrap();
        let text = p.contents.clone();
        let mut cleantext = p.contents.clone();
        for capture in re.captures_iter(&text) {
            let uncleaned_capture = &capture[0].to_string();
            let boldstring = format!(
                "<span style='font-style: italic;'>{}</span>",
                &capture[0].to_string().replace("*", "")
            );
            cleantext = cleantext.replace(uncleaned_capture, &boldstring);
        }
        let p = Parse { contents: cleantext };
        p
    }

    #[allow(dead_code)]
    fn add_picture_to_page(&mut self) -> Self{
        let p = Parse{contents: self.contents.clone()};
        let re = Regex::new(r"!\[\[([^\[\[]+)\]\]").unwrap();
        let text = p.contents.clone();
        let mut cleantext = p.contents.clone();
        for capture in re.captures_iter(&text){
            let uncleaned_capture = &capture[0].to_string();
            let cleaned_capture = &capture[0].to_string().replace("[", "").replace("]", "").replace("!", "");
            let imagestring = [
                "<img src='/img/", 
                cleaned_capture, 
                "'/>"
            ].join("");
            cleantext = cleantext.replace(uncleaned_capture, &imagestring);
        }
        let p = Parse{contents: cleantext};
        p
    }

    #[allow(dead_code)]
    fn link_to_another_page(&mut self) -> Self{
        let p = Parse{contents: self.contents.clone()};
        let re = Regex::new(r"\[\[([^\[\[]+)\]\]").unwrap();
        let text = p.contents.clone();
        let mut cleantext = p.contents.clone();
        for capture in re.captures_iter(&text){
            let uncleaned_capture = &capture[0].to_string();
            let cleaned_capture = &capture[0].to_string().replace("[", "").replace("]", "");
            if cleaned_capture.find("#").is_none() && !is_an_image(cleaned_capture.to_string()) {
                let (anchor_visible_name, anchor_href) = match cleaned_capture.match_indices("|").find_map(|(i, _val)| Some(i)) {
                    Some(cleaned_text_index) => {
                        println!("the value of res: {:?}", cleaned_capture);
                        let anchor_visible_name = cleaned_capture.get(cleaned_text_index+1..cleaned_capture.len()).unwrap();
                        let anchor_href = cleaned_capture.get(0..cleaned_text_index).unwrap();
                        (anchor_visible_name.to_string(), anchor_href.to_string())
                        
                    }, 
                    None => {
                        println!("character | was not found");
                        (cleaned_capture.to_string(), cleaned_capture.to_string())
                    }
                };
                let anchor_string = [
                    "<a href='/html/", 
                    &anchor_href,
                    ".html'/>", 
                    &anchor_visible_name,
                    "</a>"
                ].join("");
                cleantext = cleantext.replace(uncleaned_capture, &anchor_string);
            }
            if cleaned_capture.find("#^") > Some(0){
                let _cleaned_text = cleaned_capture;
                let page_ref: Vec<&str> = cleaned_capture.split("#^").collect();
                let page_display: Vec<&str> = cleaned_capture.split("|").collect();
                println!("value of page_ref: {:?}", page_ref.get(0).unwrap());
                println!("value of page_display: {:?}", page_display.get(1).unwrap());
                let anchor_string = [
                    "<a href='/html/", 
                    &page_ref.get(0).unwrap(),
                    ".html'/>", 
                    &page_display.get(1).unwrap(),
                    "</a>"
                ].join("");
                cleantext = cleantext.replace(uncleaned_capture, &anchor_string);
                // make_site_map(
                //     entry_path.clone().to_string(),
                //     "/html/".to_string + &page_ref.get(0).unwrap(),
                //     "".to_string() + &page_display.get(1).unwrap()
                // );
            }
        }
        let p = Parse{contents: cleantext};
        p
    }
}

fn create_page(entry_path: String, contents: String){
   let new_path = entry_path
       .replace("obsidian_project", "obsidian_html")
       .replace(".md", ".html");
   let mut file = fs::File::create(new_path).unwrap();
   file.write_all(contents.as_bytes()).unwrap();
}

fn store_images(entry_path:String){
    let new_path = entry_path
        .replace("obsidian_project", "obsidian_img");
    let _result = fs::copy(entry_path, new_path);
}

fn parse_file(entry_path: String){
    println!("value of entry_path in parse_file: {:?}", entry_path);
    if !is_an_image(entry_path.clone().to_string()){
        let contents = fs::read_to_string(entry_path.clone())
            .expect("Should have been able to read the file");
        let parsing_contents = Parse{contents: contents};
        let parsed_contents = parsing_contents
            .specific_page_styling(entry_path.clone())
            .add_bold_text_to_page()
            .add_italic_text_to_page()
            .carriage_return()
            .add_h2_to_page()
            .add_h1_to_page()
            .add_picture_to_page()
            .link_to_another_page();
        create_page(entry_path.clone(), parsed_contents.contents.clone());
    }else{
        store_images(entry_path.clone());
    }
}

fn set_permissions(path: String){
    let mut permissions = fs::metadata(&path)
        .unwrap()
        .permissions();

    permissions.set_mode(0o777);

    fs::set_permissions(&path, permissions)
        .unwrap();
}

fn read_files(){
    let path = Path::new("./src/obsidian_project");
    match fs::remove_dir_all("./src/obsidian_html"){
        Ok(x) => println!("remove_dir_all: {:?}", x), 
        Err(x) => println!("there was an error in remove_dir_all {:?}", x)
    }
    match fs::remove_dir_all("./src/obsidian_img"){
        Ok(x) => println!("remove dir_all: {:?}", x),
        Err(x) => println!("there was an error in remove_dir_all {:?}", x)
    }
    fs::create_dir_all("./src/obsidian_html").unwrap();
    fs::create_dir_all("./src/obsidian_img").unwrap();

    set_permissions("./src/obsidian_html".to_string());
    set_permissions("./src/obsidian_img".to_string());
    
    for entry in fs::read_dir(path).expect("Unable to list") {
        let entry = entry.expect("unable to get entry");
        parse_file(entry.path().display().to_string());
    }
}

fn get_doc_list() -> warp::reply::Json {
    let paths_html = fs::read_dir("./src/obsidian_html").unwrap();
    let mut html_vec: Vec<String> = vec![];
    for path in paths_html {
        html_vec.append(&mut vec![path.unwrap().path().display().to_string()]);
    }
    let response_json = json!({
        "data": html_vec
    });
    warp::reply::json(&response_json)
}

#[allow(dead_code)]
fn make_site_map(mut site_map: state::VecSiteLinks) -> state::VecSiteLinks{
    println!("inside make_site_map");
    println!("this is the value of the site_map: {:?}", site_map);
    let paths_html = fs::read_dir("./src/obsidian_html").unwrap();
    for path in paths_html {
        let cleaned_path = path.unwrap().path().display().to_string();
        let contents = fs::read_to_string(cleaned_path.clone())
            .expect("Should have been able to read the file");
        let document = Html::parse_document(&contents);
        let selector = Selector::parse("a").unwrap();
        for element in document.select(&selector) {
            println!("inner html: {:?}", element.inner_html());
            println!("href: {:?}", element.value().attr("href").expect("attr not found").to_string());
            site_map.add_vec(state::SiteLinks{
                path: cleaned_path.to_string(), 
                href: element.value().attr("href").expect("attr not found").to_string(),
                disp: element.inner_html()
            });
        }
    }
    site_map.clone()
}    


fn get_site_map() -> warp::reply::Json {
    let mut site_map = state::VecSiteLinks{
        links: vec![]
    };
    site_map = make_site_map(site_map);
    println!("value of site_map in get_site_map: {:?}", site_map);
    let outgoing_json = json!({
        "site_map": site_map
    });
    warp::reply::json(&outgoing_json)
}

fn return_html_page(data: Value) -> warp::reply::Json {
    let incoming_json = json!({
        "name": data["name"],
        "path": data["path"]
    });
    println!("value of response_json: {:?}", incoming_json["path"]);
    let contents = fs::read_to_string(incoming_json["path"].as_str().unwrap())
        .expect("Should have been able to read the file");
    let outgoing_json = json!({
        "name": data["name"],
        "html": contents
    });
    warp::reply::json(&outgoing_json)
}

fn update_html_page(data: Value) -> warp::reply::Json {
    let incoming_json = json!({
        "html": data["html"], 
        "path": data["path"]
    });
    let path = incoming_json["path"].as_str().unwrap();
    let html = incoming_json["html"].as_str().unwrap();
    println!("value of path: {:?}", path.clone());
    println!("value of html: {:?}", html.clone());

    let path = Path::new(path.clone());
    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
    };

    match file.write_all(html.clone().as_bytes()) {
        Ok(_) => println!("Successfully wrote to the file."),
        Err(why) => panic!("couldn't write to {}: {}", path.display(), why),
    }

    let contents = fs::read_to_string(path.clone())
    .expect("Should have been able to read the file");

    drop(file);

    let outgoing_json = json!({
        "html": contents
    });
    warp::reply::json(&outgoing_json)
}



#[derive(Deserialize, Serialize)]
struct Employee {
    name: String,
}

#[tokio::main]
async fn main() {

    let mut input = String::new();

    let mut inputtest = "-".to_string();
    let mut firstinput = false;

    //I should have multiple options here so that I can also read the .md files
    //that are new and not replace any that are already in the system - 
    //this would require more work as I would have to alter the above pipeline.
    while inputtest != "y".to_string() && inputtest != "n".to_string() {
        if firstinput == false{
            firstinput = true;
            println!("Would you like to read the .md files and replace the html? (y/n):");
        }else{
            input = "".to_string();
            println!("Please enter only y or n as input:");
        }
        io::stdin().read_line(&mut input).expect("Failed to read line");
        println!("You entered: {}", input.to_string());
        inputtest = input.to_string().replace("\n", "");
        println!("value of inputtest: {:?}", inputtest.to_string());
        if inputtest.to_string() == "y".to_string() {
            read_files();
        }
    } 

    // println!("value of computed_site_map: {:?}", site_map);

    let paths_html = fs::read_dir("./src/obsidian_html").unwrap();
    let paths_img = fs::read_dir("./src/obsidian_img").unwrap();

    let mut html_vec: Vec<String> = vec![];
    let mut img_vec:  Vec<String> = vec![];

    for path in paths_html {
        html_vec.append(&mut vec![path.unwrap().path().display().to_string()]);
    }

    for path in paths_img {
        img_vec.append(&mut vec![path.unwrap().path().display().to_string()]);
    }

    println!("value of html_vec {:?}", html_vec.clone());
    println!("value of img_vec  {:?}", img_vec.clone());

    let hi = warp::path("hi").map(|| "Hello, World!");
    let home_page = warp::get().and(warp::fs::dir("src/vue/home"));
    let about = warp::path("about").and(warp::fs::dir("src/vue/about"));
    let html = warp::path("html").and(warp::fs::dir("src/obsidian_html/"));
    let img = warp::path("img").and(warp::fs::dir("src/obsidian_img/"));
    let vue_img = warp::path("vue_img").and(warp::fs::dir("src/vue/assets/images"));
    let editor = warp::path("editor").and(warp::fs::dir("src/vue/editor"));
    let get_doc_list = warp::path("get_doc_list").map(|| get_doc_list());
    let get_site_map = warp::path("get_site_map").map(|| {
        get_site_map()
    });
    let return_html_page = warp::path!("return_html_page")
        .and(warp::post())
        .and(warp::body::json())
        .map(|data: Value| {
            return_html_page(data)
        });
    let update_html_page = warp::path!("update_html_page")
        .and(warp::post())
        .and(warp::body::json())
        .map(|data: Value| {
            update_html_page(data)
        });

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "DELETE"]);

    let routes = warp::any().and(
        home_page
        .or(hi)
        .or(html)
        .or(editor)
        .or(img)    
        .or(vue_img)
        .or(about)
        .or(get_doc_list)
        .or(return_html_page)
        .or(update_html_page)
        .or(get_site_map)
        .with(cors)
    );

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

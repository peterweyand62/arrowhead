#![deny(warnings)]
use warp::Filter;
use std::path::Path;
use std::fs;
use std::io::prelude::*;
use regex::Regex;
use warp_page::obsidian_styles::styles::specpag;

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
    for entry in fs::read_dir(path).expect("Unable to list") {
        let entry = entry.expect("unable to get entry");
        parse_file(entry.path().display().to_string());
    }
}


#[tokio::main]
async fn main() {

    read_files();
    
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
    let routes = warp::get().and(
        home_page
        .or(hi)
        .or(html)
        .or(img)
        .or(vue_img)
        .or(about)
    );

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

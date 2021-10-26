extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use std::convert::TryInto;

pub fn printing(text: &str) -> String {
    println!("{}",text);
    return format!("{}",text);
}

pub fn get_html_from_website(url: &str) -> String {
    let mut res = reqwest::get(url).unwrap();
    assert!(res.status().is_success());
    let html = res.text().unwrap();
    let html_str = String::from(html);
    return html_str
}

pub fn get_country_data_in_html(html_str: &str){
    let document = Document::from_read(html_str.as_bytes()).unwrap();
    let table = document.find(Name("table").descendant(Name("a")));
    for node in table{
        let link_text = node.text();
        println!("{} ({:?})", link_text,node.attr("href").unwrap().to_string());
        }
}

pub fn get_university_data_in_country_page(html_str: &str){
    let mut university_data = vec!["string_to_initilize_vector"];
    let document = Document::from_read(html_str.as_bytes()).unwrap();
    let table = document.find(Name("table"));
    for node in table{
        let mut link_text = node.text();
        
        println!("Before replace: {}",link_text);
        println!("\n\nAfter trim: {}",link_text.replace("   ",""));
        link_text = link_text.replace("   ","");
        //let mut tokens: Vec<&str> = link_text;
        /*
        let mut to_drop = vec![0,1];
        let length = tokens.len(); //defining this variable because the for loop needs it and cant check len mid loop in case the len changes
        println!("{}",length);
        for (index,token) in tokens.iter().enumerate(){
            if index < length-1 && tokens[index+1] == ""   { //checks if next value is empty, in which case in can be droped
                to_drop.push(index);
            }
        }

        for i in to_drop{
            /*removing all the useless* whitespace from vector
            uselss meaning its immediatly followed by more whitespace, whitespace the is followed by a letter (which makes text more readable) remains*/
            tokens.remove(i);
            }
        }
         
        println!("beep");
        for i in tokens{
            println!("{:?}",i);
        }
       */
            
        
    }
}
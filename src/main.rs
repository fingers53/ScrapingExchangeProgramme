mod functions;

fn main(){
    functions::printing("I wish this was python");

    let country_index_url: &str = "https://www.ucol.mx/relaciones-internacionales/a-donde-ir-internacional.htm";
    //let textus = functions::get_html_from_website(country_index_url);
    //functions::get_country_data_in_html(&textus);
    

    let example_country_url: &str = "https://www.ucol.mx/relaciones-internacionales/peru.htm";
    let country_html = functions::get_html_from_website(example_country_url);
    functions::get_university_data_in_country_page(&country_html);
    
    // write to file
}
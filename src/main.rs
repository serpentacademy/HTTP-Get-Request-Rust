extern crate reqwest;
//use std::collections::HashMap;
use futures::executor::block_on;
async fn post_request(){
    match reqwest::blocking::get("https://news.ycombinator.com/"){
        Ok(mut response) => {
            if response.status() == reqwest::StatusCode::OK{
                match response.text(){
                    Ok(text) => println!("response: {}", text),
                    Err(_) => println!("could not read response")
                }
            }else{
                println!("response was not 200 ok");
            }
    
        }
        Err(_) => println!("could not make the request  ")
    }

}
fn main() {
    let future = post_request(); // Nothing is printed
    block_on(future); // `future` is run and "hello, world!" is printed
//post_request();




    
}


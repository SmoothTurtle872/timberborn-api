use reqwest::blocking::Response;
use reqwest;
use urlencoding::encode;

use serde_json::{Error};

#[derive(Debug, serde::Deserialize)]
pub struct Lever {
    pub name: String,
    state: bool,
    springReturn: bool
}

// create
impl Lever {
    pub fn new(name: String, state: bool, spring_return: bool) -> Lever{
        Lever { name, state, springReturn: spring_return }
    }

    pub fn from_json(json: String) -> Result<Lever, Error>{
        let lever = serde_json::from_str(&json)?;
        Ok(lever)
    }

    pub fn from_json_array(json: String) -> Result<Vec<Result<Lever,Error>>,Error>{
        todo!()
    }
    
}

// get & set
impl Lever{
    pub fn state(&self) -> bool{
        self.state
    }

    pub fn set_state(&mut self, state: bool) -> Result<Response, &str> {
        let url: String;
        if state{
            url = format!("http://localhost:8080/api/switch-on/{}", encode(&self.name));
        }
        else {
            url = format!("http://localhost:8080/api/switch-off/{}", encode(&self.name));
        }
        let resp = reqwest::blocking::get(url);
        match resp {
            Ok(x) => {
                if !self.springReturn{
                self.state = state;
            }
                Ok(x)
            },
            Err(_) => Err("Found Error")
        }

    }

    pub fn toggle(&mut self) -> Result<Response, &str>{
        self.set_state(!self.state)
    }

    pub fn set_color(&self, color: String) -> Result<Response, &str>{
        let url = format!("http://localhost:8080/api/color/{}/{}", encode(&self.name), color);
        let resp = reqwest::blocking::get(url);
        match resp {
            Ok(x) => Ok(x),
            Err(_) => Err("Found Error")
        }
    }

    pub fn spring_return(&self) -> bool{
        self.springReturn
    }
}

impl PartialEq for Lever{
    fn eq(&self, other: &Self) -> bool {
        if self.name == other.name && self.springReturn == other.springReturn && self.state == other.state {
            return true
        }
        else{
            return false
        }
    }
}



#[cfg(test)]
mod api_tests {
    use super::*;
    
    // IMPORTANT - when testing, you must have a http lever named 'rust-test'
    #[ignore]
    #[test]
    fn turn_on() {
        let mut lever = Lever::new(String::from("rust-test"), false, false);
        let response = lever.set_state(true).unwrap().status();
        assert!(response == reqwest::StatusCode::OK && lever.state)
    }

    #[test]
    #[ignore]
    fn turn_off() {
        let mut lever = Lever::new(String::from("rust-test"), false, false);
        let response = lever.set_state(false).unwrap().status();
        assert!(response == reqwest::StatusCode::OK && !lever.state)
    }

    #[test]
    #[ignore]
    fn turn_on_spring_return() {
        let mut lever = Lever::new(String::from("rust-test"), false, true);
        let response = lever.set_state(true).unwrap().status();
        assert!(response == reqwest::StatusCode::OK && !lever.state)
    }

    #[test]
    #[ignore]
    fn set_color() {
        let lever = Lever::new(String::from("rust-test"), false, false);
        let response = lever.set_color(String::from("ff2233")).unwrap().status();
        assert!(response == reqwest::StatusCode::OK)
    }

}

#[cfg(test)]
mod non_api_tests{
    use super::*;
    #[test]
    fn load_json() {
        let lever = Lever::from_json(String::from("{\"name\":\"test\",\"state\":false,\"springReturn\":false}"));
        assert!(lever.unwrap() == Lever::new(String::from("test"), false, false))
    }
}

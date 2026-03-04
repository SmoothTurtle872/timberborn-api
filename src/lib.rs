use std::error::Error;

use reqwest::blocking::Response;
use reqwest;

pub struct Lever {
    pub name: String,
    state: bool,
    spring_return: bool
}

// create
impl Lever {
    pub fn new(name: String, state: bool, spring_return: bool) -> Lever{
        Lever { name, state, spring_return }
    }
}

// get & set
impl Lever{
    pub fn state(&self) -> bool{
        self.state
    }

    pub fn set_state(&mut self, state: bool) -> Result<Response, &str> {
        use urlencoding::encode;

        let url: String;
        if state{
            url = format!("http://localhost:8080/api/switch-on/{}", encode(&self.name));
        }
        else {
            url = format!("http://localhost:8080/api/switch-off/{}", encode(&self.name));
        }
        let resp = reqwest::blocking::get(url);
        match resp {
            Ok(x) => Ok(x),
            Err(_) => Err("Found Error")
        }
        


    }

    pub fn spring_return(&self) -> bool{
        self.spring_return
    }
}




#[cfg(test)]
mod tests {
    use super::*;

}

use reqwest::blocking::Response;
use reqwest;
use urlencoding::encode;

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
                if !self.spring_return{
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
        self.spring_return
    }
}




#[cfg(test)]
mod tests {
    use super::*;

}

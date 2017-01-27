extern crate rustc_serialize;

#[derive(RustcEncodable, RustcDecodable)]
pub struct SignUpInput {
    pub email: String,
    pub username: String,
    pub password: String
}

impl SignUpInput {
    pub fn valid(&self) -> Result<(), String> {
        try!(self.email_valid());
        try!(self.username_valid());
        try!(self.password_valid());
        Ok(())
    }

    pub fn email_valid(&self) -> Result<(), String> {
        let email = &self.email;
        if email.len() <= 5 || email.len() >= 30 {
            return Err("Email needs to be between 5 and 30 characters".into())
        }
        Ok(())
    }

    pub fn username_valid(&self) -> Result<(), String> {
        let username = &self.username;
        if username.len() <= 4 || username.len() >= 20 {
            return Err("Username needs to be between 4 and 20 characters".into())
        }
        Ok(())
    }

    pub fn password_valid(&self) -> Result<(), String> {
        let password = &self.password;
        if password.len() <= 8 || password.len() >= 32 {
            return Err("Password needs to be between 8 and 32 characters".into()) 
        }
        Ok(())
    }
}

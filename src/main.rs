use serde::Deserialize;
use reqwest::Error;


#[derive(Deserialize, Debug)]
struct Geo {
    lat: String,
    lng: String
}
#[derive(Deserialize, Debug)]
struct Address {
    street: String,
    suite: String,
    city: String,
    zipcode: String,
    geo: Geo
}

#[derive(Deserialize, Debug)]
struct Company {
    name: String,
    catchPhrase: String,
    bs: String
}
#[derive(Deserialize, Debug)]
struct Users {
    id: i32,
    name: String,
    username: String,
    email: String,
    address: Address,
    phone: String,
    website: String,
    company: Company
}


#[tokio::main]
async fn main() -> Result<(), Error> {
let response = reqwest::get("https://jsonplaceholder.typicode.com/users").await?;


    if response.status() == reqwest::StatusCode::OK{
    println!("Status: {}", response.status());
    let users: Vec<Users> = response.json().await?;

    for user in &users {
        println!("{:#?}", user.address);
    }
    println!("{:?}", &users);
    } else {
        println!("Error fetching users")
    }
   
    Ok(())
}

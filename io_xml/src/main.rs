use std::{fs, io};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
struct Users {
    user: Vec<User>
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
struct User {
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@age")]
    age: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<Address>,
    
    phones: Phones
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
struct Address {
    #[serde(rename = "@street")]
    street: String,

    #[serde(rename = "@city")]
    city: String
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
struct Phones {    
    phone: Vec<String>
}

fn main() {
    let dir = "data";
    let file_name = "users.xml";
    let path = format!("{dir}/{file_name}");

    let users = Users {
        user: vec![
            User {
                name: "John Doe".to_owned(),
                age: 43,
                address: Some(
                    Address {
                        street: "10 Downing Street".to_owned(),
                        city: "London".to_owned()
                    }
                ),
                phones: Phones {
                    phone: vec![
                        "+44 1234567".to_owned(),
                        "+44 2345678".to_owned()
                    ]
                }
            },
            User {
                name: "William Schumacher".to_owned(),
                age: 42,
                address: None,
                phones: Phones {
                    phone: vec![
                        "+44 3456789".to_owned(),
                        "+44 4567899".to_owned()
                    ]
                }
            }
        ]
    };

    let res = write_file(&users, dir, file_name);

    println!("Result of writing: {:#?}", res);    

    let res = read_file(&path);

    println!(
        "Result of reading: {:#?}", 
        res
    );
}

fn write_file(users: &Users, dir: &str, file_name: &str) -> io::Result<()> {
    fs::create_dir_all(dir)?;

    let mut file = fs::File::create(
        format!("{dir}/{file_name}")
    )?;

    let mut data = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n".to_owned();

    let res = quick_xml::se::to_writer(&mut data, users);

    match res {
        Ok(_) => {
            io::Write::write_all(&mut file, data.as_bytes())?;

            Ok(())
        },
        Err(err) => Err(io::Error::new(io::ErrorKind::Other, err))
    }  
}

fn read_file(path: &str) -> io::Result<Users> {
    let data = fs::read_to_string(path)?;

    let res = quick_xml::de::from_str(&data);

    match res {
        Ok(users) => Ok(users),
        Err(err) => Err(io::Error::new(io::ErrorKind::Other, err))
    }
}

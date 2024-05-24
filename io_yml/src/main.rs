use std::{fs, io};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct User {
    name: String,
    age: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<Address>,
    
    phones: Vec<String>
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Address {
    street: String,
    city: String
}

fn main() {
    let dir = "data";
    let file_name = "users.yml";
    let path = format!("{dir}/{file_name}");

    let users = vec![
        User {
            name: "John Doe".to_owned(),
            age: 43,
            address: Some(
                Address {
                street: "10 Downing Street".to_owned(),
                    city: "London".to_owned()
                }
            ),
            phones: vec![
                "+44 1234567".to_owned(),
                "+44 2345678".to_owned()
            ]
        },
        User {
            name: "William Schumacher".to_owned(),
            age: 42,
            address: None,
            phones: vec![
                "+44 3456789".to_owned(),
                "+44 4567899".to_owned()
            ]
        }
    ];

    let res = write_file(&users, dir, file_name);

    println!("Result of writing: {:#?}", res);    

    let res = read_file(&path);

    println!(
        "Result of reading: {:#?}", 
        res
    );    
}

fn write_file(users: &Vec<User>, dir: &str, file_name: &str) -> io::Result<()> {
    fs::create_dir_all(dir)?;

    let mut file = fs::File::create(
        format!("{dir}/{file_name}")
    )?;

    let res = serde_yaml::to_string(users);

    match res {
        Ok(data) => {
            io::Write::write_all(&mut file, data.as_bytes())?;

            Ok(())
        },
        Err(err) => Err(io::Error::new(io::ErrorKind::Other, err))
    }
}

fn read_file(path: &str) -> io::Result<Vec<User>> {
    let data = fs::read_to_string(path)?;

    let res = serde_yaml::from_str::<Vec<User>>(&data);

    match res {
        Ok(users) => Ok(users),
        Err(err) => Err(io::Error::new(io::ErrorKind::Other, err))
    }
}

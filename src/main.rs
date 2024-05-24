use std::{fs, io};

fn main() {
    let dir = "data";
    let file_name = "emily_dickinson.txt";
    let path = format!("{dir}/{file_name}");

    let res = write_file(dir, file_name);   

    println!("Result of writing: {:?}", res);    

    let res = read_file(path.as_str());

    println!(
        "Result of reading: {}", 
        if res.is_ok() {"success"} else {"error"}
    );

    if let Ok(content) = res {
        println!("Content: ");
        println!("{content}");
    }
}

fn write_file(dir: &str, file_name: &str) -> io::Result<()> {
    let poem = "
        I'm nobody! Who are you?
        Are you nobody, too?
        Then there's a pair of us - don't tell!
        They'd banish us, you know.
        
        How dreary to be somebody!
        How public, like a frog
        To tell your name the livelong day
        To an admiring bog!
    ";

    fs::create_dir_all(dir)?;

    let mut file = fs::File::create(
        format!("{dir}/{file_name}")
    )?;

    io::Write::write_all(&mut file, poem.as_bytes())    
}

fn read_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

struct Foo {
    name: String,
}

fn main() {
    let status = req_status();

    match status {
        200 => println!("ok"),
        400 => println!("bad request"),
        other => {
            println!("request failed with {} status", other);

            // get response from cache
        }
    }

    let foo = Foo {
        name: "mohammad".to_string(),
    };

    let rs = f(Some(&foo));
    match rs {
        Err(rs) => println!("{}", rs),
        Ok(rs) => println!("{}", rs),
    }

    let rs2 = f(None);
    match rs2 {
        Err(rs2) => println!("{}", rs2),
        Ok(rs2) => println!("{}", rs2),
    }
}

fn req_status() -> u32 {
    200
}

fn f(ptr: Option<&Foo>) -> Result<String, i64> {
    match ptr {
        Some(ptr) => return Ok(ptr.name.to_string()),
        None => return Err(10),
    }
}

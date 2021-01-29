fn main() {
    let status = req_status();

    match status {
        200 => println!("ok"),
        400 => println!("bad request"),
        other => {
            println!("request failed with {} status", other);

            // get reqponse from cache
        }
    }
}

fn req_status() -> u32 {
    200
}

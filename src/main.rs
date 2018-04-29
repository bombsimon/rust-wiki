use std::env;

extern crate reqwest;
extern crate json;

fn lookup(topic: String) -> Result<(), reqwest::Error> {
    let url: String = format!(
        "https://{}.wikipedia.org/w/api.php?format={}&action={}&prop={}&ppprop={}&plnamespace=0&exlimit=1&exsentences=1&explaintext=1&exintro=1&redirects=1&titles={}",
        "en", "json", "query", "extracts|links|pageprops", "disambiguation", topic
    );

    let text        = reqwest::get(&url)?.text()?;
    let parsed      = json::parse(&text).unwrap();
    let mut content = json::JsonValue::new_object();

    for (_, v) in parsed["query"]["pages"].entries() {
        content = v.clone();
        break;
    }

    println!("{}", content["title"]);
    println!("{}", content["extract"]);

    Ok(())
}

fn help() {
    println!("Usage: {} <topic>", env::args().next().expect("No application name?"));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let topic = args[1].parse().unwrap();
        lookup(topic).unwrap();
    }
    else {
        help();
    }
}

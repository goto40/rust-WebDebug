#[macro_use]
extern crate rocket;
extern crate webdebug;
use rocket::response::content::Html;

#[get("/")]
fn index() -> Html<String> {
    command("help")
}

#[get("/command?<command_text>")]
fn command(command_text: &str) -> Html<String> {
    let mut split = command_text.split(" ");
    let cmd = split.next().unwrap();
    let params = split.map(|x|{x.to_string()}).collect();
    Html(format!(
        r#"
        <html>
            <title>webdebug '{}'</title>
            <body>
                <div>
                <form action="command" id="command" method="get">
                    <label class="h2" form="command">Command:</label>
                    <input type="command_text" name="command_text" id="command_text" value="{}">
                    <button type="submit">submit command</button>
                </form>                        
                </div>
            {}
            </body>
        </html>
        "#,
        command_text,
        command_text,
        webdebug::exec_command(&webdebug::Flavor::HTML,cmd,params)
    ))
}

fn demo(_params: Vec<String>) -> String {
    "This is a 🌶 demo".to_owned()
}

#[launch]
fn rocket() -> _ {
    webdebug::add_command(
        "demo",
        "demo command",
        "a simple demo command",
        webdebug::CommandFunction::TextFn(Box::new(demo)),
    )
    .unwrap();

    webdebug::add_command(
        "demo2",
        "demo2 command",
        "a simple demo command",
        webdebug::CommandFunction::TextFn(Box::new(|_params:Vec<String>|{"This is a 🌶 demo".to_owned()})),
    )
    .unwrap();

    let config = rocket::config::Config {
        port: 3000,
        ..Default::default()
    };
    rocket::custom(config).mount("/", routes![index, command])
}

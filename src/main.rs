mod shiolink;
use shiolink::protocol::Protocol as ShiolinkProtocol;

mod shiori;

mod parse_source;
use parse_source::StdInParseSource;

mod felis;
use felis::ghost::Ghost;
use felis::Felis;

struct MyGhost {}
impl Ghost for MyGhost {
    fn request(&self, event_id: &str, references: Vec<&str>) -> String {
        return "¥w¥w¥w¥wtest str".to_string();
    }
}

fn main() {
    let parse_source = StdInParseSource::new();

    loop {
        let my_ghost = MyGhost {};
        let felis = Felis::new(&my_ghost);

        let req: ShiolinkProtocol = shiolink::protocol::parse(&parse_source);
        match req {
            ShiolinkProtocol::Load { load_dir } => {
                println!("[Load] loaddir:{load_dir}");
                felis.load(&load_dir);
            }
            ShiolinkProtocol::Sync { sync_str } => {
                println!("[Sync] syncstr:{sync_str}");
            }
            ShiolinkProtocol::Request(body) => {
                println!("{}", felis.request(body));
            }
            ShiolinkProtocol::Unload => {
                println!("[Unload]");
                felis.unload();
                break;
            }
            ShiolinkProtocol::ParseError { reason } => {
                println!("[Error] reason: {reason}");
                break;
            }
            ShiolinkProtocol::Empty => {
                // do nothing.
            }
        }
    }
}

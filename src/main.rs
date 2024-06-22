use std::net::TcpListener;

use clap::Parser;
use pizza_dough_calculator::pizza_dough::PizzaDough;
use pizza_dough_calculator::startup::configure_server;

#[derive(Parser, Debug)]
#[command(author, version,  about, long_about=None)]
struct Args {
    ///Number of pizza[s] you want to make
    #[arg(short, long, default_value_t = 2.0)]
    portions: f32,

    ///Size of pizza[s] you want to make (S, M, L , XL)
    #[arg(short, long, default_value_t=String::from("M"))]
    size: String,

    ///Type of yeast you want to use ([F]resh, [D]ry, [L]ievito Madre)
    #[arg(short, long, default_value_t=String::from("F"))]
    yeast: String,

    ///Start serving pizza dough calculation via http api
    #[arg(long)]
    serve: bool,

    ///Port to bind to
    #[arg(long, default_value_t = 8080)]
    port: u16,

    ///Host IP Address
    #[arg(long, default_value_t = String::from("127.0.0.1"))]
    host: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    tracing_subscriber::fmt::init();

    if args.serve {
        let address = format!("{}:{}", args.host, args.port);
        let listener = TcpListener::bind(address.clone())
            .unwrap_or_else(|_| panic!("Failed to bind on {}", &address));
        configure_server(listener)?.await
    } else {
        let pd = PizzaDough::new(args.portions, args.size, args.yeast);
        pd.printout();
        Ok(())
    }
}

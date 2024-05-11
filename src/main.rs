use actix_web::{App, HttpServer};
use clap::Parser;
use pizza_dough_calculator::pizza_dough::PizzaDough;
use pizza_dough_calculator::routes::health_check;

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
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    match args.serve {
        false => {
            let pd = PizzaDough::new(args.portions, args.size, args.yeast);
            pd.printout();
            Ok(())
        }
        true => {
            println!("Starting to serve Pizza via Http");

            HttpServer::new(|| App::new().service(health_check))
                .bind(("127.0.0.1", 8080))?
                .run()
                .await
        }
    }
}

use clap::Parser;
use pizza_dough_calculator::PizzaDough;

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
}

fn main() {
    let args = Args::parse();

    let pd = PizzaDough::new(args.portions, args.size, args.yeast);
    pd.printout();
}

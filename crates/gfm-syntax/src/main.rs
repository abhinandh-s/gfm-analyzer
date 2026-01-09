use gfm_syntax::*;

fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let source = EXAMPLE_38;

        println!("{source}");
    println!("{}", cst!(source).display());
}

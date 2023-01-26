use once_cell::sync::Lazy;
use tera::Tera;

pub(crate) static TEMPLATES: Lazy<Tera> = Lazy::new(|| {
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    tera
});

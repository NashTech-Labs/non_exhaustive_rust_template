extern crate movie_genres;
use movie_genres::Genres;

fn main() {
    let genres = Genres::Horror;

    match genres {
        Genres::Horror => println!("Horror Movie!!"),
        Genres::Comedy => println!("Comedy Movie!!"),
        Genres::Romance => println!("Romance Movie!!"),
        _ => println!("Other movie category"),
    }
}

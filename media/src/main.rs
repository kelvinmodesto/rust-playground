#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Manga { title: String, author: String },
    Moovie { title: String, director: String },
    AudioBook { title: String },
}

impl Media {
    fn description(&self) -> String {
        // match self {}
    }
}

fn print_media(media: Media) {
    println!("{:#?}", media);
}
fn main() {
    let manga = Media::Manga {
        title: String::from("One Piece"),
        author: String::from("Eichiro Oda"),
    };

    let movie = Media::Moovie {
        title: String::from("Matrix"),
        director: String::from("Watchowsky Sisters"),
    };

    let book = Media::Book {
        title: String::from("The Foundation"),
        author: String::from("Isaac Azimov"),
    };

    let audio_book = Media::AudioBook {
        title: String::from("1984"),
    };

    manga.description();
    audio_book.description();
    book.description();
    movie.description();
}

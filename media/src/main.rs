mod content;

use content::catalog::Catalog;
use content::media::Media;

fn main() {
    let manga = Media::Manga {
        title: String::from("One Piece"),
        author: String::from("Eichiro Oda"),
    };

    let movie = Media::Movie {
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
    let podcast = Media::Podcast(56);
    let placeholder = Media::Placeholder;

    let mut catalog = Catalog::new();

    catalog.add(manga);
    catalog.add(movie);
    catalog.add(audio_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    println!("{:#?}", catalog.get_by_index(0));
}

#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Manga { title: String, author: String },
    Movie { title: String, director: String },
    AudioBook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn description(&self) -> String {
        match self {
            Media::Book { title, author } => {
                format!("Book: {} {}", title, author)
            }
            Media::Manga { title, author } => {
                format!("Manga: {} {}", title, author)
            }
            Media::AudioBook { title } => {
                format!("AudioBook: {}", title)
            }
            Media::Movie { title, director } => {
                format!("Movie: {} {}", title, director)
            }
            Media::Podcast(episode_number) => {
                format!("Podcast: {}", episode_number)
            }
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }
}

enum MightHaveValue<'a> {
    ThereIsAValue(&'a Media),
    NoValueAvailable,
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }
    fn add(&mut self, media: Media) {
        self.items.push(media);
    }
    fn get_by_index_custom(&self, index: usize) -> MightHaveValue {
        if self.items.len() > index {
            return MightHaveValue::ThereIsAValue(&self.items[index]);
        }

        MightHaveValue::NoValueAvailable
    }

    fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            return Some(&self.items[index]);
        }

        None
    }
}

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

    prPintln!("{:#?}", catalog.get_by_index(0));
}

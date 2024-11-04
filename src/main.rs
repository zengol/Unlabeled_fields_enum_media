#[derive(Debug)]

enum Media {
    Book{title: String , autor : String},
    Movie{title:String , director: String},
    AudioBook{title: String},
    Serie{title: String},
    Podcast(u32),// Esta es una variante que contiene un solo valor de tipo u32 se lo conoce como variante de tupla de un solo elemento. A diferencia de las otras
    //como Book, Movie.. que contiene campos con nombres Podcast solo contiene un valor sin nombre.
    Placeholder, // Esta es una variante unitaria, lo que significa que no contiene ningún tipo de dato asociado. A diferencia de todas las otras variantes en el enum
    // que tienen algún tipo de dato asociado. Puede ser útil como un valor por defecto o para representar un estado especial que no requiere datos adiciaonales.

}

// impl utilizando Pattern Matching with Enum. 

impl Media {
    fn description (&self) -> String {
          match self {
            Media::Book {title, autor} => {
                format!("Book: {} {}",title, autor)
            },
            Media::Movie {title,director} => {
                format!("Movie: {} {}", title, director)
            },
            Media::AudioBook {title} => {
                format!("AudioBook {} ", title)
            }
            Media::Serie {title} => {
                format!("Audiobook: {} ", title)
            }
            // el id se extrae de la variante y se utiliza en la cadena formateada esto permite almacenar y recuperar un identificador numérico para cada podcast.
            Media::Podcast (id) => {
                format!("Podcast: {}", id)
            }
            Media::Placeholder => {
                format!("Placeholder") 
            }
        }
    } 

   
}
#[derive(Debug)]
struct Catalog {
    items: Vec<Media>
}

impl Catalog {
    //función que me sirve para (1) en main.
    fn new() ->Self {
        Catalog {items: vec![]}
    }
    //función que me sirve para (2) en main.
    fn add(&mut self, media: Media){
        self.items.push(media);
    }
}
 fn main() {
    //creamos un enum y asignamos un valor en AudioBook.
    let audiobook = Media::AudioBook { 
        title: String::from("An audiobook")
     };
     let book = Media::Book {
        title: String::from("Good Book"),
        autor: String::from("Good Autor"),
     };
     let movie = Media::Movie {
        title: String::from("Bad movie"),
        director: String::from("Bad director")
     };
     let serie= Media::Serie { title: String::from("Good serie") };

     let podcast = Media::Podcast (20);

     let placeholder= Media::Placeholder;


     /* 

 */
     // (1) inicializa  la impl
     let mut catalog = Catalog::new();

     // (2) lo que le envie por argumento la fn add() lo va a agregar al vector self.items.
     catalog.add(audiobook);
     catalog.add(book);
     catalog.add(movie);
     catalog.add(serie);
     catalog.add(podcast);
     catalog.add(placeholder);

     println!("{:#?}", catalog);
 }

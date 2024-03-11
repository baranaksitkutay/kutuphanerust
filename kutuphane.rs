// Kitapları temsil eden bir struct tanımlayın
struct Book {
    title: String,
    author: String,
    page_count: i32,
}

// Dergileri temsil eden bir struct tanımlayın
struct Magazine {
    title: String,
    issue: i32,
    topic: String,
}

// Yayınları (Publication) temsil eden bir enum tanımlayın
enum Publication {
    Book(Book),
    Magazine(Magazine),
}

// Bir vektör içinde çeşitli yayın örnekleri oluşturun
fn main() {
    let publications = vec![
        Publication::Book(Book {
            title: "Rust Programlama Dili".to_string(),
            author: "Kelsey Hightower".to_string(),
            page_count: 300,
        }),
        Publication::Magazine(Magazine {
            title: "Rust Dünyası".to_string(),
            issue: 1,
            topic: "Rust'un Geleceği".to_string(),
        }),
    ];

    // Vektördeki her bir yayını yazdırın
    for publication in publications {
        print_publication(publication);
    }
}

// Yayınları yazdıran fonksiyon
fn print_publication(publ: Publication) {
    match publ {
        Publication::Book(book) => {
            println!("Kitap: {} yazar: {}, {} sayfa", book.title, book.author, book.page_count);
        }
        Publication::Magazine(magazine) => {
            println!("Dergi: {} - Sayı: {}, Konu: {}", magazine.title, magazine.issue, magazine.topic);
        }
    }
}

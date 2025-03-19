// Music Library
pub struct MusicLibrary {}
impl MusicLibrary {
    pub fn get_songs() -> Vec<MusicLibrarySong> {
        vec![
            MusicLibrarySong {
                id: "coez-la-musica-non-c-e".into(),
                author: "Coez".into(),
                title: "La musica non c'è".into(),
            },
            MusicLibrarySong {
                id: "vamp-o2".into(),
                author: "U.L.".into(),
                title: "Vamp Original 2".into(),
            },
        ]
    }
}

// Music Library Songs
pub struct MusicLibrarySong {
    id: String,
    author: String,
    title: String,
}

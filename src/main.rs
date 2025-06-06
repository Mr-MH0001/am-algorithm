use crate::r#match::{find_best_match, AnimeTitle, ExpectAnime};

mod r#match;

fn main() {
    let mock_anime_list = vec![
        ExpectAnime {
            id: Some(serde_json::json!(1)),
            title: Some(AnimeTitle {
                english: Some("Attack on Titan".into()),
                romaji: Some("Shingeki no Kyojin".into()),
                native: Some("進撃の巨人".into()),
                user_preferred: Some("Shingeki no Kyojin".into()),
            }),
            year: Some(2013),
            episodes: Some(25),
        },
        ExpectAnime {
            id: Some(serde_json::json!(2)),
            title: Some(AnimeTitle {
                english: Some("Demon Slayer: Kimetsu no Yaiba".into()),
                romaji: Some("Kimetsu no Yaiba".into()),
                native: Some("鬼滅の刃".into()),
                user_preferred: Some("Kimetsu no Yaiba".into()),
            }),
            year: Some(2019),
            episodes: Some(26),
        },
        ExpectAnime {
            id: Some(serde_json::json!(3)),
            title: Some(AnimeTitle {
                english: Some("My Hero Academia Season 2".into()),
                romaji: Some("Boku no Hero Academia 2nd Season".into()),
                native: Some("僕のヒーローアカデミア 第2期".into()),
                user_preferred: Some("Boku no Hero Academia 2nd Season".into()),
            }),
            year: Some(2017),
            episodes: Some(25),
        },
        ExpectAnime {
            id: Some(serde_json::json!(4)),
            title: Some(AnimeTitle {
                english: Some("Steins;Gate".into()),
                romaji: Some("Steins;Gate".into()),
                native: Some("シュタインズ・ゲート".into()),
                user_preferred: Some("Steins Gate".into()),
            }),
            year: Some(2011),
            episodes: Some(24),
        },
        ExpectAnime {
            id: Some(serde_json::json!(5)),
            title: Some(AnimeTitle {
                english: Some("Fullmetal Alchemist: Brotherhood".into()),
                romaji: Some("Hagane no Renkinjutsushi: Fullmetal Alchemist".into()),
                native: Some("鋼の錬金術師 FULLMETAL ALCHEMIST".into()),
                user_preferred: Some("FMA Brotherhood".into()),
            }),
            year: Some(2009),
            episodes: Some(64),
        },
        ExpectAnime {
            id: Some(serde_json::json!(6)),
            title: Some(AnimeTitle {
                english: Some("Naruto Shippuden".into()),
                romaji: Some("Naruto: Shippuuden".into()),
                native: Some("ナルト 疾風伝".into()),
                user_preferred: Some("Naruto Shippuuden".into()),
            }),
            year: Some(2007),
            episodes: Some(500),
        },
        ExpectAnime {
            id: Some(serde_json::json!(7)),
            title: Some(AnimeTitle {
                english: Some("One Piece".into()),
                romaji: Some("One Piece".into()),
                native: Some("ワンピース".into()),
                user_preferred: Some("One Piece".into()),
            }),
            year: Some(1999),
            episodes: Some(1000),
        },
        ExpectAnime {
            id: Some(serde_json::json!(8)),
            title: Some(AnimeTitle {
                english: Some("Bleach: Thousand-Year Blood War".into()),
                romaji: Some("Bleach: Sennen Kessen-hen".into()),
                native: Some("BLEACH 千年血戦篇".into()),
                user_preferred: Some("Bleach TYBW".into()),
            }),
            year: Some(2022),
            episodes: Some(13),
        },
    ];

    let search = ExpectAnime::from_string_title("Attack on Titan".to_string(), Some(2013), Some(25));

let result = find_best_match(&search, &mock_anime_list, |anime| anime);

match result {
    Some(m) => println!("🎯 Found match: {:?} via {:?}", m.title.or(m.normalized), m.method),
    None => println!("No match found..."),
}

}

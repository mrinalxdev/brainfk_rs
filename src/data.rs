use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;

pub struct Movie {
    pub id: u32,
    pub title: String,
    pub genres: Vec<String>,
}

pub struct Rating {
    pub user_id: u32,
    pub movie_id: u32,
    pub rating: f32,
}

pub struct Dataset {
    pub movies: Vec<Movie>,
    pub ratings: Vec<Rating>,
}

impl Dataset {
    pub fn load_from_csv(movie_file: &str, rating_file: &str) -> Result<Dataset, Box<dyn Error>> {
        let mut movies = vec![];
        let mut ratings = vec![];

        let movie_reader_results = ReaderBuilder::new().has_headers(true).from_path(movie_file);
        let movie_reader = movie_reader_results?;
        for result in movie_reader.into_records() {
            let record = result?;
            let id: u32 = record[0].parse()?;
            let title = record[1].to_string();
            let genres: Vec<String> = record[2].split('|').map(|s| s.to_string()).collect();
            movies.push(Movie {id, title, genres});
        }

        let rating_reader_results = ReaderBuilder::new().has_headers(true).from_path(rating_file);
        let rating_reader = rating_reader_results?;
        for result in rating_reader.into_records() {
            let record = result?;
            let user_id: u32 = record[0].parse()?;
            let movie_id: u32 = record[1].parse()?;
            let rating: f32 = record[2].parse()?;
            ratings.push(Rating { user_id, movie_id, rating});
        }
        Ok(Dataset { movies, ratings })
    }
}

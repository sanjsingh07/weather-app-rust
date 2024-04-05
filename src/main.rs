use clap::Parser;
use serde::Deserialize;

/// [`DEAFAULT_CITY`] sets the city of which we want to return weather of.
pub const DEAFAULT_CITY: &str = "Kyiv";
pub const DEAFAULT_API_KEY: &str = "000ca14e3b8c4019dcacaa6eb09a51cf";

// Struct to map input from command arguements
#[derive(Parser, Debug)]
pub struct CmdArgs {
    #[arg(
        default_value_t = DEAFAULT_CITY.to_string(),
        help = "Name of the city, of which you want to get weather information."
    )]
    pub city_name: String,

    #[arg(
        default_value_t = DEAFAULT_API_KEY.to_string(),
        help = "Open Weather API key."
    )]
    pub api_key: String,
}

// Struct to deserialize the JSON response from OpenWeatherMap API
#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

// Struct to map weather description
#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

// Struct to map main weather data
#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: f64,
    feels_like: f64,
}

// Struct to map wind information
#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

#[derive(thiserror::Error, Debug)]
pub enum WeatherError {
    #[error("Encountered error while serialization: {0}")]
    SerializationErr(#[from] serde_json::Error),

    #[error("Encountered error while calling open weather api : {0}")]
    ReqwestEr(#[from] reqwest::Error),
}

impl CmdArgs {
    async fn get_weather_fn(&self) -> Result<(), WeatherError> {
        // this shouldn't be hardcoded and should be used as an const but since this is a very simple weather fetching
        // app so it should be fine to hardcode.
        let request_url = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={city_name}&appid={api_key}",
            city_name = self.city_name,
            api_key = self.api_key
        );

        let response = reqwest::get(&request_url).await?;

        // Check if the request was successful (status code 200)
        if response.status().is_success() {
            let weather_data = response.text().await?;
            let wather_json = serde_json::from_str::<WeatherResponse>(&weather_data)?;

            let res_print =  format!("| City: {} | Temprature: {} | feels like: {} | Wind Speed: {} | Humidity: {} | Weather Description: {}",
            wather_json.name,
            wather_json.main.temp,
            wather_json.main.feels_like,
            wather_json.wind.speed,
            wather_json.main.humidity,
            wather_json.weather[0].description
            );

            println!("---------------------------------------------------------------------------------------------------------------------------------");
            println!("{}", res_print);
            println!("---------------------------------------------------------------------------------------------------------------------------------");
        } else {
            // Print an error message if the request was not successful
            println!("Error: {}", response.status());
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let input_args = CmdArgs::parse();

    let _ = input_args.get_weather_fn().await;
}

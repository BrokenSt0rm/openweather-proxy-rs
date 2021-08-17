# OpenWeather Proxy

OpenWeather Proxy is a simple project written in Rust to proxy the requests that goes from your app to OpenWeather.
Note that this has a very specific use for [Pock](https://github.com/pock/pock) and its [Weather Widget](https://github.com/pock/weather-widget).
There is also a full route currently not binded in Actix, that reflects the OpenWeather response.

## Installation

Just clone this repository, and create a .env file based on the .env.example included.

## Usage

```
docker-compose up -d
```
Right now this project has just one endpoint that can be used as below
```
http://HOST:PORT/condition?lat={latitude}&lon={longitude}&unit=[celsius, fahrenheit, kelvin]&name=...
```

## Todo

There are still some features to be implemented, such as:
 - Request caching

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License
[MIT](https://choosealicense.com/licenses/mit/)

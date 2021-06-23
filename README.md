# OpenWeather Proxy

OpenWeather Proxy is a simple project written in Rust to proxy the requests that goes from your app to OpenWeather.

## Installation

Just clone this repository, and create a .env file based on the .env.example included.

## Usage

```
docker-compose up -d
```
The Docker container is binded to the port 8080.  
Right now this project has just one endpoint that can be used as below
```
http://HOST:PORT/weather?lat={latitude}&lon={longitude}&unit=[metrics, fahrenheit, kelvin]
```

## Todo

There are still some features to be implemented, such as:
 - Rate Limiter
 - Request caching

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License
[MIT](https://choosealicense.com/licenses/mit/)
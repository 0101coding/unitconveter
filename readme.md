M-Convert!
=========

A basic command line program that is able to convert between common day to day units. For example converting 10mph to m/s or kph. See Details below

Examples
1. ```sh
    cargo run 10F to C
    ```
2. ```sh
    cargo run
    Convert: 10F to C
    ```
3. ```sh
    cargo run
    Convert: 10F
    To: C
    ```

### Supported Units
- Length: *Meter*(m), *Kilometer*(km), *Mile*(ml), *Foot* (ft)<br />
e.g 10m to Km

- Mass: *Kilogram*(kg), *Gram*(g), *Pound*(lb), *Ounce*(oz) <br />
e.g 10oz to lb 

- Temperature: *Celcius*(°C), *Fahrenheit*(°F), *Kelvin*(°K)<br />
e.g 100°f to °C 

- Time: *Seconds*(sec), *Minutes*(min), *Hour* (hr), *Day* (d), *Month*(m), *Year*(yr), *Decade*(dec), *Century*(cent), *Millenium* (M)<br />
e.g 1hr to s


### Known Issue
The Time conversion to seconds is still buggy
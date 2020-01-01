# Collecting data (at least once per version)

## Manual frame delay benchmarks
This benchmark must be triggered manually on different devices.
The output should then be copy-pasted  without modification to [frame_delays.manual.data](./frame_delays.manual.data).
### Local tests
 1) `make`
 2) Open browser at localhost
 3) press number keys to run tests

### Running on mobile:
 1) `make mobile`
 2) Open hotspot on computer, connect with phone
 3) Open website by IP on the phone (Some hacks around keycloak urls might be necessary, e.g. changing on-submit url on HTML form)
 4) Connect to computer using remote debugging console and type 
```
document.dispatchEvent(new KeyboardEvent('keydown', {'key': '1', 'code': 'Digit1'}))
```

## Application size statistics
```
./paddlers-frontend/benchmarks/app_size_stats.sh
```
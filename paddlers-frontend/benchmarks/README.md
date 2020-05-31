# Collecting data (at least once per version)

## Manual frame delay benchmarks
This benchmark must be triggered manually on different devices.
The output should then be copy-pasted  without modification to [frame_delays.manual.data](./frame_delays.manual.data).
Repeat until stable, then pick one representative sample for each category.
### Local tests
 0) Use Lenovo Y50-70 without second monitor, close all other applications (beside VS code), connect to power
 1) `make`
 2) Open browser at localhost
 3) Different resolution and window size:
    * High: fullscreen
    * Mid: window.innerHeight=750
    * Low: window.innerHeight=450
 4) press number keys to run tests

### Running on mobile:
 1) `make mobile`
 2) Open hotspot on computer, connect with phone
 3) Open website by IP on the phone in fullscreen (Some hacks around keycloak urls might be necessary, e.g. changing on-submit url on HTML form)
 4) Connect to computer using old firefox and WebIDE for remote debugging console, use ctrl+f5 in console, go to full-screen and type in console
```
document.dispatchEvent(new KeyboardEvent('keydown', {'key': '1', 'code': 'Digit1'}))
```

## Application size statistics
```
./paddlers-frontend/benchmarks/app_size_stats.sh
```
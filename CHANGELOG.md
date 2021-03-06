# Changelog

## 0.1.1 - YYYY-MM-DD

## 0.1.0 - 2018-04-02
* To respect tradition on command line, the default OUTPUT is `"-"` (i.e. the _dash_) instead of the `"STDOUT"`: this means that you CAN now name the output file `"STDOUT"` without confusion
* Started introducing testing (though I'm still learning how best to structure them for Rust)
* Separating code more and more into modules (first module, `CLI`)
* Setup Travis to handle build and release using cross compilation. Targets: Linux, Windows and MacOSX at 32 and 64 bit

## 0.0.2 - 2018-03-10
* Option to select the OUTPUT file where the HTTP Response body will be flushed
* Began splitting code into modules. First module: `cli`.

## 0.0.1 - 2018-02-23
* First release. Does practically nothing.

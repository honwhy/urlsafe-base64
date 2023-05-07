# URL Safe Base64

URL Safe Base64 util module for Node.js applications

## Installation

- clone this repository

- install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

- build

```
  wasm-pack build --target bundler
```

- install package to global node_modules

```
  cd pkg
  npm link
```

- start web app

```
cd webapp
npm link urlsafe-base64
npm install
npm run dev -- --open
```

## Usage

Require it within your module:

``` javascript
  import {encode, decode} from 'urlsafe-base64'
```

### .encode(string)

Encodes a string as a URL Safe Base64 string. This function encodes to 
the RFC 4648 Spec where '+' is encoded as '-' and '/' is encoded as '_'. 
The padding character '=' is removed.


### .decode(string)

Decodes a URL Safe Base64 string as a string.


## Inspires by

- [RGBboy/urlsafe-base64](http://travis-ci.org/RGBboy/urlsafe-base64)
- [Compiling from Rust to WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm)

MIT License

Copyright (c) 2023 Honwhy Wang

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
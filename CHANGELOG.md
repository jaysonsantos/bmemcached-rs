<a name="0.5.0"></a>
## 0.5.0 (2018-07-30)


#### Features

*   Make protocol public to allow client to choose how to use it ([eca24b94](https://github.com/jaysonsantos/bmemcached-rs/commit/eca24b9433e1a5706054109987636c0a3c9b1d08))
*   Enable custom types to be used by user (#9) ([db76fd63](https://github.com/jaysonsantos/bmemcached-rs/commit/db76fd631b75e59a207e95f891f0e3150fc13c9b))
*   Enable custom types to be used by user (#7) ([97d2816f](https://github.com/jaysonsantos/bmemcached-rs/commit/97d2816f044415c89e457265bbfa11a81dc55716))
* **error:**  Change error system to error-chain (#11) ([ea627ee3](https://github.com/jaysonsantos/bmemcached-rs/commit/ea627ee3456f73d39c7097b1f1647039bc74f27f))
* **protocol:**
  *  Add support to insert slices ([d9956b79](https://github.com/jaysonsantos/bmemcached-rs/commit/d9956b79ea87be8b781b1b73f08bebc03b77201c))
  *  Add support to InvalidArguments ([72c6f5c9](https://github.com/jaysonsantos/bmemcached-rs/commit/72c6f5c96911cba0e7feacd3b9e621253fd7f50b))
  *  Add error for big payloads ([3da968d7](https://github.com/jaysonsantos/bmemcached-rs/commit/3da968d73f1c42f30b116873fcec04e1f779f747))
  *  Make write_request write to a buffer before sending the whole payload ([0534d479](https://github.com/jaysonsantos/bmemcached-rs/commit/0534d479ebb5770c114c0e6da764860e9bb11a45))
  *  Make protocol's connection buffered ([3c7e9e70](https://github.com/jaysonsantos/bmemcached-rs/commit/3c7e9e70b51dbe935fff6d5594076d42230862f2))

#### Bug Fixes

* **protocol:**  Make sure the library wont break with big key names ([a327f208](https://github.com/jaysonsantos/bmemcached-rs/commit/a327f2081234560f32d26a95606ef300cae25cc6))



<a name="0.4.0"></a>
## 0.4.0 (2017-11-01)


#### Features

* **error:**  Change error system to error-chain (#11) ([ea627ee3](https://github.com/jaysonsantos/bmemcached-rs/commit/ea627ee3456f73d39c7097b1f1647039bc74f27f))



<a name="0.3.0"></a>
### 0.3.0 (2017-09-13)


#### Features

*   Enable custom types to be used by user (#9) ([db76fd63](https://github.com/jaysonsantos/bmemcached-rs/commit/db76fd631b75e59a207e95f891f0e3150fc13c9b))
*   Enable custom types to be used by user (#7) ([97d2816f](https://github.com/jaysonsantos/bmemcached-rs/commit/97d2816f044415c89e457265bbfa11a81dc55716))

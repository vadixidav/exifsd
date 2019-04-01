# exifsd

Exif serialization and deserialization

## Development

To run the tests for this library, the git submodule `exif-samples` must be checked out.

To do that run the following in this repository:

```bash
git submodule update --init
```

You can also clone this repository recursively:

```bash
git clone --recursive https://github.com/vadixidav/exifsd.git
```

## Credits

[This excellent overview of the Exif file format](https://www.media.mit.edu/pia/Research/deepview/exif.html) was incredibly useful in the development of this library. Big thanks to TsuruZoh Tachibanaya!

[This overview helped in identifying JPEG markers](http://vip.sugovica.hu/Sardi/kepnezo/JPEG%20File%20Layout%20and%20Format.htm).

[Wikipedia was useful in parsing entopy-encoded data](https://en.wikipedia.org/wiki/JPEG#Syntax_and_structure).

[This](http://gvsoft.no-ip.org/exif/exif-explanation.html) seems to be mostly the same as the one by TsuruZoh Tachibanaya, but it provides some slight variation and is more compact.

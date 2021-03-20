# zorro_cli
Command line interface for Zorro, a library for stashing data into images, and pulling it back out.

# build steps:

clone

        git clone git@github.com:chrisftw/zorro_cli.git

build

        cd zorro_cli
        cargo build --release

move into $PATH

        cp ./target/release/zorro_cli /usr/local/bin

# usage:
get help

        $ zorro_cli -h
        zorro_cli - serializing data into png format
        -m, --mode      (default 'static')
        -t, --depth     (default 8)
        -o, --output    (default 'out.png')
        -s, --source    (default '')
        -f, --filepath  (default '')
        -d, --decode
        -e, --encode
        <input>         (default 'stdin')

**mode**/**depth** current modes are static and hidden. More are coming.
*static* places lots of data in each pixel for smallest images. Can not use a source image. Uses depth 8.

*hidden* places data (depth 2 or 6) under existing data. 2 not very noticable. 6 is very noticable where the data resides.

**output** is path where you want your encoded image or decoded text to go.

**source** is the image path of original image for hidden data. Must be RGB png 8 bit depth currently.

**filepath** is the path of the data you want to encode (if blank uses stdin)

**encode**/**decode** choose one.

**input** file path to decode.


Example:

encoding:

         zorro_cli -o ./encoded_image.png --depth 2 -m hidden -f ./my_secret_diary_original.txt -s ./my_face.png -e

decoding: 

        zorro_cli ../encoded_image.png -d -o ./my_secret_diary.txt

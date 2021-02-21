extern crate lapp;
extern crate zorro;

fn main() {
    let args = lapp::parse_args("
        zorro_cli - serializing data into png format
        -v, --verbose   verbose output
        -m, --mode      (default 'static')
        -o, --output    (default 'out.png')
        -d, --decode
        -e, --encode
        -f, --filepath
        <input>         (default 'stdin')
    ");
    let do_decode = args.get_bool("decode");
    // let verbose = args.get_bool("verbose");
    let mode = args.get_string("mode");
    let input = args.get_string("input");
    let output = args.get_string("output");
    let filepath = args.get_string("filepath");
    if do_decode {
        if output {
            zorro::zorro::decode_to_file(&input, &output, &mode);
        } else {
            println!("{:?}", zorro::zorro::decode(&input, &mode));
        }
    } else {
        if filepath {
            zorro::zorro::encode_from_file(&filepath, &output, &mode);
        } else {
            zorro::zorro::encode(&input, &output, &mode);
        }
    }
}

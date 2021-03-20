extern crate lapp;
extern crate zorro;

fn main() {
    let args = lapp::parse_args("
        zorro_cli - serializing data into png format
        -v, --verbose   verbose output
        -m, --mode      (default 'static')
        -t, --depth     (default 8)
        -o, --output    (default 'out.png')
        -s, --source    (default '')
        -f, --filepath  (default '')
        -d, --decode
        -e, --encode
        <input>         (default 'stdin')
    ");
    
    // let verbose = args.get_bool("verbose");
    let mode = args.get_string("mode");
    let depth:u8 = args.get_integer("depth") as u8;
    let output = args.get_string("output");
    let source = args.get_string("source");
    let filepath = args.get_string("filepath");
    let do_decode = args.get_bool("decode");
    let input = args.get_string("input");
    if do_decode {
        if output.len() > 0 {
            zorro::decode_to_file(&input, &output);
        } else {
            println!("{:?}", zorro::decode(&input));
        }
    } else {
        if filepath.len() > 0 {
            zorro::encode_from_file(&filepath, &output, &mode, depth, &source);
        } else {
            zorro::encode(&input, &output, &mode, depth, &source);
        }
    }
}

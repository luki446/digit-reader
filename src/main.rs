#![feature(box_syntax)]
use nn::{NN, HaltCondition};
use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::mem;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2
    {
        match args[1].as_ref()
        {
            "create" =>
            {
                let net = NN::new(&[28 * 28, 8 * 8, 10]);
                let mut out = File::create(&args[2]).expect("Cannot open file");
                out.write(net.to_json().as_bytes()).expect("Cannot save to file");
            },
            "train" =>
            {
                let mut network: NN;
                {
                    let mut net_src = String::new();
                    let mut input_file_net = File::open(&args[2]).expect("Cannot open file");
                    input_file_net.read_to_string(&mut net_src).expect("Cannot read file to string");
                    network = NN::from_json(&net_src);
                }
                let mut in_images = File::open("images").expect("Cannot open file with images");
                let mut in_labels = File::open("labels").expect("Cannot open file with labels");

                let mut images_raw = box [[0; 28 * 28]; 60000];
                let mut labels_raw = box [[0;1]; 60000];

                unsafe 
                {
                    let mut foo: [u8; 4] = [0;4];
                    in_images.read_exact(&mut foo).unwrap();
                    foo.reverse();
                    let header: i32 = mem::transmute(foo);

                    in_images.read_exact(&mut foo).unwrap();
                    foo.reverse();
                    let amount: i32 = mem::transmute(foo);

                    in_images.read_exact(&mut foo).unwrap();
                    foo.reverse();
                    let width_n: i32 = mem::transmute(foo);

                    in_images.read_exact(&mut foo).unwrap();
                    foo.reverse();
                    let height_n: i32 = mem::transmute(foo);
                    

                    println!("File \"images\" with {} images {}x{}, header: {}", amount, width_n, height_n, header);
                }

                unsafe 
                {
                    let mut foo: [u8; 4] = [0;4];
                    in_labels.read_exact(&mut foo).unwrap();
                    foo.reverse();
                    let header: i32 = mem::transmute(foo);

                    in_labels.read_exact(&mut foo).unwrap();
                    foo.reverse();
                    let amount: i32 = mem::transmute(foo);

                    println!("File \"labels\" with {} labels, header: {}", amount, header);
                }
                for i in 0..60000
                {
                    in_images.read_exact(&mut images_raw[i]).unwrap();
                    in_labels.read_exact(&mut labels_raw[i]).unwrap();
                }

                let mut data: Vec::<(Vec::<f64>, Vec::<f64>)> = Vec::with_capacity(60000);

                for i in 0..60000
                {
                    let foo1: Vec<f64> = images_raw[i].into_iter()
                    .map(|foo|
                        *foo as f64 / 255.).collect();
                    let foo2 = (0..10).map(|n| -> f64
                        {
                            if labels_raw[i][0] == n 
                            {
                                return 1.;
                            }
                            else
                            {
                                return 0.;
                            }
                        }
                        ).collect::<Vec<f64>>();
                    data.insert(i, (foo1, foo2));
                }
                println!("No elo!");
                network.train(data.as_slice())
                    .halt_condition(HaltCondition::Timer(std::time::duration::Duration::seconds(20)))
                    .log_interval(Some(5))
                    .rate( 0.3 )
                    .go();
                
                let mut out = File::create(&args[2]).expect("Cannot open file");
                out.write(network.to_json().as_bytes()).expect("Cannot write to file");
 
            },  
            "recognize" =>
            {

            },
            _ =>
            {
                panic!("{} is wrong second argument", args[1]);
            },
        }
    }
    else
    {
        panic!("Too few arguments!\n Example usage ./digit-reader create neural");
    }
}

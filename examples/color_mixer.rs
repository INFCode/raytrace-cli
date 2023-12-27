use raytrace_cli::color::{ColorMixer, LinearMixer, LinearRgbColor, RMSMixer};

fn main() {
    let block_width = 64;
    let block_height = 64;
    let line_width = 8;

    let num_colors = 8;

    let height = block_height * num_colors + line_width * (num_colors - 1);
    let width = 3 * block_width;

    print!("P3\n{} {}\n255\n", width, height);

    let colors = [
        [
            LinearRgbColor::from_hex(0xff0000),
            LinearRgbColor::from_hex(0x00ff00),
        ],
        [
            LinearRgbColor::from_hex(0xffffff),
            LinearRgbColor::from_hex(0x000000),
        ],
        [
            LinearRgbColor::from_hex(0x8000a0),
            LinearRgbColor::from_hex(0x2f4f00),
        ],
        [
            LinearRgbColor::from_hex(0xaabeef),
            LinearRgbColor::from_hex(0xc00100),
        ],
        [
            LinearRgbColor::from_hex(0xcf0000),
            LinearRgbColor::from_hex(0x10f010),
        ],
        [
            LinearRgbColor::from_hex(0x3aaa11),
            LinearRgbColor::from_hex(0xff0000),
        ],
        [
            LinearRgbColor::from_hex(0x1f3f5f),
            LinearRgbColor::from_hex(0x10ff70),
        ],
        [
            LinearRgbColor::from_hex(0xff1030),
            LinearRgbColor::from_hex(0x4090aa),
        ],
    ];
    let mut mixed = vec![];

    for [c1, c2] in &colors {
        let mut linear = LinearMixer::new();
        let mut rms = RMSMixer::new();
        linear.add(&c1).add(&c2);
        rms.add(&c1).add(&c2);
        mixed.push([linear.mix(), rms.mix()]);
    }

    for color_i in 0..num_colors {
        for h in 0..block_height {
            for part_j in 0..3 {
                for _ in 0..block_width {
                    match part_j {
                        0 => println!("{}", colors[color_i][0]),
                        1 => {
                            if h < block_height / 2 {
                                println!("{}", mixed[color_i][0])
                            } else {
                                println!("{}", mixed[color_i][1])
                            }
                        }
                        2 => println!("{}", colors[color_i][1]),
                        _ => panic!(),
                    }
                }
            }
        }
        for _ in 0..line_width {
            for _ in 0..width {
                println!("{}", LinearRgbColor::from_hex(0x000000));
            }
        }
    }
}

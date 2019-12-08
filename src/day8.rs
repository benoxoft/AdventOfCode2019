#[aoc(day8, part1)]
fn find_solution1(input: &str) -> usize {
    let layers = split_image(input, 25, 6);
    let sol = layers_check(&layers);
    sol
}

#[aoc(day8, part2)]
fn find_solution2(input: &str) -> String {
    let template_str = "222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222";
    let layers = split_image(input, 25, 6);
    let ret = layers_stack(&layers, template_str);
    ret
}

fn split_image(dsn: &str, wide: usize, tall: usize) -> Vec<&str> {
    let layer_size = wide * tall;
    let layer_count = dsn.len() / layer_size;
    let mut layers = Vec::new();

    for i in 0..layer_count {
        let layer = &dsn[i*layer_size..(i+1)*layer_size];
        layers.push(layer);
    }
    layers
}

fn layers_stack(layers: &Vec<&str>, template_str: &str) -> String {
    let mut template: Vec<char> = template_str.chars().collect();

    for layer_idx in 0..layers.len() {
        println!("LAYER {}", layer_idx);
        let layer = layers.get(layer_idx).unwrap();
        for idx in 0..layer.len() {
            let c_layer = layer.chars().nth(idx).unwrap();
            let c_template = template[idx];
            if c_layer == '2' {
                continue;
            } else if c_template == '2' {
                template[idx] = c_layer;
            }
        }
    }
    let ret: String = template.into_iter().collect();
    ret
}

fn layers_check(layers: &Vec<&str>) -> usize {
    let mut fewest_zeros = std::usize::MAX;
    let mut fewest_layer: &str = "";

    for layer in layers {
        let mut zero_count = 0;

        for c in layer.chars() {
            if c == '0' {
                zero_count += 1;
            }
        }
        if zero_count < fewest_zeros {
            fewest_zeros = zero_count;
            fewest_layer = layer;
        }
    }

    let mut count1 = 0;
    let mut count2 = 0;
    for c in fewest_layer.chars() {
        if c == '1' {
            count1 += 1;
        } else if c == '2' {
            count2 += 1;
        }
    }
    count1 * count2
}

#[test]
fn test_image() {
    let data = "123456789012";
    let layers = split_image(data, 3, 2);
    assert_eq!(layers.get(0).unwrap(), &"123456");
    assert_eq!(layers.get(1).unwrap(), &"789012");   
}

#[test]
fn test_decode() {
    let input = "0222112222120000";
    let template_str = "2222";
    let layers = split_image(input, 2, 2);
    let ret = layers_stack(&layers, template_str);
    assert_eq!(ret, "0110");
}

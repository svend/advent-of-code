use std::fmt;

type Layer = Vec<u32>;

fn flatten_layers(top: &[u32], bottom: &[u32]) -> Layer {
    assert_eq!(top.len(), bottom.len());
    let mut layer = Layer::new();
    for (p1, p2) in top.iter().zip(bottom.iter()) {
        let p = if *p1 == 2 { p2 } else { p1 };
        layer.push(*p);
    }
    layer
}

struct Image {
    width: usize,
    layers: Vec<Layer>,
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, layer) in self.layers.iter().enumerate() {
            writeln!(f, "layer {}:", i)?;
            for row in layer.chunks(self.width) {
                let row: String = row
                    .iter()
                    .map(|n| n.to_string())
                    .map(|s| if s == "0" { " ".to_string() } else { s })
                    .collect();
                writeln!(f, "{}", row)?;
            }
        }
        Ok(())
    }
}

impl Image {
    fn new(width: usize, height: usize, data: &[u32]) -> Self {
        let layers: Vec<Layer> = data.chunks(width * height).map(|c| c.to_vec()).collect();
        Image { width, layers }
    }

    fn layer_with_fewest(&self, n: u32) -> Option<&Layer> {
        self.layers
            .iter()
            .map(|layer| (layer, layer.iter().filter(|i| i == &&n).count()))
            // error: cannot infer an appropriate lifetime for pattern due to conflicting requirements
            // .max_by_key(|(_layer, count)| count)
            .min_by_key(|layer_count| layer_count.1)
            .map(|(layer, _count)| layer)
    }

    fn checksum(&self) -> Option<usize> {
        if let Some(layer) = self.layer_with_fewest(0) {
            println!(
                "fewest: {}",
                layer.iter().map(|n| n.to_string()).collect::<String>()
            );
            let ones = layer.iter().filter(|n| **n == 1).count();
            let twos = layer.iter().filter(|n| **n == 2).count();
            Some(ones * twos)
        } else {
            None
        }
    }

    fn flatten(&self) -> Self {
        let width = self.width;
        let len = self.layers.first().map_or(0, |layer| layer.len());
        let layer = self
            .layers
            .iter()
            .fold(vec![2; len], |acc, layer| flatten_layers(&acc, layer));
        let layers = vec![layer];
        Image { width, layers }
    }
}

fn parse_input(s: &str) -> Vec<u32> {
    s.trim_end()
        .chars()
        .map(|c| c.to_digit(10).expect("invalid digit"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_parse_input() {
        let input = "123456789012";
        let expected = &[1, 2, 3, 4, 5, 6, 8, 9, 0, 1, 2];
        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_image() {
        let input = parse_input("123456789012");
        Image::new(3, 2, &input);
    }

    #[test]
    fn test_checksum() {
        let input = parse_input("123456789012");
        let image = Image::new(3, 2, &input);
        assert_eq!(image.checksum().unwrap(), 1);

        let input = parse_input(include_str!("day8.input"));
        let image = Image::new(25, 6, &input);
        assert_eq!(image.checksum().unwrap(), 1548)
    }

    #[test]
    fn test_flatten() {
        let input = parse_input("0222112222120000");
        let image = Image::new(2, 2, &input);
        let image = image.flatten();
        println!("{}", image);

        let input = parse_input(include_str!("day8.input"));
        let image = Image::new(25, 6, &input);
        let image = image.flatten();
        println!("{}", image);
    }
}

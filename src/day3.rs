#[derive(Debug)]
struct Claim {
    id: u32,
    origin_x: usize,
    origin_y: usize,
    width: usize,
    height: usize,
}

fn parse_claim(s: &str) -> Claim {
    let fields: Vec<_> = s.split_whitespace().collect();
    let id = fields[0].trim_start_matches('#').parse::<u32>().unwrap();
    let loc: Vec<_> = fields[2]
        .trim_end_matches(':')
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let origin_x = loc[0];
    let origin_y = loc[1];
    let dims: Vec<_> = fields[3]
        .split('x')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let width = dims[0];
    let height = dims[1];

    Claim {
        id,
        origin_x,
        origin_y,
        width,
        height,
    }
}

type Sheet = Vec<u32>;

fn sheet_new(size: usize) -> Sheet {
    vec![0; size * size]
}

fn apply_claims(size: usize, claims: &[Claim]) -> Sheet {
    let mut sheet = sheet_new(size);

    for c in claims {
        for x in 0..c.width {
            for y in 0..c.height {
                sheet[(c.origin_x + x) * size + (c.origin_y + y)] += 1;
            }
        }
    }

    sheet
}

fn overlaps(size: usize, claims: &[Claim]) -> u32 {
    let sheet = apply_claims(size, claims);

    let mut overlaps = 0;
    for cell in sheet.iter() {
        if cell > &1 {
            overlaps += 1;
        }
    }

    overlaps
}

fn no_overlaps(size: usize, claims: &[Claim]) -> Vec<u32> {
    let sheet = apply_claims(size, &claims);

    let mut ok_claims = vec![];
    for c in claims {
        let mut overlaps = false;
        for x in 0..c.width {
            for y in 0..c.height {
                if sheet[(c.origin_x + x) * size + (c.origin_y + y)] > 1 {
                    overlaps = true;
                }
            }
        }
        if !overlaps {
            ok_claims.push(c.id);
        }
    }

    ok_claims
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlaps() {
        let input = include_str!("3.input");
        let claims: Vec<_> = input.lines().map(|s| parse_claim(s)).collect();

        assert_eq!(overlaps(5000, &claims), 115304);
    }

    #[test]
    fn test_no_overlaps() {
        let input = include_str!("3.input");
        let claims: Vec<_> = input.lines().map(|s| parse_claim(s)).collect();

        let ok_claims = no_overlaps(5000, &claims);
        assert_eq!(ok_claims.len(), 1);
        assert_eq!(ok_claims.first(), Some(&275));
    }
}

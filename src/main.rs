use std::str::FromStr;
use palette::encoding::Srgb;
use rand::Rng;
use reqwest::Response;
use svg::Document;
use svg::node::element::Rectangle;


#[derive(Debug)]
struct Palette {
    colors: Vec<palette::rgb::Rgb<Srgb, u8>>,
}

impl Palette {
    fn get_random_color(&self) -> String {
        let mut rnd = rand::thread_rng();
        let color = &self.colors[rnd.gen_range(0..self.colors.len())];

        format!("#{:X}{:X}{:X}", color.red, color.green, color.blue)
    }
}

fn rect_with_pos_and_size(x: i32, y: i32, width: i32, height: i32) -> Rectangle {
    Rectangle::new()
        .set("x", x)
        .set("y", y)
        .set("width", width)
        .set("height", height)
}

fn grid_cells() -> (i32, i32) {
    let mut rnd = rand::thread_rng();

    (rnd.gen_range(3..=8), rnd.gen_range(3..=8))
}

async fn get_palettes() -> reqwest::Result<Response> {
    reqwest::get("https://unpkg.com/nice-color-palettes@3.0.0/100.json").await
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let palettes: Vec<Palette> = get_palettes()
        .await?.json::<Vec<Vec<String>>>()
        .await?.into_iter().map(|colors|
        {
            Palette {
                colors: colors.into_iter().map(|color| {
                    palette::Srgb::from_str(color.as_str()).unwrap()
                }).collect()
            }
        }
    ).collect();


    let mut rnd = rand::thread_rng();

    let palette = &palettes[rnd.gen_range(0..palettes.len())];

    let (x, y) = grid_cells();


    let mut document = Document::new();

    for y in 0..y {
        for x in 0..x {
            let col = palette.get_random_color();
            let rect = rect_with_pos_and_size(x * 10, y * 10, 10, 10)
                .set("fill", col);
            // .set("stroke", "black");
            document = document.add(rect);
        }
    }


    document = document.set("viewBox", (0, 0, x * 10, y * 10));

    svg::save("image.svg", &document).unwrap();

    Ok(())
}

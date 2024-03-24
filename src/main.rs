use std::fmt;

// #[derive(Debug)]
struct PokemonProduct {
    url: Option<String>,
    image: Option<String>,
    name: Option<String>,
    price: Option<String>,
}

impl fmt::Display for PokemonProduct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "URL: {:?}, Image: {:?}, Name: {:?}, Price: {:?}",
            self.url, self.image, self.name, self.price
        )
    }
}

fn scrape_to_csv() {
    let response = reqwest::blocking::get("https://scrapeme.live/shop/");

    let html_content = response.unwrap().text().unwrap();
    // println!("{html_content}");

    let document = scraper::Html::parse_document(&html_content);
    // println!("{document:?}");

    let html_product_selector = scraper::Selector::parse("li.product").unwrap();
    // println!("{html_product_selector:?}");

    let html_products = document.select(&html_product_selector);
    // println!("{html_products:?}");

    let mut pokemon_products: Vec<PokemonProduct> = Vec::new();

    for html_product in html_products {
        let url = html_product
            .select(&scraper::Selector::parse("a").unwrap())
            .next()
            .and_then(|a| a.value().attr("href"))
            .map(str::to_owned);

        println!("{url:?}");

        let image = html_product
            .select(&scraper::Selector::parse("img").unwrap())
            .next()
            .and_then(|img| img.value().attr("src"))
            .map(str::to_owned);

        println!("{image:?}");

        let name = html_product
            .select(&scraper::Selector::parse("h2").unwrap())
            .next()
            .map(|h2| h2.text().collect::<String>());

        println!("{name:?}");

        let price = html_product
            .select(&scraper::Selector::parse(".price").unwrap())
            .next()
            .map(|price| price.text().collect::<String>());

        println!("{price:?}");

        let pokemon_product = PokemonProduct {
            url,
            image,
            name,
            price,
        };
        pokemon_products.push(pokemon_product);
    }
    for product in &pokemon_products {
        println!("{}", product);
    }

    let path = std::path::Path::new("products.csv");
    let mut writer = csv::Writer::from_path(path).unwrap();

    writer
        .write_record(["url", "image", "name", "price"])
        .unwrap();

    for product in pokemon_products {
        let url = product.url.unwrap();
        let image = product.image.unwrap();
        let name = product.name.unwrap();
        let price = product.price.unwrap();
        writer.write_record(&[url, image, name, price]).unwrap();
    }
    writer.flush().unwrap();
}

fn crawling_pages() {
    let first_page = "https://scrapeme.live/shop/page/1/";

    let mut pages_to_scrape: Vec<String> = vec![first_page.to_owned()];
    let mut pages_discovered = std::collections::HashSet::new();

    let mut i = 1;
    let max_iterations = 5;

    while !pages_to_scrape.is_empty() && i <= max_iterations {
        let page_to_scrape = pages_to_scrape.remove(0);

        let response = reqwest::blocking::get(page_to_scrape);
        let html_content = response.unwrap().text().unwrap();
        let document = scraper::Html::parse_document(&html_content);

        let html_pagination_link_selector = scraper::Selector::parse("a.page-numbers").unwrap();
        let html_pagination_links = document.select(&html_pagination_link_selector);

        for html_pagination_link in html_pagination_links {
            let pagination_url = html_pagination_link
                .value()
                .attr("href")
                .unwrap()
                .to_owned();

            if !pages_discovered.contains(&pagination_url) {
                pages_discovered.insert(pagination_url.clone());

                if !pages_to_scrape.contains(&pagination_url) {
                    pages_to_scrape.push(pagination_url.clone());
                }
            }
        }

        i += 1;
        println!("{i}");
        println!("{pages_discovered:?}");
        println!("{pages_to_scrape:?}");
    }
}

fn take_screenshot() {
    let browser = headless_chrome::Browser::default().unwrap();
    let tab = browser.new_tab().unwrap();
    tab.navigate_to("https://scrapeme.live/shop/").unwrap();

    let screenshot_data = tab
        .capture_screenshot(
            headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption::Png,
            None,
            None,
            true,
        )
        .unwrap();

    std::fs::write("screenshot.png", screenshot_data).unwrap();
}

fn main() {
    scrape_to_csv();
    crawling_pages();
    take_screenshot();
}

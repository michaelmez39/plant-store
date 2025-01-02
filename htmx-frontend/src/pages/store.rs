use crate::{
    components::{notification, Color, PageWrapper},
    utils::display_decimal,
};
use axum::extract::Path;
use maud::{html, Markup};
use store_lib::store::ItemListing;
use tracing::info;
use uuid::Uuid;

pub async fn store(page: PageWrapper) -> Markup {
    page.render(page_body().await)
}

async fn page_body() -> Markup {
    html! {
        section.hero {
            .hero-body {
                h2.title { "Plantomics" }
                h4.subtitle { "The best house plants you can find" }
            }
        }
        .container {
            section.block {
                h3.is-6 { "Available Rocks" }
                (rock_scroller().await)
            }
        }
    }
}

async fn rock_listing(rock: &ItemListing) -> Markup {
    html! {
        .card {
            .card-image {
                .figure {
                    img.img src=(format!("/assets/images/{}", rock.image));
                }
            }
            .card-content {
                .media {
                    .media-content {
                        h4.title.is-5 { (rock.name) }
                        h6.subtitle.is-6.has-text-gray-light {
                            (display_decimal(rock.price))
                        }
                    }
                    .media-right {
                        button.is-size-4 title="Add to cart"
                        hx-put=(format!("/add-to-cart/{}", rock.listing_id))
                        hx-target="#notifications"
                        hx-swap="afterbegin"
                        {
                            b { "+" }
                        }
                    }
                }
            }
        }
    }
}

pub async fn add_to_cart(Path(id): Path<Uuid>) -> Markup {
    info!("Listing item {} added to a cart", id);
    notification("Added to cart!", Color::Success, true).await
}

pub async fn rock_list(Path(page): Path<usize>) -> Markup {
    let rocks: Vec<ItemListing> = (0..12).map(|_| ItemListing::random()).collect();
    html! {
        @for rock in &rocks {
            .cell { (rock_listing(rock).await) }
        }
        #replaceMe.level.cell.is-full-cell.mx-auto {
            button.button.mb-2
                hx-target="#replaceMe"
                hx-swap="outerHTML"
                hx-get=(format!("/rock-list/{}", page + 1)) {
                    "Load More"
                }
        }
    }
}

async fn rock_scroller() -> Markup {
    html! {
        #rocks.grid.is-col-min-10.is-row-gap-2.mx-2 {
            (rock_list(Path(1)).await)
        }
    }
}

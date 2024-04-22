use axum::extract::Path;
use maud::{Markup, html};
use tracing::info;
use uuid::Uuid;

use crate::{ItemListing, components::{page_wrapper, notification, Color}};

pub async fn store() -> Markup {
    html!((page_wrapper(page_body().await, true).await))
}

async fn page_body() -> Markup {
    html! {
        .container {
            #notifications;
            section.block{
                h2.title.is-2 { "Plants and Gems" }
                h4.subtitle.is-4 { "The best of both in one place" }
            }
            section.block {
                h3.title.is-3 { "Available Rocks" }
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
                    img.img src=(format!("http://localhost:3000/assets/{}", rock.image));
                }
            }
            .card-content {
                .media {
                    .media-content {
                        h4.title.is-5 { (rock.name) }
                        h6.subtitle.is-6.has-text-gray-light {
                            (format!("${}.{}{}", rock.price / 100, (rock.price / 10) % 10, rock.price % 10))
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


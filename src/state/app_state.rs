use std::sync::Arc;

use crate::utils::user_store::UserStore;
use crate::utils::orders_store::OrdersStore;
use crate::utils::favorites_store::FavoritesStore;
use crate::utils::cart_store::CartStore;
use crate::utils::promo_codes_store::PromoCodesStore;

#[allow(dead_code)]
pub struct AppState {
    pub users_store: Arc<UserStore>,
    pub orders_store: Arc<OrdersStore>,
    pub favorites_store: Arc<FavoritesStore>,
    pub carts_store: Arc<CartStore>,
    pub promocodes_store: Arc<PromoCodesStore>,
}

impl AppState {
    pub fn new(
        users_store: Arc<UserStore>,
        orders_store: Arc<OrdersStore>,
        favorites_store: Arc<FavoritesStore>,
        carts_store: Arc<CartStore>,
        promocodes_store: Arc<PromoCodesStore>,
    ) -> Self {
        AppState {
            users_store,
            orders_store,
            favorites_store,
            carts_store,
            promocodes_store,
        }
    }
}
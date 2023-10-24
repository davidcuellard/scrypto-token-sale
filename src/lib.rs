use scrypto::prelude::*;

#[blueprint]
mod token_sale {
    //Create access rules
    enable_method_auth! {
        roles {
            admin => updatable_by: [OWNER];
        },
        methods {
            buy => PUBLIC;
            withdraw_funds => restrict_to: [admin, OWNER];
            change_price => restrict_to: [admin, OWNER];
            withdraw_all_nb => PUBLIC;
        }
    }

    struct TokenSale {
        // The vault where the DCTokens will be stored.
        dc_tokens_vault: Vault,
        // The vault where the xrd payments will be stored.
        xrd_tokens_vault: Vault,
        // The price of a single DCToken.
        price_per_token: Decimal
    }

    impl TokenSale {
        // Implement the functions and methods which will manage those resources and data

        // This is a function, and can be called directly on the blueprint once deployed
        pub fn new(price_per_token: Decimal) -> (Global<TokenSale>, FungibleBucket, FungibleBucket) {

            // Create Owner badge
            let owner_badge = ResourceBuilder::new_fungible(OwnerRole::None) // #1
                .metadata(metadata!(init{"name"=>"owner badge", locked;}))
                .divisibility(DIVISIBILITY_NONE)
                .mint_initial_supply(1);

            // Create Admin badge
            // Owner role updatable by Owner
            let admin_badge = ResourceBuilder::new_fungible(OwnerRole::Updatable(rule!(require( // #2
                owner_badge.resource_address()
                ))))
                .metadata(metadata!(init{"name"=>"admin badge", locked;}))
                .divisibility(DIVISIBILITY_NONE)
                .mint_initial_supply(1);

            
            // Create DC Token
            let my_bucket: Bucket = ResourceBuilder::new_fungible(OwnerRole::None)
                .metadata(metadata! {
                    init {
                        "name" => "DCToken", locked;
                        "symbol" => "DCT", locked;
                    }
                })
                .mint_initial_supply(1000)
                .into();


            // Instantiate a TokenSales component
            let component_address = Self {
                dc_tokens_vault: Vault::with_bucket(my_bucket),
                xrd_tokens_vault: Vault::new(XRD),
                price_per_token: price_per_token
                }
                .instantiate()
                .prepare_to_globalize(
                    OwnerRole::Fixed(rule!(require(owner_badge.resource_address())))
                )
                .roles(roles!( 
                    admin => rule!(require(admin_badge.resource_address())); 
                ))
                .globalize();

            (component_address, admin_badge, owner_badge)

        }

        // Create method functions, because it needs a reference to self.  Methods can only be called on components
        pub fn buy(&mut self, funds: Bucket) -> Bucket {
            let purchase_amount: Decimal = funds.amount() / self.price_per_token;
            self.xrd_tokens_vault.put(funds);
            self.dc_tokens_vault.take(purchase_amount)
        }

        pub fn withdraw_funds(&mut self, amount: Decimal) -> Bucket {
            self.xrd_tokens_vault.take(amount)
        }

        pub fn change_price(&mut self, price: Decimal) {
            self.price_per_token = price;
        }

        pub fn withdraw_all_nb(&mut self) -> Bucket {
            self.xrd_tokens_vault.take_all()
        }
    }
}

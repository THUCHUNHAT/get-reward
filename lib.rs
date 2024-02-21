#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod data;
mod errors;
mod testing;
mod traits;
pub use data::{PSP22Data, PSP22Event};
pub use errors::PSP22Error;
pub use traits::{PSP22Burnable, PSP22Metadata, PSP22Mintable, PSP22};

#[ink::contract]
mod token {
    use crate::{PSP22Data, PSP22Error, PSP22Event, PSP22Metadata, PSP22};
    use ink::prelude::{string::String, vec::Vec};
    // use ink::env::debug_println;
    use ink::env::debug_println;

    #[ink(storage)]
    pub struct Token {
        data: PSP22Data, // (1)
        name: Option<String>,
        symbol: Option<String>,
        decimals: u8,
    }

    impl Token {
        #[ink(constructor)]
        pub fn new(
            supply: u128,
            name: Option<String>,
            symbol: Option<String>,
            decimals: u8,
        ) -> Self {
            Self {
                data: PSP22Data::new(supply, Self::env().caller()), // (2)
                name,
                symbol,
                decimals,
            }
        }

        // A helper function translating a vector of PSP22Events into the proper
        // ink event types (defined internally in this contract) and emitting them.
        // (5)
        fn emit_events(&self, events: Vec<PSP22Event>) {
            for event in events {
                match event {
                    PSP22Event::Transfer { from, to, value } => {
                        self.env().emit_event(Transfer { from, to, value })
                    }
                    PSP22Event::Approval {
                        owner,
                        spender,
                        amount,
                    } => self.env().emit_event(Approval {
                        owner,
                        spender,
                        amount,
                    }),
                }
            }
        }

        // #[ink(message)]
        // #[ink(payable)]
        // pub fn get_reward(&mut self, amount: u128) -> Result<(), PSP22Error> {
        //     let caller = self.env().caller();
        //     let current_balance = self.env().transferred_value();

        //     if current_balance < amount {
        //         return Err(PSP22Error::Custom(String::from("Không đủ số dư để nhận thưởng")));
        //     }
        //     // self.tran(amount)?;
        //     self.data.mint(caller, amount)?;

        //     Ok(())
        // }

        // #[ink(message)]
        // #[ink(payable)]
        // pub fn get_reward(&mut self) -> Result<(), PSP22Error> {
        //     let caller = self.env().caller();
        //     let t0_amount: Balance = 10;

        //     let current_balance: Balance = self.env().transferred_value();

        //     if  current_balance < 10000000000000 {
        //         return Err(PSP22Error::Custom(String::from("Không đủ số dư để nhận thưởng")));
        //     }

        //     self.data.mint(caller, t0_amount)?;

        //     Ok(())
        // }

        //         #[ink(message)]
        // #[ink(payable)]
        // pub fn get_reward(&mut self) -> Result<(), PSP22Error> {
        //     let caller = self.env().caller();
        //     let t0_amount: Balance = 10; // Số lượng T0 mặc định
        //     let current_balance: Balance = 10; // Giá trị đã chuyển mặc định

        //     if current_balance < t0_amount {
        //         return Err(PSP22Error::Custom(String::from("Không đủ số dư để nhận thưởng")));
        //     }

        //     self.data.mint(caller, t0_amount)?;

        //     Ok(())
        // }

        // #[ink(message)]
        // #[ink(payable)]
        // pub fn get_reward(&mut self) -> Result<(), PSP22Error> {
        //     let caller = self.env().caller();
        //     let t0_amount: Balance = 10;

        //     let current_balance: Balance = self.env().transferred_value();

        //     if current_balance < 10000000000000 {
        //         return Err(PSP22Error::Custom(String::from("Không đủ số dư để nhận thưởng")));
        //     }

        //     self.data.mint(caller, t0_amount)?;

        //     let random_number = (self.env().block_timestamp() % 10000000) as u128 % 11;
        //     let reward_amount = 10 + random_number;
        //     self.data.mint(caller, reward_amount)?;
        //     debug_println!("Người gọi nhận được {} token", reward_amount);

        //     Ok(())
        // }

        #[ink(message)]
        #[ink(payable)]
        pub fn get_reward(&mut self) -> Result<(), PSP22Error> {
            let caller = self.env().caller();
            let current_balance: Balance = self.env().transferred_value();
            let required_t0: Balance = 10000000000000;
            if current_balance < required_t0 {
                return Err(PSP22Error::Custom(String::from(
                    "Không đủ số dư để nhận thưởng",
                )));
            }
            let random_number = (self.env().block_timestamp() % 10000000) as u128 % 11;
            let reward_amount = 10 + random_number;
            self.data.mint(caller, reward_amount)?;
            debug_println!("Người gọi nhận được {} token", reward_amount);

            Ok(())
        }

        
    }

    // (3)
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        amount: u128,
    }

    // (3)
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: u128,
    }

    // (4)
    impl PSP22 for Token {
        #[ink(message)]
        fn total_supply(&self) -> u128 {
            self.data.total_supply()
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> u128 {
            self.data.balance_of(owner)
        }

        #[ink(message)]
        fn allowance(&self, owner: AccountId, spender: AccountId) -> u128 {
            self.data.allowance(owner, spender)
        }

        #[ink(message)]
        fn transfer(
            &mut self,
            to: AccountId,
            value: u128,
            _data: Vec<u8>,
        ) -> Result<(), PSP22Error> {
            let events = self.data.transfer(self.env().caller(), to, value)?;
            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: u128,
            _data: Vec<u8>,
        ) -> Result<(), PSP22Error> {
            let events = self
                .data
                .transfer_from(self.env().caller(), from, to, value)?;
            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn approve(&mut self, spender: AccountId, value: u128) -> Result<(), PSP22Error> {
            let events = self.data.approve(self.env().caller(), spender, value)?;
            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn increase_allowance(
            &mut self,
            spender: AccountId,
            delta_value: u128,
        ) -> Result<(), PSP22Error> {
            let events = self
                .data
                .increase_allowance(self.env().caller(), spender, delta_value)?;
            self.emit_events(events);
            Ok(())
        }

        #[ink(message)]
        fn decrease_allowance(
            &mut self,
            spender: AccountId,
            delta_value: u128,
        ) -> Result<(), PSP22Error> {
            let events = self
                .data
                .decrease_allowance(self.env().caller(), spender, delta_value)?;
            self.emit_events(events);
            Ok(())
        }
    }

    // (6)
    impl PSP22Metadata for Token {
        #[ink(message)]
        fn token_name(&self) -> Option<String> {
            self.name.clone()
        }
        #[ink(message)]
        fn token_symbol(&self) -> Option<String> {
            self.symbol.clone()
        }
        #[ink(message)]
        fn token_decimals(&self) -> u8 {
            self.decimals
        }
    }

    // (7)
    #[cfg(test)]
    mod tests {
        crate::tests!(Token, (|supply| Token::new(supply, None, None, 0)));
    }
}

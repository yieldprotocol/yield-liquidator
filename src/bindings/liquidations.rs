pub use liquidations_mod::*;
mod liquidations_mod {
    #![allow(dead_code)]
    #![allow(unused_imports)]
    use ethers::{
        contract::{
            builders::{ContractCall, Event},
            Contract, Lazy,
        },
        core::{
            abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
            types::*,
        },
        providers::JsonRpcClient,
        signers::{Client, Signer},
    };
    #[doc = "Liquidations was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static LIQUIDATIONS_ABI: Lazy<Abi> = Lazy::new(|| {
        serde_json :: from_str ( "[\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"dai_\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"treasury_\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"controller_\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"constructor\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"user\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"delegate\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"bool\",\n          \"name\": \"enabled\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"name\": \"Delegate\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": false,\n          \"internalType\": \"address\",\n          \"name\": \"access\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"GrantedAccess\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"user\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"started\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"collateral\",\n          \"type\": \"uint256\"\n        },\n        {\n          \"indexed\": false,\n          \"internalType\": \"uint256\",\n          \"name\": \"debt\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"Liquidation\",\n      \"type\": \"event\"\n    },\n    {\n      \"anonymous\": false,\n      \"inputs\": [\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"previousOwner\",\n          \"type\": \"address\"\n        },\n        {\n          \"indexed\": true,\n          \"internalType\": \"address\",\n          \"name\": \"newOwner\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"OwnershipTransferred\",\n      \"type\": \"event\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"AUCTION_TIME\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"DUST\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"FEE\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint128\",\n          \"name\": \"\",\n          \"type\": \"uint128\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"UNIT\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"WETH\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bytes32\",\n          \"name\": \"\",\n          \"type\": \"bytes32\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"user\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"aboveDustOrZero\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"delegate\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"addDelegate\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"authorized\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"from\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"to\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"liquidated\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"daiAmount\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"buy\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"delegated\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"user\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"erase\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint128\",\n          \"name\": \"\",\n          \"type\": \"uint128\"\n        },\n        {\n          \"internalType\": \"uint128\",\n          \"name\": \"\",\n          \"type\": \"uint128\"\n        }\n      ],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"user\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"to\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"liquidate\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"liquidations\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"live\",\n      \"outputs\": [\n        {\n          \"internalType\": \"bool\",\n          \"name\": \"\",\n          \"type\": \"bool\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"user\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"orchestrate\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"owner\",\n      \"outputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"user\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"price\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"renounceOwnership\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"delegate\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"revokeDelegate\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"shutdown\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [],\n      \"name\": \"totals\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint128\",\n          \"name\": \"collateral\",\n          \"type\": \"uint128\"\n        },\n        {\n          \"internalType\": \"uint128\",\n          \"name\": \"debt\",\n          \"type\": \"uint128\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"newOwner\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"transferOwnership\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"\",\n          \"type\": \"address\"\n        }\n      ],\n      \"name\": \"vaults\",\n      \"outputs\": [\n        {\n          \"internalType\": \"uint128\",\n          \"name\": \"collateral\",\n          \"type\": \"uint128\"\n        },\n        {\n          \"internalType\": \"uint128\",\n          \"name\": \"debt\",\n          \"type\": \"uint128\"\n        }\n      ],\n      \"stateMutability\": \"view\",\n      \"type\": \"function\"\n    },\n    {\n      \"inputs\": [\n        {\n          \"internalType\": \"address\",\n          \"name\": \"from\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"address\",\n          \"name\": \"to\",\n          \"type\": \"address\"\n        },\n        {\n          \"internalType\": \"uint256\",\n          \"name\": \"tokenAmount\",\n          \"type\": \"uint256\"\n        }\n      ],\n      \"name\": \"withdraw\",\n      \"outputs\": [],\n      \"stateMutability\": \"nonpayable\",\n      \"type\": \"function\"\n    }\n]\n" ) . expect ( "invalid abi" )
    });
    #[derive(Clone)]
    pub struct Liquidations<P, S>(Contract<P, S>);
    impl<P, S> std::ops::Deref for Liquidations<P, S> {
        type Target = Contract<P, S>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<P: JsonRpcClient, S: Signer> std::fmt::Debug for Liquidations<P, S> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Liquidations))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, P: JsonRpcClient, S: Signer> Liquidations<P, S> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<Address>, C: Into<Arc<Client<P, S>>>>(address: T, client: C) -> Self {
            let contract = Contract::new(address.into(), LIQUIDATIONS_ABI.clone(), client.into());
            Self(contract)
        }
        #[doc = "Calls the contract's `WETH` (0xad5c4648) function"]
        pub fn weth(&self) -> ContractCall<P, S, [u8; 32]> {
            self.0
                .method_hash([173, 92, 70, 72], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addDelegate` (0xe71bdf41) function"]
        pub fn add_delegate(&self, delegate: Address) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([231, 27, 223, 65], delegate)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidations` (0x937d4c42) function"]
        pub fn liquidations(&self, p0: Address) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([147, 125, 76, 66], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `AUCTION_TIME` (0xe592301a) function"]
        pub fn auction_time(&self) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([229, 146, 48, 26], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `authorized` (0xb9181611) function"]
        pub fn authorized(&self, p0: Address) -> ContractCall<P, S, bool> {
            self.0
                .method_hash([185, 24, 22, 17], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `price` (0xaea91078) function"]
        pub fn price(&self, user: Address) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([174, 169, 16, 120], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `revokeDelegate` (0xfa352c00) function"]
        pub fn revoke_delegate(&self, delegate: Address) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([250, 53, 44, 0], delegate)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `FEE` (0xc57981b5) function"]
        pub fn fee(&self) -> ContractCall<P, S, u128> {
            self.0
                .method_hash([197, 121, 129, 181], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `live` (0x957aa58c) function"]
        pub fn live(&self) -> ContractCall<P, S, bool> {
            self.0
                .method_hash([149, 122, 165, 140], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `UNIT` (0x9d8e2177) function"]
        pub fn unit(&self) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([157, 142, 33, 119], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(&self) -> ContractCall<P, S, Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `buy` (0xe8cdcc99) function"]
        pub fn buy(
            &self,
            from: Address,
            to: Address,
            liquidated: Address,
            dai_amount: U256,
        ) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([232, 205, 204, 153], (from, to, liquidated, dai_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `delegated` (0xf6bcbd31) function"]
        pub fn delegated(&self, p0: Address, p1: Address) -> ContractCall<P, S, bool> {
            self.0
                .method_hash([246, 188, 189, 49], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `liquidate` (0x86b9d81f) function"]
        pub fn liquidate(&self, user: Address, to: Address) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([134, 185, 216, 31], (user, to))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `shutdown` (0xfc0e74d1) function"]
        pub fn shutdown(&self) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([252, 14, 116, 209], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(&self, new_owner: Address) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `erase` (0xe85b0080) function"]
        pub fn erase(&self, user: Address) -> ContractCall<P, S, (u128, u128)> {
            self.0
                .method_hash([232, 91, 0, 128], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `orchestrate` (0x80f5a440) function"]
        pub fn orchestrate(&self, user: Address) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([128, 245, 164, 64], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `vaults` (0xa622ee7c) function"]
        pub fn vaults(&self, p0: Address) -> ContractCall<P, S, (u128, u128)> {
            self.0
                .method_hash([166, 34, 238, 124], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `aboveDustOrZero` (0xe5442a9e) function"]
        pub fn above_dust_or_zero(&self, user: Address) -> ContractCall<P, S, bool> {
            self.0
                .method_hash([229, 68, 42, 158], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `DUST` (0x4e0cd799) function"]
        pub fn dust(&self) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([78, 12, 215, 153], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0xd9caed12) function"]
        pub fn withdraw(
            &self,
            from: Address,
            to: Address,
            token_amount: U256,
        ) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([217, 202, 237, 18], (from, to, token_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totals` (0xc038a38e) function"]
        pub fn totals(&self) -> ContractCall<P, S, (u128, u128)> {
            self.0
                .method_hash([192, 56, 163, 142], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `GrantedAccess` event"]
        pub fn granted_access_filter(&self) -> Event<P, GrantedAccessFilter> {
            self.0
                .event("GrantedAccess")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Liquidation` event"]
        pub fn liquidation_filter(&self) -> Event<P, LiquidationFilter> {
            self.0
                .event("Liquidation")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(&self) -> Event<P, OwnershipTransferredFilter> {
            self.0
                .event("OwnershipTransferred")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Delegate` event"]
        pub fn delegate_filter(&self) -> Event<P, DelegateFilter> {
            self.0
                .event("Delegate")
                .expect("event not found (this should never happen)")
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct GrantedAccessFilter {
        pub access: Address,
    }
    impl GrantedAccessFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                106, 124, 95, 40, 234, 86, 79, 175, 200, 176, 121, 102, 58, 42, 155, 157, 38, 254,
                115, 93, 151, 201, 118, 17, 72, 76, 224, 68, 244, 10, 22, 226,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`GrantedAccess(address)`"]
        pub const fn abi_signature() -> &'static str {
            "GrantedAccess(address)"
        }
    }
    impl Detokenize for GrantedAccessFilter {
        fn from_tokens(tokens: Vec<Token>) -> Result<Self, InvalidOutputType> {
            if tokens.len() != 1 {
                return Err(InvalidOutputType(format!(
                    "Expected {} tokens, got {}: {:?}",
                    1,
                    tokens.len(),
                    tokens
                )));
            }
            #[allow(unused_mut)]
            let mut tokens = tokens.into_iter();
            let access = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(GrantedAccessFilter { access })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct LiquidationFilter {
        pub user: Address,
        pub started: U256,
        pub collateral: U256,
        pub debt: U256,
    }
    impl LiquidationFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                26, 16, 114, 235, 152, 66, 79, 81, 217, 173, 86, 82, 111, 5, 205, 130, 220, 70,
                248, 150, 90, 150, 183, 196, 33, 241, 130, 117, 117, 61, 36, 140,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`Liquidation(address,uint256,uint256,uint256)`"]
        pub const fn abi_signature() -> &'static str {
            "Liquidation(address,uint256,uint256,uint256)"
        }
    }
    impl Detokenize for LiquidationFilter {
        fn from_tokens(tokens: Vec<Token>) -> Result<Self, InvalidOutputType> {
            if tokens.len() != 4 {
                return Err(InvalidOutputType(format!(
                    "Expected {} tokens, got {}: {:?}",
                    4,
                    tokens.len(),
                    tokens
                )));
            }
            #[allow(unused_mut)]
            let mut tokens = tokens.into_iter();
            let user = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let started =
                Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let collateral =
                Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let debt = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(LiquidationFilter {
                user,
                started,
                collateral,
                debt,
            })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct OwnershipTransferredFilter {
        pub previous_owner: Address,
        pub new_owner: Address,
    }
    impl OwnershipTransferredFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                139, 224, 7, 156, 83, 22, 89, 20, 19, 68, 205, 31, 208, 164, 242, 132, 25, 73, 127,
                151, 34, 163, 218, 175, 227, 180, 24, 111, 107, 100, 87, 224,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`OwnershipTransferred(address,address)`"]
        pub const fn abi_signature() -> &'static str {
            "OwnershipTransferred(address,address)"
        }
    }
    impl Detokenize for OwnershipTransferredFilter {
        fn from_tokens(tokens: Vec<Token>) -> Result<Self, InvalidOutputType> {
            if tokens.len() != 2 {
                return Err(InvalidOutputType(format!(
                    "Expected {} tokens, got {}: {:?}",
                    2,
                    tokens.len(),
                    tokens
                )));
            }
            #[allow(unused_mut)]
            let mut tokens = tokens.into_iter();
            let previous_owner =
                Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let new_owner =
                Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(OwnershipTransferredFilter {
                previous_owner,
                new_owner,
            })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct DelegateFilter {
        pub user: Address,
        pub delegate: Address,
        pub enabled: bool,
    }
    impl DelegateFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                4, 91, 15, 239, 1, 119, 45, 47, 187, 165, 61, 189, 56, 201, 119, 120, 6, 234, 192,
                134, 91, 0, 175, 67, 171, 207, 188, 175, 80, 218, 146, 6,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`Delegate(address,address,bool)`"]
        pub const fn abi_signature() -> &'static str {
            "Delegate(address,address,bool)"
        }
    }
    impl Detokenize for DelegateFilter {
        fn from_tokens(tokens: Vec<Token>) -> Result<Self, InvalidOutputType> {
            if tokens.len() != 3 {
                return Err(InvalidOutputType(format!(
                    "Expected {} tokens, got {}: {:?}",
                    3,
                    tokens.len(),
                    tokens
                )));
            }
            #[allow(unused_mut)]
            let mut tokens = tokens.into_iter();
            let user = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let delegate =
                Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let enabled =
                Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(DelegateFilter {
                user,
                delegate,
                enabled,
            })
        }
    }
}

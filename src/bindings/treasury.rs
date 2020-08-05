pub use treasury_mod::*;
mod treasury_mod {
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
    #[doc = "Treasury was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static TREASURY_ABI: Lazy<Abi> = Lazy::new(|| {
        serde_json :: from_str ( "[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"vat_\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"weth_\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"dai_\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"wethJoin_\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"daiJoin_\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"pot_\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"chai_\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"access\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"GrantedAccess\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"previousOwner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"newOwner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"OwnershipTransferred\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"UNIT\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"authorized\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"debt\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"live\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"orchestrate\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"owner\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"power\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"chaiAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"pullChai\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"daiAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"pullDai\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"wethAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"pullWeth\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"chaiAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"pushChai\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"daiAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"pushDai\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"wethAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"pushWeth\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"unwind_\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"registerUnwind\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"renounceOwnership\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"savings\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"shutdown\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"newOwner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"transferOwnership\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]\n" ) . expect ( "invalid abi" )
    });
    #[derive(Clone)]
    pub struct Treasury<P, S>(Contract<P, S>);
    impl<P, S> std::ops::Deref for Treasury<P, S> {
        type Target = Contract<P, S>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<P: JsonRpcClient, S: Signer> std::fmt::Debug for Treasury<P, S> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Treasury))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, P: JsonRpcClient, S: Signer> Treasury<P, S> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<Address>, C: Into<Arc<Client<P, S>>>>(address: T, client: C) -> Self {
            let contract = Contract::new(address.into(), TREASURY_ABI.clone(), client.into());
            Self(contract)
        }
        #[doc = "Calls the contract's `debt` (0x0dca59c1) function"]
        pub fn debt(&self) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([13, 202, 89, 193], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `savings` (0xc86d6ae4) function"]
        pub fn savings(&self) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([200, 109, 106, 228], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `UNIT` (0x9d8e2177) function"]
        pub fn unit(&self) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([157, 142, 33, 119], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `orchestrate` (0x80f5a440) function"]
        pub fn orchestrate(&self, user: Address) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([128, 245, 164, 64], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pushDai` (0xf9391991) function"]
        pub fn push_dai(&self, from: Address, dai_amount: U256) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([249, 57, 25, 145], (from, dai_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `authorized` (0xb9181611) function"]
        pub fn authorized(&self, p0: Address) -> ContractCall<P, S, bool> {
            self.0
                .method_hash([185, 24, 22, 17], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `shutdown` (0xfc0e74d1) function"]
        pub fn shutdown(&self) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([252, 14, 116, 209], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pushWeth` (0x9a5aaf78) function"]
        pub fn push_weth(&self, from: Address, weth_amount: U256) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([154, 90, 175, 120], (from, weth_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pullWeth` (0x8ac96dfc) function"]
        pub fn pull_weth(&self, to: Address, weth_amount: U256) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([138, 201, 109, 252], (to, weth_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pushChai` (0x1997c361) function"]
        pub fn push_chai(&self, from: Address, chai_amount: U256) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([25, 151, 195, 97], (from, chai_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pullChai` (0x9fa83689) function"]
        pub fn pull_chai(&self, to: Address, chai_amount: U256) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([159, 168, 54, 137], (to, chai_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pullDai` (0x55027703) function"]
        pub fn pull_dai(&self, to: Address, dai_amount: U256) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([85, 2, 119, 3], (to, dai_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(&self, new_owner: Address) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `live` (0x957aa58c) function"]
        pub fn live(&self) -> ContractCall<P, S, bool> {
            self.0
                .method_hash([149, 122, 165, 140], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(&self) -> ContractCall<P, S, Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `registerUnwind` (0x1e2043b1) function"]
        pub fn register_unwind(&self, unwind: Address) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([30, 32, 67, 177], unwind)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `power` (0x4a4d59fa) function"]
        pub fn power(&self) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([74, 77, 89, 250], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(&self) -> Event<P, OwnershipTransferredFilter> {
            self.0
                .event("OwnershipTransferred")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `GrantedAccess` event"]
        pub fn granted_access_filter(&self) -> Event<P, GrantedAccessFilter> {
            self.0
                .event("GrantedAccess")
                .expect("event not found (this should never happen)")
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
}

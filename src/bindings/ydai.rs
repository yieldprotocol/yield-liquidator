pub use ydai_mod::*;
mod ydai_mod {
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
    #[doc = "YDai was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static YDAI_ABI: Lazy<Abi> = Lazy::new(|| {
        serde_json :: from_str ( "[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"vat_\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"pot_\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"treasury_\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"maturity_\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"string\",\n        \"name\": \"name\",\n        \"type\": \"string\"\n      },\n      {\n        \"internalType\": \"string\",\n        \"name\": \"symbol\",\n        \"type\": \"string\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"spender\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"value\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Approval\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"delegate\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bool\",\n        \"name\": \"enabled\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"Delegate\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"access\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"GrantedAccess\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"rate\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"chi\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Matured\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"previousOwner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"newOwner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"OwnershipTransferred\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"yDaiIn\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"daiOut\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Redeemed\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"value\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Transfer\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"DOMAIN_SEPARATOR\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"PERMIT_TYPEHASH\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"UNIT\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"WETH\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"delegate\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"addDelegate\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"spender\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"allowance\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"spender\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"approve\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"authorized\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"balanceOf\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"yDaiAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"burn\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"chi0\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"chiGrowth\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"decimals\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint8\",\n        \"name\": \"\",\n        \"type\": \"uint8\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"spender\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"subtractedValue\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"decreaseAllowance\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"delegated\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"yDaiAmount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"flashMint\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"spender\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"addedValue\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"increaseAllowance\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"isMature\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"mature\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"maturity\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"yDaiAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"mint\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"name\",\n    \"outputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"\",\n        \"type\": \"string\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"nonces\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"orchestrate\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"owner\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"spender\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"deadline\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint8\",\n        \"name\": \"v\",\n        \"type\": \"uint8\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"r\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"s\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"permit\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"rate0\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"rateGrowth\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"yDaiAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"redeem\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"renounceOwnership\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"delegate\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"revokeDelegate\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"symbol\",\n    \"outputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"\",\n        \"type\": \"string\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"totalSupply\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"transfer\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"transferFrom\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"newOwner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"transferOwnership\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]\n" ) . expect ( "invalid abi" )
    });
    #[derive(Clone)]
    pub struct YDai<P, S>(Contract<P, S>);
    impl<P, S> std::ops::Deref for YDai<P, S> {
        type Target = Contract<P, S>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<P: JsonRpcClient, S: Signer> std::fmt::Debug for YDai<P, S> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(YDai))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, P: JsonRpcClient, S: Signer> YDai<P, S> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<Address>, C: Into<Arc<Client<P, S>>>>(address: T, client: C) -> Self {
            let contract = Contract::new(address.into(), YDAI_ABI.clone(), client.into());
            Self(contract)
        }
        #[doc = "Calls the contract's `increaseAllowance` (0x39509351) function"]
        pub fn increase_allowance(
            &self,
            spender: Address,
            added_value: U256,
        ) -> ContractCall<P, S, bool> {
            self.0
                .method_hash([57, 80, 147, 81], (spender, added_value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decimals` (0x313ce567) function"]
        pub fn decimals(&self) -> ContractCall<P, S, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `redeem` (0x0e6dfcd5) function"]
        pub fn redeem(
            &self,
            from: Address,
            to: Address,
            y_dai_amount: U256,
        ) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([14, 109, 252, 213], (from, to, y_dai_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(&self) -> ContractCall<P, S, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `approve` (0x095ea7b3) function"]
        pub fn approve(&self, spender: Address, amount: U256) -> ContractCall<P, S, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `allowance` (0xdd62ed3e) function"]
        pub fn allowance(&self, owner: Address, spender: Address) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([221, 98, 237, 62], (owner, spender))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `UNIT` (0x9d8e2177) function"]
        pub fn unit(&self) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([157, 142, 33, 119], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalSupply` (0x18160ddd) function"]
        pub fn total_supply(&self) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            sender: Address,
            recipient: Address,
            amount: U256,
        ) -> ContractCall<P, S, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (sender, recipient, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `authorized` (0xb9181611) function"]
        pub fn authorized(&self, p0: Address) -> ContractCall<P, S, bool> {
            self.0
                .method_hash([185, 24, 22, 17], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(&self, account: Address) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mature` (0x87b65207) function"]
        pub fn mature(&self) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([135, 182, 82, 7], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rateGrowth` (0xabf1614b) function"]
        pub fn rate_growth(&self) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([171, 241, 97, 75], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nonces` (0x7ecebe00) function"]
        pub fn nonces(&self, p0: Address) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([126, 206, 190, 0], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rate0` (0x93c19e18) function"]
        pub fn rate_0(&self) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([147, 193, 158, 24], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `chi0` (0x12970eac) function"]
        pub fn chi_0(&self) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([18, 151, 14, 172], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isMature` (0xae4e7fdf) function"]
        pub fn is_mature(&self) -> ContractCall<P, S, bool> {
            self.0
                .method_hash([174, 78, 127, 223], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `PERMIT_TYPEHASH` (0x30adf81f) function"]
        pub fn permit_typehash(&self) -> ContractCall<P, S, [u8; 32]> {
            self.0
                .method_hash([48, 173, 248, 31], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `maturity` (0x204f83f9) function"]
        pub fn maturity(&self) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([32, 79, 131, 249], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `delegated` (0xf6bcbd31) function"]
        pub fn delegated(&self, p0: Address, p1: Address) -> ContractCall<P, S, bool> {
            self.0
                .method_hash([246, 188, 189, 49], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function"]
        pub fn domain_separator(&self) -> ContractCall<P, S, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `orchestrate` (0x80f5a440) function"]
        pub fn orchestrate(&self, user: Address) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([128, 245, 164, 64], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `permit` (0xd505accf) function"]
        pub fn permit(
            &self,
            owner: Address,
            spender: Address,
            amount: U256,
            deadline: U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ContractCall<P, S, ()> {
            self.0
                .method_hash(
                    [213, 5, 172, 207],
                    (owner, spender, amount, deadline, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mint` (0x40c10f19) function"]
        pub fn mint(&self, to: Address, y_dai_amount: U256) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([64, 193, 15, 25], (to, y_dai_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transfer` (0xa9059cbb) function"]
        pub fn transfer(&self, recipient: Address, amount: U256) -> ContractCall<P, S, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (recipient, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addDelegate` (0xe71bdf41) function"]
        pub fn add_delegate(&self, delegate: Address) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([231, 27, 223, 65], delegate)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `chiGrowth` (0xcb240198) function"]
        pub fn chi_growth(&self) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([203, 36, 1, 152], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `flashMint` (0x3ea21465) function"]
        pub fn flash_mint(
            &self,
            to: Address,
            y_dai_amount: U256,
            data: Vec<u8>,
        ) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([62, 162, 20, 101], (to, y_dai_amount, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(&self) -> ContractCall<P, S, Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `decreaseAllowance` (0xa457c2d7) function"]
        pub fn decrease_allowance(
            &self,
            spender: Address,
            subtracted_value: U256,
        ) -> ContractCall<P, S, bool> {
            self.0
                .method_hash([164, 87, 194, 215], (spender, subtracted_value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `revokeDelegate` (0xfa352c00) function"]
        pub fn revoke_delegate(&self, delegate: Address) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([250, 53, 44, 0], delegate)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `burn` (0x9dc29fac) function"]
        pub fn burn(&self, from: Address, y_dai_amount: U256) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([157, 194, 159, 172], (from, y_dai_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `WETH` (0xad5c4648) function"]
        pub fn weth(&self) -> ContractCall<P, S, [u8; 32]> {
            self.0
                .method_hash([173, 92, 70, 72], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ContractCall<P, S, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(&self, new_owner: Address) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> Event<P, TransferFilter> {
            self.0
                .event("Transfer")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Redeemed` event"]
        pub fn redeemed_filter(&self) -> Event<P, RedeemedFilter> {
            self.0
                .event("Redeemed")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Matured` event"]
        pub fn matured_filter(&self) -> Event<P, MaturedFilter> {
            self.0
                .event("Matured")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Delegate` event"]
        pub fn delegate_filter(&self) -> Event<P, DelegateFilter> {
            self.0
                .event("Delegate")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> Event<P, ApprovalFilter> {
            self.0
                .event("Approval")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `GrantedAccess` event"]
        pub fn granted_access_filter(&self) -> Event<P, GrantedAccessFilter> {
            self.0
                .event("GrantedAccess")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(&self) -> Event<P, OwnershipTransferredFilter> {
            self.0
                .event("OwnershipTransferred")
                .expect("event not found (this should never happen)")
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct TransferFilter {
        pub from: Address,
        pub to: Address,
        pub value: U256,
    }
    impl TransferFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                221, 242, 82, 173, 27, 226, 200, 155, 105, 194, 176, 104, 252, 55, 141, 170, 149,
                43, 167, 241, 99, 196, 161, 22, 40, 245, 90, 77, 245, 35, 179, 239,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`Transfer(address,address,uint256)`"]
        pub const fn abi_signature() -> &'static str {
            "Transfer(address,address,uint256)"
        }
    }
    impl Detokenize for TransferFilter {
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
            let from = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let to = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let value = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(TransferFilter { from, to, value })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct RedeemedFilter {
        pub from: Address,
        pub to: Address,
        pub y_dai_in: U256,
        pub dai_out: U256,
    }
    impl RedeemedFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                92, 223, 7, 173, 15, 194, 34, 68, 39, 32, 177, 8, 227, 237, 76, 70, 64, 240, 250,
                220, 42, 178, 37, 62, 102, 242, 89, 160, 254, 168, 52, 128,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`Redeemed(address,address,uint256,uint256)`"]
        pub const fn abi_signature() -> &'static str {
            "Redeemed(address,address,uint256,uint256)"
        }
    }
    impl Detokenize for RedeemedFilter {
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
            let from = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let to = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let y_dai_in =
                Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let dai_out =
                Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(RedeemedFilter {
                from,
                to,
                y_dai_in,
                dai_out,
            })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct MaturedFilter {
        pub rate: U256,
        pub chi: U256,
    }
    impl MaturedFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                48, 167, 66, 212, 81, 17, 41, 82, 3, 196, 184, 219, 97, 76, 139, 5, 221, 19, 35,
                167, 197, 21, 159, 184, 139, 12, 199, 59, 228, 107, 122, 117,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`Matured(uint256,uint256)`"]
        pub const fn abi_signature() -> &'static str {
            "Matured(uint256,uint256)"
        }
    }
    impl Detokenize for MaturedFilter {
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
            let rate = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let chi = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(MaturedFilter { rate, chi })
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
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct ApprovalFilter {
        pub owner: Address,
        pub spender: Address,
        pub value: U256,
    }
    impl ApprovalFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                140, 91, 225, 229, 235, 236, 125, 91, 209, 79, 113, 66, 125, 30, 132, 243, 221, 3,
                20, 192, 247, 178, 41, 30, 91, 32, 10, 200, 199, 195, 185, 37,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`Approval(address,address,uint256)`"]
        pub const fn abi_signature() -> &'static str {
            "Approval(address,address,uint256)"
        }
    }
    impl Detokenize for ApprovalFilter {
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
            let owner = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let spender =
                Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let value = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(ApprovalFilter {
                owner,
                spender,
                value,
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
}

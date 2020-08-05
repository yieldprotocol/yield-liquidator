pub use vat_mod::*;
mod vat_mod {
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
    #[doc = "Vat was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static VAT_ABI: Lazy<Abi> = Lazy::new(|| {
        serde_json :: from_str ( "[\n  {\n    \"inputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"anonymous\": true,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes4\",\n        \"name\": \"sig\",\n        \"type\": \"bytes4\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"arg1\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"arg2\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"arg3\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"LogNote\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"Line\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"cage\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"can\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"dai\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"debt\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"usr\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"deny\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"ilk\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"what\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"data\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"file\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"what\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"data\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"file\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"ilk\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"src\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"dst\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"wad\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"flux\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"i\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"u\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"int256\",\n        \"name\": \"rate\",\n        \"type\": \"int256\"\n      }\n    ],\n    \"name\": \"fold\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"ilk\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"src\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"dst\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"int256\",\n        \"name\": \"dink\",\n        \"type\": \"int256\"\n      },\n      {\n        \"internalType\": \"int256\",\n        \"name\": \"dart\",\n        \"type\": \"int256\"\n      }\n    ],\n    \"name\": \"fork\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"i\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"u\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"v\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"w\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"int256\",\n        \"name\": \"dink\",\n        \"type\": \"int256\"\n      },\n      {\n        \"internalType\": \"int256\",\n        \"name\": \"dart\",\n        \"type\": \"int256\"\n      }\n    ],\n    \"name\": \"frob\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"gem\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"i\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"u\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"v\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"w\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"int256\",\n        \"name\": \"dink\",\n        \"type\": \"int256\"\n      },\n      {\n        \"internalType\": \"int256\",\n        \"name\": \"dart\",\n        \"type\": \"int256\"\n      }\n    ],\n    \"name\": \"grab\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"rad\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"heal\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"usr\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"hope\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"ilks\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"Art\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"rate\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"spot\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"line\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"dust\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"ilk\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"init\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"live\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"src\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"dst\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"rad\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"move\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"usr\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"nope\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"usr\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"rely\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"sin\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"ilk\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"usr\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"int256\",\n        \"name\": \"wad\",\n        \"type\": \"int256\"\n      }\n    ],\n    \"name\": \"slip\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"u\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"v\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"rad\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"suck\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"urns\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"ink\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"art\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"vice\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"wards\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  }\n]\n" ) . expect ( "invalid abi" )
    });
    #[derive(Clone)]
    pub struct Vat<P, S>(Contract<P, S>);
    impl<P, S> std::ops::Deref for Vat<P, S> {
        type Target = Contract<P, S>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<P: JsonRpcClient, S: Signer> std::fmt::Debug for Vat<P, S> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Vat))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, P: JsonRpcClient, S: Signer> Vat<P, S> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<Address>, C: Into<Arc<Client<P, S>>>>(address: T, client: C) -> Self {
            let contract = Contract::new(address.into(), VAT_ABI.clone(), client.into());
            Self(contract)
        }
        #[doc = "Calls the contract's `frob` (0x76088703) function"]
        pub fn frob(
            &self,
            i: [u8; 32],
            u: Address,
            v: Address,
            w: Address,
            dink: I256,
            dart: I256,
        ) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([118, 8, 135, 3], (i, u, v, w, dink, dart))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `init` (0x3b663195) function"]
        pub fn init(&self, ilk: [u8; 32]) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([59, 102, 49, 149], ilk)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hope` (0xa3b22fc4) function"]
        pub fn hope(&self, usr: Address) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([163, 178, 47, 196], usr)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `grab` (0x7bab3f40) function"]
        pub fn grab(
            &self,
            i: [u8; 32],
            u: Address,
            v: Address,
            w: Address,
            dink: I256,
            dart: I256,
        ) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([123, 171, 63, 64], (i, u, v, w, dink, dart))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ilks` (0xd9638d36) function"]
        pub fn ilks(&self, p0: [u8; 32]) -> ContractCall<P, S, (U256, U256, U256, U256, U256)> {
            self.0
                .method_hash([217, 99, 141, 54], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `flux` (0x6111be2e) function"]
        pub fn flux(
            &self,
            ilk: [u8; 32],
            src: Address,
            dst: Address,
            wad: U256,
        ) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([97, 17, 190, 46], (ilk, src, dst, wad))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sin` (0xf059212a) function"]
        pub fn sin(&self, p0: Address) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([240, 89, 33, 42], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `gem` (0x214414d5) function"]
        pub fn gem(&self, p0: [u8; 32], p1: Address) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([33, 68, 20, 213], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `heal` (0xf37ac61c) function"]
        pub fn heal(&self, rad: U256) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([243, 122, 198, 28], rad)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `urns` (0x2424be5c) function"]
        pub fn urns(&self, p0: [u8; 32], p1: Address) -> ContractCall<P, S, (U256, U256)> {
            self.0
                .method_hash([36, 36, 190, 92], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `file` (0x1a0b287e) function"]
        pub fn file(&self, ilk: [u8; 32], what: [u8; 32], data: U256) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([26, 11, 40, 126], (ilk, what, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `file` (0x29ae8114) function"]
        pub fn file(&self, what: [u8; 32], data: U256) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([41, 174, 129, 20], (what, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nope` (0xdc4d20fa) function"]
        pub fn nope(&self, usr: Address) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([220, 77, 32, 250], usr)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `suck` (0xf24e23eb) function"]
        pub fn suck(&self, u: Address, v: Address, rad: U256) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([242, 78, 35, 235], (u, v, rad))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `debt` (0x0dca59c1) function"]
        pub fn debt(&self) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([13, 202, 89, 193], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `wards` (0xbf353dbb) function"]
        pub fn wards(&self, p0: Address) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([191, 53, 61, 187], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `can` (0x4538c4eb) function"]
        pub fn can(&self, p0: Address, p1: Address) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([69, 56, 196, 235], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deny` (0x9c52a7f1) function"]
        pub fn deny(&self, usr: Address) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([156, 82, 167, 241], usr)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `cage` (0x69245009) function"]
        pub fn cage(&self) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([105, 36, 80, 9], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `slip` (0x7cdd3fde) function"]
        pub fn slip(&self, ilk: [u8; 32], usr: Address, wad: I256) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([124, 221, 63, 222], (ilk, usr, wad))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `move` (0xbb35783b) function"]
        pub fn move_(&self, src: Address, dst: Address, rad: U256) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([187, 53, 120, 59], (src, dst, rad))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `dai` (0x6c25b346) function"]
        pub fn dai(&self, p0: Address) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([108, 37, 179, 70], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fold` (0xb65337df) function"]
        pub fn fold(&self, i: [u8; 32], u: Address, rate: I256) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([182, 83, 55, 223], (i, u, rate))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fork` (0x870c616d) function"]
        pub fn fork(
            &self,
            ilk: [u8; 32],
            src: Address,
            dst: Address,
            dink: I256,
            dart: I256,
        ) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([135, 12, 97, 109], (ilk, src, dst, dink, dart))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `vice` (0x2d61a355) function"]
        pub fn vice(&self) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([45, 97, 163, 85], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `live` (0x957aa58c) function"]
        pub fn live(&self) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([149, 122, 165, 140], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `Line` (0xbabe8a3f) function"]
        pub fn line(&self) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([186, 190, 138, 63], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rely` (0x65fae35e) function"]
        pub fn rely(&self, usr: Address) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([101, 250, 227, 94], usr)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `LogNote` event"]
        pub fn log_note_filter(&self) -> Event<P, LogNoteFilter> {
            self.0
                .event("LogNote")
                .expect("event not found (this should never happen)")
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct LogNoteFilter {
        pub sig: [u8; 4],
        pub arg_1: [u8; 32],
        pub arg_2: [u8; 32],
        pub arg_3: [u8; 32],
        pub data: Vec<u8>,
    }
    impl LogNoteFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                211, 255, 48, 249, 75, 180, 235, 180, 243, 215, 115, 234, 38, 182, 239, 199, 50,
                139, 151, 102, 249, 159, 25, 223, 246, 240, 19, 146, 19, 139, 228, 109,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`LogNote(bytes4,bytes32,bytes32,bytes32,bytes) anonymous`"]
        pub const fn abi_signature() -> &'static str {
            "LogNote(bytes4,bytes32,bytes32,bytes32,bytes) anonymous"
        }
    }
    impl Detokenize for LogNoteFilter {
        fn from_tokens(tokens: Vec<Token>) -> Result<Self, InvalidOutputType> {
            if tokens.len() != 5 {
                return Err(InvalidOutputType(format!(
                    "Expected {} tokens, got {}: {:?}",
                    5,
                    tokens.len(),
                    tokens
                )));
            }
            #[allow(unused_mut)]
            let mut tokens = tokens.into_iter();
            let sig = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let arg_1 = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let arg_2 = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let arg_3 = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let data = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(LogNoteFilter {
                sig,
                arg_1,
                arg_2,
                arg_3,
                data,
            })
        }
    }
}

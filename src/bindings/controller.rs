pub use controller_mod::*;
mod controller_mod {
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
    #[doc = "Controller was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CONTROLLER_ABI: Lazy<Abi> = Lazy::new(|| {
        serde_json :: from_str ( "[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"vat_\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"pot_\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"treasury_\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"collateral\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"uint256\",\n        \"name\": \"maturity\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"int256\",\n        \"name\": \"amount\",\n        \"type\": \"int256\"\n      }\n    ],\n    \"name\": \"Borrowed\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"delegate\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bool\",\n        \"name\": \"enabled\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"Delegate\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"access\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"GrantedAccess\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"previousOwner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"newOwner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"OwnershipTransferred\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"collateral\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"int256\",\n        \"name\": \"amount\",\n        \"type\": \"int256\"\n      }\n    ],\n    \"name\": \"Posted\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"CHAI\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"DUST\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"THREE_MONTHS\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"UNIT\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"WETH\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"collateral\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"aboveDustOrZero\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"delegate\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"addDelegate\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"yDaiContract\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"addSeries\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"authorized\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"collateral\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"maturity\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"yDaiAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"borrow\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"maturity\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"containsSeries\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"collateral\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"maturity\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"debtDai\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"debtYDai\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"delegated\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"collateral\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"erase\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"collateral\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"maturity\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"yDaiAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"inDai\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"collateral\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"maturity\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"daiAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"inYDai\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"collateral\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"isCollateralized\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"live\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"collateral\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"locked\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"orchestrate\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"owner\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"collateral\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"post\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"posted\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"collateral\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"powerOf\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"renounceOwnership\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"collateral\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"maturity\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"daiAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"repayDai\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"collateral\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"maturity\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"yDaiAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"repayYDai\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"delegate\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"revokeDelegate\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"series\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract IYDai\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"seriesIterator\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"shutdown\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"collateral\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"user\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"totalDebtDai\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"totalSeries\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"newOwner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"transferOwnership\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"collateral\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"withdraw\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]\n" ) . expect ( "invalid abi" )
    });
    #[derive(Clone)]
    pub struct Controller<P, S>(Contract<P, S>);
    impl<P, S> std::ops::Deref for Controller<P, S> {
        type Target = Contract<P, S>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<P: JsonRpcClient, S: Signer> std::fmt::Debug for Controller<P, S> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Controller))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, P: JsonRpcClient, S: Signer> Controller<P, S> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<Address>, C: Into<Arc<Client<P, S>>>>(address: T, client: C) -> Self {
            let contract = Contract::new(address.into(), CONTROLLER_ABI.clone(), client.into());
            Self(contract)
        }
        #[doc = "Calls the contract's `WETH` (0xad5c4648) function"]
        pub fn weth(&self) -> ContractCall<P, S, [u8; 32]> {
            self.0
                .method_hash([173, 92, 70, 72], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addSeries` (0x7c43a201) function"]
        pub fn add_series(&self, y_dai_contract: Address) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([124, 67, 162, 1], y_dai_contract)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `delegated` (0xf6bcbd31) function"]
        pub fn delegated(&self, p0: Address, p1: Address) -> ContractCall<P, S, bool> {
            self.0
                .method_hash([246, 188, 189, 49], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(&self) -> ContractCall<P, S, Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `repayDai` (0x25d3cd17) function"]
        pub fn repay_dai(
            &self,
            collateral: [u8; 32],
            maturity: U256,
            from: Address,
            to: Address,
            dai_amount: U256,
        ) -> ContractCall<P, S, ()> {
            self.0
                .method_hash(
                    [37, 211, 205, 23],
                    (collateral, maturity, from, to, dai_amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `posted` (0xdd62f423) function"]
        pub fn posted(&self, p0: [u8; 32], p1: Address) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([221, 98, 244, 35], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `THREE_MONTHS` (0xecbefab7) function"]
        pub fn three_months(&self) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([236, 190, 250, 183], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `revokeDelegate` (0xfa352c00) function"]
        pub fn revoke_delegate(&self, delegate: Address) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([250, 53, 44, 0], delegate)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `borrow` (0x801d325f) function"]
        pub fn borrow(
            &self,
            collateral: [u8; 32],
            maturity: U256,
            from: Address,
            to: Address,
            y_dai_amount: U256,
        ) -> ContractCall<P, S, ()> {
            self.0
                .method_hash(
                    [128, 29, 50, 95],
                    (collateral, maturity, from, to, y_dai_amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `post` (0xadb9e54e) function"]
        pub fn post(
            &self,
            collateral: [u8; 32],
            from: Address,
            to: Address,
            amount: U256,
        ) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([173, 185, 229, 78], (collateral, from, to, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdraw` (0x8e0cc176) function"]
        pub fn withdraw(
            &self,
            collateral: [u8; 32],
            from: Address,
            to: Address,
            amount: U256,
        ) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([142, 12, 193, 118], (collateral, from, to, amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `aboveDustOrZero` (0x4cd4423d) function"]
        pub fn above_dust_or_zero(
            &self,
            collateral: [u8; 32],
            user: Address,
        ) -> ContractCall<P, S, bool> {
            self.0
                .method_hash([76, 212, 66, 61], (collateral, user))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addDelegate` (0xe71bdf41) function"]
        pub fn add_delegate(&self, delegate: Address) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([231, 27, 223, 65], delegate)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `seriesIterator` (0xdd102225) function"]
        pub fn series_iterator(&self, p0: U256) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([221, 16, 34, 37], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(&self, new_owner: Address) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `containsSeries` (0xd0a3a7e4) function"]
        pub fn contains_series(&self, maturity: U256) -> ContractCall<P, S, bool> {
            self.0
                .method_hash([208, 163, 167, 228], maturity)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `inDai` (0x2f2ea5de) function"]
        pub fn in_dai(
            &self,
            collateral: [u8; 32],
            maturity: U256,
            y_dai_amount: U256,
        ) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([47, 46, 165, 222], (collateral, maturity, y_dai_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `UNIT` (0x9d8e2177) function"]
        pub fn unit(&self) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([157, 142, 33, 119], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `authorized` (0xb9181611) function"]
        pub fn authorized(&self, p0: Address) -> ContractCall<P, S, bool> {
            self.0
                .method_hash([185, 24, 22, 17], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `inYDai` (0xed7eebf2) function"]
        pub fn in_y_dai(
            &self,
            collateral: [u8; 32],
            maturity: U256,
            dai_amount: U256,
        ) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([237, 126, 235, 242], (collateral, maturity, dai_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalSeries` (0x0c6a595a) function"]
        pub fn total_series(&self) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([12, 106, 89, 90], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `CHAI` (0xb6dbf9ce) function"]
        pub fn chai(&self) -> ContractCall<P, S, [u8; 32]> {
            self.0
                .method_hash([182, 219, 249, 206], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isCollateralized` (0x7ad29f40) function"]
        pub fn is_collateralized(
            &self,
            collateral: [u8; 32],
            user: Address,
        ) -> ContractCall<P, S, bool> {
            self.0
                .method_hash([122, 210, 159, 64], (collateral, user))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `live` (0x957aa58c) function"]
        pub fn live(&self) -> ContractCall<P, S, bool> {
            self.0
                .method_hash([149, 122, 165, 140], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `DUST` (0x4e0cd799) function"]
        pub fn dust(&self) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([78, 12, 215, 153], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `repayYDai` (0x91371ffb) function"]
        pub fn repay_y_dai(
            &self,
            collateral: [u8; 32],
            maturity: U256,
            from: Address,
            to: Address,
            y_dai_amount: U256,
        ) -> ContractCall<P, S, ()> {
            self.0
                .method_hash(
                    [145, 55, 31, 251],
                    (collateral, maturity, from, to, y_dai_amount),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `series` (0xdc22cb6a) function"]
        pub fn series(&self, p0: U256) -> ContractCall<P, S, Address> {
            self.0
                .method_hash([220, 34, 203, 106], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `debtDai` (0xc5ec0c17) function"]
        pub fn debt_dai(
            &self,
            collateral: [u8; 32],
            maturity: U256,
            user: Address,
        ) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([197, 236, 12, 23], (collateral, maturity, user))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `orchestrate` (0x80f5a440) function"]
        pub fn orchestrate(&self, user: Address) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([128, 245, 164, 64], user)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `shutdown` (0xfc0e74d1) function"]
        pub fn shutdown(&self) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([252, 14, 116, 209], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalDebtDai` (0xe68b7cf4) function"]
        pub fn total_debt_dai(
            &self,
            collateral: [u8; 32],
            user: Address,
        ) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([230, 139, 124, 244], (collateral, user))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `powerOf` (0xc06331dc) function"]
        pub fn power_of(&self, collateral: [u8; 32], user: Address) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([192, 99, 49, 220], (collateral, user))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `debtYDai` (0x98cdae5a) function"]
        pub fn debt_y_dai(&self, p0: [u8; 32], p1: U256, p2: Address) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([152, 205, 174, 90], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `erase` (0x0d4c43be) function"]
        pub fn erase(
            &self,
            collateral: [u8; 32],
            user: Address,
        ) -> ContractCall<P, S, (U256, U256)> {
            self.0
                .method_hash([13, 76, 67, 190], (collateral, user))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ContractCall<P, S, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `locked` (0x354d9fe0) function"]
        pub fn locked(&self, collateral: [u8; 32], user: Address) -> ContractCall<P, S, U256> {
            self.0
                .method_hash([53, 77, 159, 224], (collateral, user))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Delegate` event"]
        pub fn delegate_filter(&self) -> Event<P, DelegateFilter> {
            self.0
                .event("Delegate")
                .expect("event not found (this should never happen)")
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
        #[doc = "Gets the contract's `Posted` event"]
        pub fn posted_filter(&self) -> Event<P, PostedFilter> {
            self.0
                .event("Posted")
                .expect("event not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Borrowed` event"]
        pub fn borrowed_filter(&self) -> Event<P, BorrowedFilter> {
            self.0
                .event("Borrowed")
                .expect("event not found (this should never happen)")
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
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct PostedFilter {
        pub collateral: [u8; 32],
        pub user: Address,
        pub amount: U256,
    }
    impl PostedFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                206, 25, 148, 114, 124, 241, 100, 227, 8, 1, 241, 64, 179, 1, 158, 13, 72, 98, 212,
                48, 67, 138, 180, 29, 225, 20, 75, 93, 183, 25, 197, 131,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`Posted(bytes32,address,int256)`"]
        pub const fn abi_signature() -> &'static str {
            "Posted(bytes32,address,int256)"
        }
    }
    impl Detokenize for PostedFilter {
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
            let collateral =
                Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let user = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let amount = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(PostedFilter {
                collateral,
                user,
                amount,
            })
        }
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct BorrowedFilter {
        pub collateral: [u8; 32],
        pub maturity: U256,
        pub user: Address,
        pub amount: U256,
    }
    impl BorrowedFilter {
        #[doc = r" Retrieves the signature for the event this data corresponds to."]
        #[doc = r" This signature is the Keccak-256 hash of the ABI signature of"]
        #[doc = r" this event."]
        pub const fn signature() -> H256 {
            H256([
                61, 192, 237, 40, 107, 26, 251, 194, 34, 138, 30, 127, 129, 252, 63, 102, 170, 159,
                66, 61, 202, 12, 106, 40, 62, 175, 165, 62, 147, 173, 239, 190,
            ])
        }
        #[doc = r" Retrieves the ABI signature for the event this data corresponds"]
        #[doc = r" to. For this event the value should always be:"]
        #[doc = r""]
        #[doc = "`Borrowed(bytes32,uint256,address,int256)`"]
        pub const fn abi_signature() -> &'static str {
            "Borrowed(bytes32,uint256,address,int256)"
        }
    }
    impl Detokenize for BorrowedFilter {
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
            let collateral =
                Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let maturity =
                Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let user = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            let amount = Tokenizable::from_token(tokens.next().expect("this should never happen"))?;
            Ok(BorrowedFilter {
                collateral,
                maturity,
                user,
                amount,
            })
        }
    }
}

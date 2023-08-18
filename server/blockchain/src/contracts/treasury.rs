pub use treasury::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod treasury {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_name"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_publicKey"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("depositPermit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("depositPermit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("v"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("eip712Domain"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("eip712Domain"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fields"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        1usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes1"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("verifyingContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("salt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("extensions"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("name"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nonces"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nonces"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawPermit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawPermit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("v"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("r"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("s"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Deposit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Deposit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EIP712DomainChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EIP712DomainChanged",
                            ),
                            inputs: ::std::vec![],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Withdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("nonce"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("deadline"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("InvalidShortString"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidShortString"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StringTooLong"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("StringTooLong"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("str"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static TREASURY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01``@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0*\xCF8\x03\x80b\0*\xCF\x839\x81\x81\x01`@R\x81\x01\x90b\0\08\x91\x90b\0\x05\x99V[\x81`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPb\0\0\x8Fb\0\0\x83b\0\x01\xA4` \x1B` \x1CV[b\0\x01\xAC` \x1B` \x1CV[b\0\0\xAA`\x01\x83b\0\x02p` \x1Bb\0\x08\xA5\x17\x90\x91\x90` \x1CV[a\x01 \x81\x81RPPb\0\0\xCD`\x02\x82b\0\x02p` \x1Bb\0\x08\xA5\x17\x90\x91\x90` \x1CV[a\x01@\x81\x81RPP\x81\x80Q\x90` \x01 `\xE0\x81\x81RPP\x80\x80Q\x90` \x01 a\x01\0\x81\x81RPPF`\xA0\x81\x81RPPb\0\x01\x0Cb\0\x02\xCD` \x1B` \x1CV[`\x80\x81\x81RPP0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xC0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPPP\x80`\x03`\0a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81`\x04\x90\x81b\0\x01\x9B\x91\x90b\0\x08JV[PPPb\0\n\xE3V[`\x003\x90P\x90V[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\0\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[`\0` \x83Q\x10\x15b\0\x02\x96Wb\0\x02\x8E\x83b\0\x03*` \x1B` \x1CV[\x90Pb\0\x02\xC7V[\x82b\0\x02\xAD\x83b\0\x03\x97` \x1Bb\0\x08\xE9\x17` \x1CV[`\0\x01\x90\x81b\0\x02\xBE\x91\x90b\0\x08JV[P`\xFF`\0\x1B\x90P[\x92\x91PPV[`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F`\xE0Qa\x01\0QF0`@Q` \x01b\0\x03\x0F\x95\x94\x93\x92\x91\x90b\0\tnV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\0\x80\x82\x90P`\x1F\x81Q\x11\x15b\0\x03zW\x82`@Q\x7F0Z'\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01b\0\x03q\x91\x90b\0\n\x1DV[`@Q\x80\x91\x03\x90\xFD[\x80Q\x81b\0\x03\x88\x90b\0\nsV[`\0\x1C\x17`\0\x1B\x91PP\x91\x90PV[`\0\x81\x90P\x91\x90PV[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[b\0\x04\n\x82b\0\x03\xBFV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15b\0\x04,Wb\0\x04+b\0\x03\xD0V[[\x80`@RPPPV[`\0b\0\x04Ab\0\x03\xA1V[\x90Pb\0\x04O\x82\x82b\0\x03\xFFV[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15b\0\x04rWb\0\x04qb\0\x03\xD0V[[b\0\x04}\x82b\0\x03\xBFV[\x90P` \x81\x01\x90P\x91\x90PV[`\0[\x83\x81\x10\x15b\0\x04\xAAW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pb\0\x04\x8DV[`\0\x84\x84\x01RPPPPV[`\0b\0\x04\xCDb\0\x04\xC7\x84b\0\x04TV[b\0\x045V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15b\0\x04\xECWb\0\x04\xEBb\0\x03\xBAV[[b\0\x04\xF9\x84\x82\x85b\0\x04\x8AV[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12b\0\x05\x19Wb\0\x05\x18b\0\x03\xB5V[[\x81Qb\0\x05+\x84\x82` \x86\x01b\0\x04\xB6V[\x91PP\x92\x91PPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0b\0\x05a\x82b\0\x054V[\x90P\x91\x90PV[b\0\x05s\x81b\0\x05TV[\x81\x14b\0\x05\x7FW`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x05\x93\x81b\0\x05hV[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15b\0\x05\xB3Wb\0\x05\xB2b\0\x03\xABV[[`\0\x83\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x05\xD4Wb\0\x05\xD3b\0\x03\xB0V[[b\0\x05\xE2\x85\x82\x86\x01b\0\x05\x01V[\x92PP` b\0\x05\xF5\x85\x82\x86\x01b\0\x05\x82V[\x91PP\x92P\x92\x90PV[`\0\x81Q\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[`\0`\x02\x82\x04\x90P`\x01\x82\x16\x80b\0\x06RW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x06hWb\0\x06gb\0\x06\nV[[P\x91\x90PV[`\0\x81\x90P\x81`\0R` `\0 \x90P\x91\x90PV[`\0` `\x1F\x83\x01\x04\x90P\x91\x90PV[`\0\x82\x82\x1B\x90P\x92\x91PPV[`\0`\x08\x83\x02b\0\x06\xD2\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82b\0\x06\x93V[b\0\x06\xDE\x86\x83b\0\x06\x93V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[`\0\x81\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[`\0b\0\x07+b\0\x07%b\0\x07\x1F\x84b\0\x06\xF6V[b\0\x07\0V[b\0\x06\xF6V[\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[b\0\x07G\x83b\0\x07\nV[b\0\x07_b\0\x07V\x82b\0\x072V[\x84\x84Tb\0\x06\xA0V[\x82UPPPPV[`\0\x90V[b\0\x07vb\0\x07gV[b\0\x07\x83\x81\x84\x84b\0\x07<V[PPPV[[\x81\x81\x10\x15b\0\x07\xABWb\0\x07\x9F`\0\x82b\0\x07lV[`\x01\x81\x01\x90Pb\0\x07\x89V[PPV[`\x1F\x82\x11\x15b\0\x07\xFAWb\0\x07\xC4\x81b\0\x06nV[b\0\x07\xCF\x84b\0\x06\x83V[\x81\x01` \x85\x10\x15b\0\x07\xDFW\x81\x90P[b\0\x07\xF7b\0\x07\xEE\x85b\0\x06\x83V[\x83\x01\x82b\0\x07\x88V[PP[PPPV[`\0\x82\x82\x1C\x90P\x92\x91PPV[`\0b\0\x08\x1F`\0\x19\x84`\x08\x02b\0\x07\xFFV[\x19\x80\x83\x16\x91PP\x92\x91PPV[`\0b\0\x08:\x83\x83b\0\x08\x0CV[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[b\0\x08U\x82b\0\x05\xFFV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x08qWb\0\x08pb\0\x03\xD0V[[b\0\x08}\x82Tb\0\x069V[b\0\x08\x8A\x82\x82\x85b\0\x07\xAFV[`\0` \x90P`\x1F\x83\x11`\x01\x81\x14b\0\x08\xC2W`\0\x84\x15b\0\x08\xADW\x82\x87\x01Q\x90P[b\0\x08\xB9\x85\x82b\0\x08,V[\x86UPb\0\t)V[`\x1F\x19\x84\x16b\0\x08\xD2\x86b\0\x06nV[`\0[\x82\x81\x10\x15b\0\x08\xFCW\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pb\0\x08\xD5V[\x86\x83\x10\x15b\0\t\x1CW\x84\x89\x01Qb\0\t\x18`\x1F\x89\x16\x82b\0\x08\x0CV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[`\0\x81\x90P\x91\x90PV[b\0\tF\x81b\0\t1V[\x82RPPV[b\0\tW\x81b\0\x06\xF6V[\x82RPPV[b\0\th\x81b\0\x05TV[\x82RPPV[`\0`\xA0\x82\x01\x90Pb\0\t\x85`\0\x83\x01\x88b\0\t;V[b\0\t\x94` \x83\x01\x87b\0\t;V[b\0\t\xA3`@\x83\x01\x86b\0\t;V[b\0\t\xB2``\x83\x01\x85b\0\tLV[b\0\t\xC1`\x80\x83\x01\x84b\0\t]V[\x96\x95PPPPPPV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0b\0\t\xE9\x82b\0\x05\xFFV[b\0\t\xF5\x81\x85b\0\t\xCBV[\x93Pb\0\n\x07\x81\x85` \x86\x01b\0\x04\x8AV[b\0\n\x12\x81b\0\x03\xBFV[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Rb\0\n9\x81\x84b\0\t\xDCV[\x90P\x92\x91PPV[`\0\x81Q\x90P\x91\x90PV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[`\0b\0\nj\x82Qb\0\t1V[\x80\x91PP\x91\x90PV[`\0b\0\n\x80\x82b\0\nAV[\x82b\0\n\x8C\x84b\0\nLV[\x90Pb\0\n\x99\x81b\0\n\\V[\x92P` \x82\x10\x15b\0\n\xDCWb\0\n\xD7\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83` \x03`\x08\x02b\0\x06\x93V[\x83\x16\x92P[PP\x91\x90PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa\x1F\x91b\0\x0B>`\09`\0a\x04\x94\x01R`\0a\x04`\x01R`\0a\x0C\xFB\x01R`\0a\x0C\xDA\x01R`\0a\x08\xF7\x01R`\0a\tM\x01R`\0a\tv\x01Ra\x1F\x91`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x93W`\x005`\xE0\x1C\x80c~\xCE\xBE\0\x11a\0fW\x80c~\xCE\xBE\0\x14a\0\xFAW\x80c\x84\xB0\x19n\x14a\x01*W\x80c\x8D\xA5\xCB[\x14a\x01NW\x80c\xE9\xDE\xB8\xEC\x14a\x01lW\x80c\xF2\xFD\xE3\x8B\x14a\x01\x88Wa\0\x93V[\x80c\x06\xFD\xDE\x03\x14a\0\x98W\x80c6D\xE5\x15\x14a\0\xB6W\x80ck\x9D\x15\x9D\x14a\0\xD4W\x80cqP\x18\xA6\x14a\0\xF0W[`\0\x80\xFD[a\0\xA0a\x01\xA4V[`@Qa\0\xAD\x91\x90a\x11FV[`@Q\x80\x91\x03\x90\xF3[a\0\xBEa\x026V[`@Qa\0\xCB\x91\x90a\x11\x81V[`@Q\x80\x91\x03\x90\xF3[a\0\xEE`\x04\x806\x03\x81\x01\x90a\0\xE9\x91\x90a\x12\x9AV[a\x02EV[\0[a\0\xF8a\x03\xE9V[\0[a\x01\x14`\x04\x806\x03\x81\x01\x90a\x01\x0F\x91\x90a\x13'V[a\x03\xFDV[`@Qa\x01!\x91\x90a\x13cV[`@Q\x80\x91\x03\x90\xF3[a\x012a\x04MV[`@Qa\x01E\x97\x96\x95\x94\x93\x92\x91\x90a\x14\x86V[`@Q\x80\x91\x03\x90\xF3[a\x01Va\x05OV[`@Qa\x01c\x91\x90a\x15\nV[`@Q\x80\x91\x03\x90\xF3[a\x01\x86`\x04\x806\x03\x81\x01\x90a\x01\x81\x91\x90a\x15%V[a\x05xV[\0[a\x01\xA2`\x04\x806\x03\x81\x01\x90a\x01\x9D\x91\x90a\x13'V[a\x08\"V[\0[```\x04\x80Ta\x01\xB3\x90a\x15\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x01\xDF\x90a\x15\xF6V[\x80\x15a\x02,W\x80`\x1F\x10a\x02\x01Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02,V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\x0FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0a\x02@a\x08\xF3V[\x90P\x90V[`\x003\x90P\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD5\x05\xAC\xCF\x820\x89\x89\x89\x89\x89`@Q\x88c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\x8F\x97\x96\x95\x94\x93\x92\x91\x90a\x166V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\xA9W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xBDW=`\0\x80>=`\0\xFD[PPPP\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c#\xB8r\xDD\x820\x89`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\xFE\x93\x92\x91\x90a\x16\xA5V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\x1DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03A\x91\x90a\x17\x14V[P\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x03`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F|\xFF\xF9\x08\xA4\xB5\x83\xF3d0\xB2]u\x96LE\x8D\x8E\xDE\x8A\x99\xBDa\xBEu\x0E\x97\xEE\x1B/:\x96\x89`@Qa\x03\xD8\x91\x90a\x13cV[`@Q\x80\x91\x03\x90\xA4PPPPPPPV[a\x03\xF1a\t\xAAV[a\x03\xFB`\0a\n(V[V[`\0a\x04F`\x05`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 a\n\xECV[\x90P\x91\x90PV[`\0``\x80`\0\x80`\0``a\x04\x8D`\x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\xFA\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x04\xC1`\x02\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\xFA\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[F0`\0\x80\x1B`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\xE2Wa\x04\xE1a\x17AV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\x10W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94\x93\x92\x91\x90\x96P\x96P\x96P\x96P\x96P\x96P\x96P\x90\x91\x92\x93\x94\x95\x96V[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[`\x003\x90P`\0a\x05\x88\x82a\x0B\xAAV[\x90P\x86B\x11\x15a\x05\xCDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05\xC4\x90a\x17\xBCV[`@Q\x80\x91\x03\x90\xFD[`\0\x7F\xBD\xB8\xDBW\xB2\xE3\x14G\xBF\xF6@\xFC\xC2\x87\xB0Z1\xC3\xDAE\xCA\x14\x0E\xAA[\xA8\xF3\xB0\x9A\xC9\x95\x06`\x03`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x8C\x8C\x86\x8D`@Q` \x01a\x06.\x97\x96\x95\x94\x93\x92\x91\x90a\x17\xDCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a\x06Q\x82a\x0C\x08V[\x90P`\0a\x06a\x82\x8A\x8A\x8Aa\x0C\"V[\x90P`\x03`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x06\xF3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06\xEA\x90a\x18\x97V[`@Q\x80\x91\x03\x90\xFD[\x8Bs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB\x87\x8D`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07.\x92\x91\x90a\x18\xB7V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07MW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07q\x91\x90a\x17\x14V[P\x8Bs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x03`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBB\xBD\xEEb({[\xF3\xBE\xE1<\xAB`\xA2\x9A\xD7)\xCF8\x10\x9B\xCC\xBD*\x98j\x11\xC9\x9B\x8C\xA7\x04\x8E\x88\x8F`@Qa\x08\x0C\x93\x92\x91\x90a\x18\xE0V[`@Q\x80\x91\x03\x90\xA4PPPPPPPPPPPPV[a\x08*a\t\xAAV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x08\x99W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\x90\x90a\x19\x89V[`@Q\x80\x91\x03\x90\xFD[a\x08\xA2\x81a\n(V[PV[`\0` \x83Q\x10\x15a\x08\xC1Wa\x08\xBA\x83a\x0CMV[\x90Pa\x08\xE3V[\x82a\x08\xCB\x83a\x08\xE9V[`\0\x01\x90\x81a\x08\xDA\x91\x90a\x1BUV[P`\xFF`\0\x1B\x90P[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80\x15a\toWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a\t\x9CW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\t\xA7V[a\t\xA4a\x0C\xB5V[\x90P[\x90V[a\t\xB2a\rKV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\t\xD0a\x05OV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\n&W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n\x1D\x90a\x1CsV[`@Q\x80\x91\x03\x90\xFD[V[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\0\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[`\0\x81`\0\x01T\x90P\x91\x90PV[```\xFF`\0\x1B\x83\x14a\x0B\x17Wa\x0B\x10\x83a\rSV[\x90Pa\x0B\xA4V[\x81\x80Ta\x0B#\x90a\x15\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0BO\x90a\x15\xF6V[\x80\x15a\x0B\x9CW\x80`\x1F\x10a\x0BqWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\x9CV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\x7FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P[\x92\x91PPV[`\0\x80`\x05`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x90Pa\x0B\xF7\x81a\n\xECV[\x91Pa\x0C\x02\x81a\r\xC7V[P\x91\x90PV[`\0a\x0C\x1Ba\x0C\x15a\x08\xF3V[\x83a\r\xDDV[\x90P\x91\x90PV[`\0\x80`\0a\x0C3\x87\x87\x87\x87a\x0E\x1EV[\x91P\x91Pa\x0C@\x81a\x0F\0V[\x81\x92PPP\x94\x93PPPPV[`\0\x80\x82\x90P`\x1F\x81Q\x11\x15a\x0C\x9AW\x82`@Q\x7F0Z'\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\x91\x91\x90a\x11FV[`@Q\x80\x91\x03\x90\xFD[\x80Q\x81a\x0C\xA6\x90a\x1C\xC3V[`\0\x1C\x17`\0\x1B\x91PP\x91\x90PV[`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F0`@Q` \x01a\r0\x95\x94\x93\x92\x91\x90a\x1D*V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\x003\x90P\x90V[```\0a\r`\x83a\x10fV[\x90P`\0` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x7FWa\r~a\x17AV[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\r\xB1W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x81\x81R\x83` \x82\x01R\x80\x92PPP\x91\x90PV[`\x01\x81`\0\x01`\0\x82\x82T\x01\x92PP\x81\x90UPPV[`\0`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x02\x82\x01R\x82`\"\x82\x01R`B\x81 \x91PP\x92\x91PPV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83`\0\x1C\x11\x15a\x0EYW`\0`\x03\x91P\x91Pa\x0E\xF7V[`\0`\x01\x87\x87\x87\x87`@Q`\0\x81R` \x01`@R`@Qa\x0E~\x94\x93\x92\x91\x90a\x1D}V[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x0E\xA0W=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q\x90P`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0E\xEEW`\0`\x01\x92P\x92PPa\x0E\xF7V[\x80`\0\x92P\x92PP[\x94P\x94\x92PPPV[`\0`\x04\x81\x11\x15a\x0F\x14Wa\x0F\x13a\x1D\xC2V[[\x81`\x04\x81\x11\x15a\x0F'Wa\x0F&a\x1D\xC2V[[\x03\x15a\x10cW`\x01`\x04\x81\x11\x15a\x0FAWa\x0F@a\x1D\xC2V[[\x81`\x04\x81\x11\x15a\x0FTWa\x0FSa\x1D\xC2V[[\x03a\x0F\x94W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F\x8B\x90a\x1E=V[`@Q\x80\x91\x03\x90\xFD[`\x02`\x04\x81\x11\x15a\x0F\xA8Wa\x0F\xA7a\x1D\xC2V[[\x81`\x04\x81\x11\x15a\x0F\xBBWa\x0F\xBAa\x1D\xC2V[[\x03a\x0F\xFBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F\xF2\x90a\x1E\xA9V[`@Q\x80\x91\x03\x90\xFD[`\x03`\x04\x81\x11\x15a\x10\x0FWa\x10\x0Ea\x1D\xC2V[[\x81`\x04\x81\x11\x15a\x10\"Wa\x10!a\x1D\xC2V[[\x03a\x10bW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10Y\x90a\x1F;V[`@Q\x80\x91\x03\x90\xFD[[PV[`\0\x80`\xFF\x83`\0\x1C\x16\x90P`\x1F\x81\x11\x15a\x10\xADW`@Q\x7F\xB3Q+\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x91PP\x91\x90PV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x10\xF0W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x10\xD5V[`\0\x84\x84\x01RPPPPV[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[`\0a\x11\x18\x82a\x10\xB6V[a\x11\"\x81\x85a\x10\xC1V[\x93Pa\x112\x81\x85` \x86\x01a\x10\xD2V[a\x11;\x81a\x10\xFCV[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x11`\x81\x84a\x11\rV[\x90P\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x11{\x81a\x11hV[\x82RPPV[`\0` \x82\x01\x90Pa\x11\x96`\0\x83\x01\x84a\x11rV[\x92\x91PPV[`\0\x80\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x11\xCC\x82a\x11\xA1V[\x90P\x91\x90PV[a\x11\xDC\x81a\x11\xC1V[\x81\x14a\x11\xE7W`\0\x80\xFD[PV[`\0\x815\x90Pa\x11\xF9\x81a\x11\xD3V[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x12\x12\x81a\x11\xFFV[\x81\x14a\x12\x1DW`\0\x80\xFD[PV[`\0\x815\x90Pa\x12/\x81a\x12\tV[\x92\x91PPV[`\0`\xFF\x82\x16\x90P\x91\x90PV[a\x12K\x81a\x125V[\x81\x14a\x12VW`\0\x80\xFD[PV[`\0\x815\x90Pa\x12h\x81a\x12BV[\x92\x91PPV[a\x12w\x81a\x11hV[\x81\x14a\x12\x82W`\0\x80\xFD[PV[`\0\x815\x90Pa\x12\x94\x81a\x12nV[\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x12\xB7Wa\x12\xB6a\x11\x9CV[[`\0a\x12\xC5\x89\x82\x8A\x01a\x11\xEAV[\x96PP` a\x12\xD6\x89\x82\x8A\x01a\x12 V[\x95PP`@a\x12\xE7\x89\x82\x8A\x01a\x12 V[\x94PP``a\x12\xF8\x89\x82\x8A\x01a\x12YV[\x93PP`\x80a\x13\t\x89\x82\x8A\x01a\x12\x85V[\x92PP`\xA0a\x13\x1A\x89\x82\x8A\x01a\x12\x85V[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0` \x82\x84\x03\x12\x15a\x13=Wa\x13<a\x11\x9CV[[`\0a\x13K\x84\x82\x85\x01a\x11\xEAV[\x91PP\x92\x91PPV[a\x13]\x81a\x11\xFFV[\x82RPPV[`\0` \x82\x01\x90Pa\x13x`\0\x83\x01\x84a\x13TV[\x92\x91PPV[`\0\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a\x13\xB3\x81a\x13~V[\x82RPPV[a\x13\xC2\x81a\x11\xC1V[\x82RPPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x13\xFD\x81a\x11\xFFV[\x82RPPV[`\0a\x14\x0F\x83\x83a\x13\xF4V[` \x83\x01\x90P\x92\x91PPV[`\0` \x82\x01\x90P\x91\x90PV[`\0a\x143\x82a\x13\xC8V[a\x14=\x81\x85a\x13\xD3V[\x93Pa\x14H\x83a\x13\xE4V[\x80`\0[\x83\x81\x10\x15a\x14yW\x81Qa\x14`\x88\x82a\x14\x03V[\x97Pa\x14k\x83a\x14\x1BV[\x92PP`\x01\x81\x01\x90Pa\x14LV[P\x85\x93PPPP\x92\x91PPV[`\0`\xE0\x82\x01\x90Pa\x14\x9B`\0\x83\x01\x8Aa\x13\xAAV[\x81\x81\x03` \x83\x01Ra\x14\xAD\x81\x89a\x11\rV[\x90P\x81\x81\x03`@\x83\x01Ra\x14\xC1\x81\x88a\x11\rV[\x90Pa\x14\xD0``\x83\x01\x87a\x13TV[a\x14\xDD`\x80\x83\x01\x86a\x13\xB9V[a\x14\xEA`\xA0\x83\x01\x85a\x11rV[\x81\x81\x03`\xC0\x83\x01Ra\x14\xFC\x81\x84a\x14(V[\x90P\x98\x97PPPPPPPPV[`\0` \x82\x01\x90Pa\x15\x1F`\0\x83\x01\x84a\x13\xB9V[\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x15DWa\x15Ca\x11\x9CV[[`\0a\x15R\x8A\x82\x8B\x01a\x11\xEAV[\x97PP` a\x15c\x8A\x82\x8B\x01a\x12 V[\x96PP`@a\x15t\x8A\x82\x8B\x01a\x12 V[\x95PP``a\x15\x85\x8A\x82\x8B\x01a\x12YV[\x94PP`\x80a\x15\x96\x8A\x82\x8B\x01a\x12\x85V[\x93PP`\xA0a\x15\xA7\x8A\x82\x8B\x01a\x12\x85V[\x92PP`\xC0a\x15\xB8\x8A\x82\x8B\x01a\x11\xEAV[\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[`\0`\x02\x82\x04\x90P`\x01\x82\x16\x80a\x16\x0EW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x16!Wa\x16 a\x15\xC7V[[P\x91\x90PV[a\x160\x81a\x125V[\x82RPPV[`\0`\xE0\x82\x01\x90Pa\x16K`\0\x83\x01\x8Aa\x13\xB9V[a\x16X` \x83\x01\x89a\x13\xB9V[a\x16e`@\x83\x01\x88a\x13TV[a\x16r``\x83\x01\x87a\x13TV[a\x16\x7F`\x80\x83\x01\x86a\x16'V[a\x16\x8C`\xA0\x83\x01\x85a\x11rV[a\x16\x99`\xC0\x83\x01\x84a\x11rV[\x98\x97PPPPPPPPV[`\0``\x82\x01\x90Pa\x16\xBA`\0\x83\x01\x86a\x13\xB9V[a\x16\xC7` \x83\x01\x85a\x13\xB9V[a\x16\xD4`@\x83\x01\x84a\x13TV[\x94\x93PPPPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\x16\xF1\x81a\x16\xDCV[\x81\x14a\x16\xFCW`\0\x80\xFD[PV[`\0\x81Q\x90Pa\x17\x0E\x81a\x16\xE8V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x17*Wa\x17)a\x11\x9CV[[`\0a\x178\x84\x82\x85\x01a\x16\xFFV[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[\x7FERC20Permit: expired deadline\0\0\0`\0\x82\x01RPV[`\0a\x17\xA6`\x1D\x83a\x10\xC1V[\x91Pa\x17\xB1\x82a\x17pV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x17\xD5\x81a\x17\x99V[\x90P\x91\x90PV[`\0`\xE0\x82\x01\x90Pa\x17\xF1`\0\x83\x01\x8Aa\x11rV[a\x17\xFE` \x83\x01\x89a\x13\xB9V[a\x18\x0B`@\x83\x01\x88a\x13\xB9V[a\x18\x18``\x83\x01\x87a\x13\xB9V[a\x18%`\x80\x83\x01\x86a\x13TV[a\x182`\xA0\x83\x01\x85a\x13TV[a\x18?`\xC0\x83\x01\x84a\x13TV[\x98\x97PPPPPPPPV[\x7FERC20Permit: invalid signature\0\0`\0\x82\x01RPV[`\0a\x18\x81`\x1E\x83a\x10\xC1V[\x91Pa\x18\x8C\x82a\x18KV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x18\xB0\x81a\x18tV[\x90P\x91\x90PV[`\0`@\x82\x01\x90Pa\x18\xCC`\0\x83\x01\x85a\x13\xB9V[a\x18\xD9` \x83\x01\x84a\x13TV[\x93\x92PPPV[`\0``\x82\x01\x90Pa\x18\xF5`\0\x83\x01\x86a\x13TV[a\x19\x02` \x83\x01\x85a\x13TV[a\x19\x0F`@\x83\x01\x84a\x13TV[\x94\x93PPPPV[\x7FOwnable: new owner is the zero a`\0\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\x19s`&\x83a\x10\xC1V[\x91Pa\x19~\x82a\x19\x17V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x19\xA2\x81a\x19fV[\x90P\x91\x90PV[`\0\x81\x90P\x81`\0R` `\0 \x90P\x91\x90PV[`\0` `\x1F\x83\x01\x04\x90P\x91\x90PV[`\0\x82\x82\x1B\x90P\x92\x91PPV[`\0`\x08\x83\x02a\x1A\x0B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a\x19\xCEV[a\x1A\x15\x86\x83a\x19\xCEV[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[`\0\x81\x90P\x91\x90PV[`\0a\x1ARa\x1AMa\x1AH\x84a\x11\xFFV[a\x1A-V[a\x11\xFFV[\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a\x1Al\x83a\x1A7V[a\x1A\x80a\x1Ax\x82a\x1AYV[\x84\x84Ta\x19\xDBV[\x82UPPPPV[`\0\x90V[a\x1A\x95a\x1A\x88V[a\x1A\xA0\x81\x84\x84a\x1AcV[PPPV[[\x81\x81\x10\x15a\x1A\xC4Wa\x1A\xB9`\0\x82a\x1A\x8DV[`\x01\x81\x01\x90Pa\x1A\xA6V[PPV[`\x1F\x82\x11\x15a\x1B\tWa\x1A\xDA\x81a\x19\xA9V[a\x1A\xE3\x84a\x19\xBEV[\x81\x01` \x85\x10\x15a\x1A\xF2W\x81\x90P[a\x1B\x06a\x1A\xFE\x85a\x19\xBEV[\x83\x01\x82a\x1A\xA5V[PP[PPPV[`\0\x82\x82\x1C\x90P\x92\x91PPV[`\0a\x1B,`\0\x19\x84`\x08\x02a\x1B\x0EV[\x19\x80\x83\x16\x91PP\x92\x91PPV[`\0a\x1BE\x83\x83a\x1B\x1BV[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a\x1B^\x82a\x10\xB6V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1BwWa\x1Bva\x17AV[[a\x1B\x81\x82Ta\x15\xF6V[a\x1B\x8C\x82\x82\x85a\x1A\xC8V[`\0` \x90P`\x1F\x83\x11`\x01\x81\x14a\x1B\xBFW`\0\x84\x15a\x1B\xADW\x82\x87\x01Q\x90P[a\x1B\xB7\x85\x82a\x1B9V[\x86UPa\x1C\x1FV[`\x1F\x19\x84\x16a\x1B\xCD\x86a\x19\xA9V[`\0[\x82\x81\x10\x15a\x1B\xF5W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa\x1B\xD0V[\x86\x83\x10\x15a\x1C\x12W\x84\x89\x01Qa\x1C\x0E`\x1F\x89\x16\x82a\x1B\x1BV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[\x7FOwnable: caller is not the owner`\0\x82\x01RPV[`\0a\x1C]` \x83a\x10\xC1V[\x91Pa\x1Ch\x82a\x1C'V[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1C\x8C\x81a\x1CPV[\x90P\x91\x90PV[`\0\x81Q\x90P\x91\x90PV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[`\0a\x1C\xBA\x82Qa\x11hV[\x80\x91PP\x91\x90PV[`\0a\x1C\xCE\x82a\x1C\x93V[\x82a\x1C\xD8\x84a\x1C\x9EV[\x90Pa\x1C\xE3\x81a\x1C\xAEV[\x92P` \x82\x10\x15a\x1D#Wa\x1D\x1E\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83` \x03`\x08\x02a\x19\xCEV[\x83\x16\x92P[PP\x91\x90PV[`\0`\xA0\x82\x01\x90Pa\x1D?`\0\x83\x01\x88a\x11rV[a\x1DL` \x83\x01\x87a\x11rV[a\x1DY`@\x83\x01\x86a\x11rV[a\x1Df``\x83\x01\x85a\x13TV[a\x1Ds`\x80\x83\x01\x84a\x13\xB9V[\x96\x95PPPPPPV[`\0`\x80\x82\x01\x90Pa\x1D\x92`\0\x83\x01\x87a\x11rV[a\x1D\x9F` \x83\x01\x86a\x16'V[a\x1D\xAC`@\x83\x01\x85a\x11rV[a\x1D\xB9``\x83\x01\x84a\x11rV[\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x1E'`\x18\x83a\x10\xC1V[\x91Pa\x1E2\x82a\x1D\xF1V[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1EV\x81a\x1E\x1AV[\x90P\x91\x90PV[\x7FECDSA: invalid signature length\0`\0\x82\x01RPV[`\0a\x1E\x93`\x1F\x83a\x10\xC1V[\x91Pa\x1E\x9E\x82a\x1E]V[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1E\xC2\x81a\x1E\x86V[\x90P\x91\x90PV[\x7FECDSA: invalid signature 's' val`\0\x82\x01R\x7Fue\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\x1F%`\"\x83a\x10\xC1V[\x91Pa\x1F0\x82a\x1E\xC9V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1FT\x81a\x1F\x18V[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 \xA4\x98~d\x84\xAB\xDC\xFE\x8D\xA0\t\xA3G\"\x14\xA7\xE1\x99\xFAIs\xA7_\xE5\r\x9Dj.\xD7\x80v\x81dsolcC\0\x08\x12\x003";
    /// The bytecode of the contract.
    pub static TREASURY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x93W`\x005`\xE0\x1C\x80c~\xCE\xBE\0\x11a\0fW\x80c~\xCE\xBE\0\x14a\0\xFAW\x80c\x84\xB0\x19n\x14a\x01*W\x80c\x8D\xA5\xCB[\x14a\x01NW\x80c\xE9\xDE\xB8\xEC\x14a\x01lW\x80c\xF2\xFD\xE3\x8B\x14a\x01\x88Wa\0\x93V[\x80c\x06\xFD\xDE\x03\x14a\0\x98W\x80c6D\xE5\x15\x14a\0\xB6W\x80ck\x9D\x15\x9D\x14a\0\xD4W\x80cqP\x18\xA6\x14a\0\xF0W[`\0\x80\xFD[a\0\xA0a\x01\xA4V[`@Qa\0\xAD\x91\x90a\x11FV[`@Q\x80\x91\x03\x90\xF3[a\0\xBEa\x026V[`@Qa\0\xCB\x91\x90a\x11\x81V[`@Q\x80\x91\x03\x90\xF3[a\0\xEE`\x04\x806\x03\x81\x01\x90a\0\xE9\x91\x90a\x12\x9AV[a\x02EV[\0[a\0\xF8a\x03\xE9V[\0[a\x01\x14`\x04\x806\x03\x81\x01\x90a\x01\x0F\x91\x90a\x13'V[a\x03\xFDV[`@Qa\x01!\x91\x90a\x13cV[`@Q\x80\x91\x03\x90\xF3[a\x012a\x04MV[`@Qa\x01E\x97\x96\x95\x94\x93\x92\x91\x90a\x14\x86V[`@Q\x80\x91\x03\x90\xF3[a\x01Va\x05OV[`@Qa\x01c\x91\x90a\x15\nV[`@Q\x80\x91\x03\x90\xF3[a\x01\x86`\x04\x806\x03\x81\x01\x90a\x01\x81\x91\x90a\x15%V[a\x05xV[\0[a\x01\xA2`\x04\x806\x03\x81\x01\x90a\x01\x9D\x91\x90a\x13'V[a\x08\"V[\0[```\x04\x80Ta\x01\xB3\x90a\x15\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x01\xDF\x90a\x15\xF6V[\x80\x15a\x02,W\x80`\x1F\x10a\x02\x01Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02,V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\x0FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[`\0a\x02@a\x08\xF3V[\x90P\x90V[`\x003\x90P\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD5\x05\xAC\xCF\x820\x89\x89\x89\x89\x89`@Q\x88c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\x8F\x97\x96\x95\x94\x93\x92\x91\x90a\x166V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\xA9W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\xBDW=`\0\x80>=`\0\xFD[PPPP\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c#\xB8r\xDD\x820\x89`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\xFE\x93\x92\x91\x90a\x16\xA5V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03\x1DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03A\x91\x90a\x17\x14V[P\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x03`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F|\xFF\xF9\x08\xA4\xB5\x83\xF3d0\xB2]u\x96LE\x8D\x8E\xDE\x8A\x99\xBDa\xBEu\x0E\x97\xEE\x1B/:\x96\x89`@Qa\x03\xD8\x91\x90a\x13cV[`@Q\x80\x91\x03\x90\xA4PPPPPPPV[a\x03\xF1a\t\xAAV[a\x03\xFB`\0a\n(V[V[`\0a\x04F`\x05`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 a\n\xECV[\x90P\x91\x90PV[`\0``\x80`\0\x80`\0``a\x04\x8D`\x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\xFA\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x04\xC1`\x02\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\xFA\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[F0`\0\x80\x1B`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\xE2Wa\x04\xE1a\x17AV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\x10W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94\x93\x92\x91\x90\x96P\x96P\x96P\x96P\x96P\x96P\x96P\x90\x91\x92\x93\x94\x95\x96V[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[`\x003\x90P`\0a\x05\x88\x82a\x0B\xAAV[\x90P\x86B\x11\x15a\x05\xCDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x05\xC4\x90a\x17\xBCV[`@Q\x80\x91\x03\x90\xFD[`\0\x7F\xBD\xB8\xDBW\xB2\xE3\x14G\xBF\xF6@\xFC\xC2\x87\xB0Z1\xC3\xDAE\xCA\x14\x0E\xAA[\xA8\xF3\xB0\x9A\xC9\x95\x06`\x03`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x8C\x8C\x86\x8D`@Q` \x01a\x06.\x97\x96\x95\x94\x93\x92\x91\x90a\x17\xDCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a\x06Q\x82a\x0C\x08V[\x90P`\0a\x06a\x82\x8A\x8A\x8Aa\x0C\"V[\x90P`\x03`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x06\xF3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06\xEA\x90a\x18\x97V[`@Q\x80\x91\x03\x90\xFD[\x8Bs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA9\x05\x9C\xBB\x87\x8D`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07.\x92\x91\x90a\x18\xB7V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07MW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07q\x91\x90a\x17\x14V[P\x8Bs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x03`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBB\xBD\xEEb({[\xF3\xBE\xE1<\xAB`\xA2\x9A\xD7)\xCF8\x10\x9B\xCC\xBD*\x98j\x11\xC9\x9B\x8C\xA7\x04\x8E\x88\x8F`@Qa\x08\x0C\x93\x92\x91\x90a\x18\xE0V[`@Q\x80\x91\x03\x90\xA4PPPPPPPPPPPPV[a\x08*a\t\xAAV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x08\x99W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x08\x90\x90a\x19\x89V[`@Q\x80\x91\x03\x90\xFD[a\x08\xA2\x81a\n(V[PV[`\0` \x83Q\x10\x15a\x08\xC1Wa\x08\xBA\x83a\x0CMV[\x90Pa\x08\xE3V[\x82a\x08\xCB\x83a\x08\xE9V[`\0\x01\x90\x81a\x08\xDA\x91\x90a\x1BUV[P`\xFF`\0\x1B\x90P[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80\x15a\toWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a\t\x9CW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90Pa\t\xA7V[a\t\xA4a\x0C\xB5V[\x90P[\x90V[a\t\xB2a\rKV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\t\xD0a\x05OV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\n&W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n\x1D\x90a\x1CsV[`@Q\x80\x91\x03\x90\xFD[V[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81`\0\x80a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPV[`\0\x81`\0\x01T\x90P\x91\x90PV[```\xFF`\0\x1B\x83\x14a\x0B\x17Wa\x0B\x10\x83a\rSV[\x90Pa\x0B\xA4V[\x81\x80Ta\x0B#\x90a\x15\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0BO\x90a\x15\xF6V[\x80\x15a\x0B\x9CW\x80`\x1F\x10a\x0BqWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\x9CV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\x7FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P[\x92\x91PPV[`\0\x80`\x05`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x90Pa\x0B\xF7\x81a\n\xECV[\x91Pa\x0C\x02\x81a\r\xC7V[P\x91\x90PV[`\0a\x0C\x1Ba\x0C\x15a\x08\xF3V[\x83a\r\xDDV[\x90P\x91\x90PV[`\0\x80`\0a\x0C3\x87\x87\x87\x87a\x0E\x1EV[\x91P\x91Pa\x0C@\x81a\x0F\0V[\x81\x92PPP\x94\x93PPPPV[`\0\x80\x82\x90P`\x1F\x81Q\x11\x15a\x0C\x9AW\x82`@Q\x7F0Z'\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C\x91\x91\x90a\x11FV[`@Q\x80\x91\x03\x90\xFD[\x80Q\x81a\x0C\xA6\x90a\x1C\xC3V[`\0\x1C\x17`\0\x1B\x91PP\x91\x90PV[`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F0`@Q` \x01a\r0\x95\x94\x93\x92\x91\x90a\x1D*V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\x003\x90P\x90V[```\0a\r`\x83a\x10fV[\x90P`\0` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x7FWa\r~a\x17AV[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\r\xB1W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x81\x81R\x83` \x82\x01R\x80\x92PPP\x91\x90PV[`\x01\x81`\0\x01`\0\x82\x82T\x01\x92PP\x81\x90UPPV[`\0`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x02\x82\x01R\x82`\"\x82\x01R`B\x81 \x91PP\x92\x91PPV[`\0\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83`\0\x1C\x11\x15a\x0EYW`\0`\x03\x91P\x91Pa\x0E\xF7V[`\0`\x01\x87\x87\x87\x87`@Q`\0\x81R` \x01`@R`@Qa\x0E~\x94\x93\x92\x91\x90a\x1D}V[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x0E\xA0W=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q\x90P`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0E\xEEW`\0`\x01\x92P\x92PPa\x0E\xF7V[\x80`\0\x92P\x92PP[\x94P\x94\x92PPPV[`\0`\x04\x81\x11\x15a\x0F\x14Wa\x0F\x13a\x1D\xC2V[[\x81`\x04\x81\x11\x15a\x0F'Wa\x0F&a\x1D\xC2V[[\x03\x15a\x10cW`\x01`\x04\x81\x11\x15a\x0FAWa\x0F@a\x1D\xC2V[[\x81`\x04\x81\x11\x15a\x0FTWa\x0FSa\x1D\xC2V[[\x03a\x0F\x94W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F\x8B\x90a\x1E=V[`@Q\x80\x91\x03\x90\xFD[`\x02`\x04\x81\x11\x15a\x0F\xA8Wa\x0F\xA7a\x1D\xC2V[[\x81`\x04\x81\x11\x15a\x0F\xBBWa\x0F\xBAa\x1D\xC2V[[\x03a\x0F\xFBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F\xF2\x90a\x1E\xA9V[`@Q\x80\x91\x03\x90\xFD[`\x03`\x04\x81\x11\x15a\x10\x0FWa\x10\x0Ea\x1D\xC2V[[\x81`\x04\x81\x11\x15a\x10\"Wa\x10!a\x1D\xC2V[[\x03a\x10bW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10Y\x90a\x1F;V[`@Q\x80\x91\x03\x90\xFD[[PV[`\0\x80`\xFF\x83`\0\x1C\x16\x90P`\x1F\x81\x11\x15a\x10\xADW`@Q\x7F\xB3Q+\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x91PP\x91\x90PV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x10\xF0W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x10\xD5V[`\0\x84\x84\x01RPPPPV[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[`\0a\x11\x18\x82a\x10\xB6V[a\x11\"\x81\x85a\x10\xC1V[\x93Pa\x112\x81\x85` \x86\x01a\x10\xD2V[a\x11;\x81a\x10\xFCV[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x11`\x81\x84a\x11\rV[\x90P\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x11{\x81a\x11hV[\x82RPPV[`\0` \x82\x01\x90Pa\x11\x96`\0\x83\x01\x84a\x11rV[\x92\x91PPV[`\0\x80\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x11\xCC\x82a\x11\xA1V[\x90P\x91\x90PV[a\x11\xDC\x81a\x11\xC1V[\x81\x14a\x11\xE7W`\0\x80\xFD[PV[`\0\x815\x90Pa\x11\xF9\x81a\x11\xD3V[\x92\x91PPV[`\0\x81\x90P\x91\x90PV[a\x12\x12\x81a\x11\xFFV[\x81\x14a\x12\x1DW`\0\x80\xFD[PV[`\0\x815\x90Pa\x12/\x81a\x12\tV[\x92\x91PPV[`\0`\xFF\x82\x16\x90P\x91\x90PV[a\x12K\x81a\x125V[\x81\x14a\x12VW`\0\x80\xFD[PV[`\0\x815\x90Pa\x12h\x81a\x12BV[\x92\x91PPV[a\x12w\x81a\x11hV[\x81\x14a\x12\x82W`\0\x80\xFD[PV[`\0\x815\x90Pa\x12\x94\x81a\x12nV[\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x12\xB7Wa\x12\xB6a\x11\x9CV[[`\0a\x12\xC5\x89\x82\x8A\x01a\x11\xEAV[\x96PP` a\x12\xD6\x89\x82\x8A\x01a\x12 V[\x95PP`@a\x12\xE7\x89\x82\x8A\x01a\x12 V[\x94PP``a\x12\xF8\x89\x82\x8A\x01a\x12YV[\x93PP`\x80a\x13\t\x89\x82\x8A\x01a\x12\x85V[\x92PP`\xA0a\x13\x1A\x89\x82\x8A\x01a\x12\x85V[\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0` \x82\x84\x03\x12\x15a\x13=Wa\x13<a\x11\x9CV[[`\0a\x13K\x84\x82\x85\x01a\x11\xEAV[\x91PP\x92\x91PPV[a\x13]\x81a\x11\xFFV[\x82RPPV[`\0` \x82\x01\x90Pa\x13x`\0\x83\x01\x84a\x13TV[\x92\x91PPV[`\0\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a\x13\xB3\x81a\x13~V[\x82RPPV[a\x13\xC2\x81a\x11\xC1V[\x82RPPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x13\xFD\x81a\x11\xFFV[\x82RPPV[`\0a\x14\x0F\x83\x83a\x13\xF4V[` \x83\x01\x90P\x92\x91PPV[`\0` \x82\x01\x90P\x91\x90PV[`\0a\x143\x82a\x13\xC8V[a\x14=\x81\x85a\x13\xD3V[\x93Pa\x14H\x83a\x13\xE4V[\x80`\0[\x83\x81\x10\x15a\x14yW\x81Qa\x14`\x88\x82a\x14\x03V[\x97Pa\x14k\x83a\x14\x1BV[\x92PP`\x01\x81\x01\x90Pa\x14LV[P\x85\x93PPPP\x92\x91PPV[`\0`\xE0\x82\x01\x90Pa\x14\x9B`\0\x83\x01\x8Aa\x13\xAAV[\x81\x81\x03` \x83\x01Ra\x14\xAD\x81\x89a\x11\rV[\x90P\x81\x81\x03`@\x83\x01Ra\x14\xC1\x81\x88a\x11\rV[\x90Pa\x14\xD0``\x83\x01\x87a\x13TV[a\x14\xDD`\x80\x83\x01\x86a\x13\xB9V[a\x14\xEA`\xA0\x83\x01\x85a\x11rV[\x81\x81\x03`\xC0\x83\x01Ra\x14\xFC\x81\x84a\x14(V[\x90P\x98\x97PPPPPPPPV[`\0` \x82\x01\x90Pa\x15\x1F`\0\x83\x01\x84a\x13\xB9V[\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x15DWa\x15Ca\x11\x9CV[[`\0a\x15R\x8A\x82\x8B\x01a\x11\xEAV[\x97PP` a\x15c\x8A\x82\x8B\x01a\x12 V[\x96PP`@a\x15t\x8A\x82\x8B\x01a\x12 V[\x95PP``a\x15\x85\x8A\x82\x8B\x01a\x12YV[\x94PP`\x80a\x15\x96\x8A\x82\x8B\x01a\x12\x85V[\x93PP`\xA0a\x15\xA7\x8A\x82\x8B\x01a\x12\x85V[\x92PP`\xC0a\x15\xB8\x8A\x82\x8B\x01a\x11\xEAV[\x91PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[`\0`\x02\x82\x04\x90P`\x01\x82\x16\x80a\x16\x0EW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x16!Wa\x16 a\x15\xC7V[[P\x91\x90PV[a\x160\x81a\x125V[\x82RPPV[`\0`\xE0\x82\x01\x90Pa\x16K`\0\x83\x01\x8Aa\x13\xB9V[a\x16X` \x83\x01\x89a\x13\xB9V[a\x16e`@\x83\x01\x88a\x13TV[a\x16r``\x83\x01\x87a\x13TV[a\x16\x7F`\x80\x83\x01\x86a\x16'V[a\x16\x8C`\xA0\x83\x01\x85a\x11rV[a\x16\x99`\xC0\x83\x01\x84a\x11rV[\x98\x97PPPPPPPPV[`\0``\x82\x01\x90Pa\x16\xBA`\0\x83\x01\x86a\x13\xB9V[a\x16\xC7` \x83\x01\x85a\x13\xB9V[a\x16\xD4`@\x83\x01\x84a\x13TV[\x94\x93PPPPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\x16\xF1\x81a\x16\xDCV[\x81\x14a\x16\xFCW`\0\x80\xFD[PV[`\0\x81Q\x90Pa\x17\x0E\x81a\x16\xE8V[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x17*Wa\x17)a\x11\x9CV[[`\0a\x178\x84\x82\x85\x01a\x16\xFFV[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[\x7FERC20Permit: expired deadline\0\0\0`\0\x82\x01RPV[`\0a\x17\xA6`\x1D\x83a\x10\xC1V[\x91Pa\x17\xB1\x82a\x17pV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x17\xD5\x81a\x17\x99V[\x90P\x91\x90PV[`\0`\xE0\x82\x01\x90Pa\x17\xF1`\0\x83\x01\x8Aa\x11rV[a\x17\xFE` \x83\x01\x89a\x13\xB9V[a\x18\x0B`@\x83\x01\x88a\x13\xB9V[a\x18\x18``\x83\x01\x87a\x13\xB9V[a\x18%`\x80\x83\x01\x86a\x13TV[a\x182`\xA0\x83\x01\x85a\x13TV[a\x18?`\xC0\x83\x01\x84a\x13TV[\x98\x97PPPPPPPPV[\x7FERC20Permit: invalid signature\0\0`\0\x82\x01RPV[`\0a\x18\x81`\x1E\x83a\x10\xC1V[\x91Pa\x18\x8C\x82a\x18KV[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x18\xB0\x81a\x18tV[\x90P\x91\x90PV[`\0`@\x82\x01\x90Pa\x18\xCC`\0\x83\x01\x85a\x13\xB9V[a\x18\xD9` \x83\x01\x84a\x13TV[\x93\x92PPPV[`\0``\x82\x01\x90Pa\x18\xF5`\0\x83\x01\x86a\x13TV[a\x19\x02` \x83\x01\x85a\x13TV[a\x19\x0F`@\x83\x01\x84a\x13TV[\x94\x93PPPPV[\x7FOwnable: new owner is the zero a`\0\x82\x01R\x7Fddress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\x19s`&\x83a\x10\xC1V[\x91Pa\x19~\x82a\x19\x17V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x19\xA2\x81a\x19fV[\x90P\x91\x90PV[`\0\x81\x90P\x81`\0R` `\0 \x90P\x91\x90PV[`\0` `\x1F\x83\x01\x04\x90P\x91\x90PV[`\0\x82\x82\x1B\x90P\x92\x91PPV[`\0`\x08\x83\x02a\x1A\x0B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a\x19\xCEV[a\x1A\x15\x86\x83a\x19\xCEV[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[`\0\x81\x90P\x91\x90PV[`\0a\x1ARa\x1AMa\x1AH\x84a\x11\xFFV[a\x1A-V[a\x11\xFFV[\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a\x1Al\x83a\x1A7V[a\x1A\x80a\x1Ax\x82a\x1AYV[\x84\x84Ta\x19\xDBV[\x82UPPPPV[`\0\x90V[a\x1A\x95a\x1A\x88V[a\x1A\xA0\x81\x84\x84a\x1AcV[PPPV[[\x81\x81\x10\x15a\x1A\xC4Wa\x1A\xB9`\0\x82a\x1A\x8DV[`\x01\x81\x01\x90Pa\x1A\xA6V[PPV[`\x1F\x82\x11\x15a\x1B\tWa\x1A\xDA\x81a\x19\xA9V[a\x1A\xE3\x84a\x19\xBEV[\x81\x01` \x85\x10\x15a\x1A\xF2W\x81\x90P[a\x1B\x06a\x1A\xFE\x85a\x19\xBEV[\x83\x01\x82a\x1A\xA5V[PP[PPPV[`\0\x82\x82\x1C\x90P\x92\x91PPV[`\0a\x1B,`\0\x19\x84`\x08\x02a\x1B\x0EV[\x19\x80\x83\x16\x91PP\x92\x91PPV[`\0a\x1BE\x83\x83a\x1B\x1BV[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a\x1B^\x82a\x10\xB6V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1BwWa\x1Bva\x17AV[[a\x1B\x81\x82Ta\x15\xF6V[a\x1B\x8C\x82\x82\x85a\x1A\xC8V[`\0` \x90P`\x1F\x83\x11`\x01\x81\x14a\x1B\xBFW`\0\x84\x15a\x1B\xADW\x82\x87\x01Q\x90P[a\x1B\xB7\x85\x82a\x1B9V[\x86UPa\x1C\x1FV[`\x1F\x19\x84\x16a\x1B\xCD\x86a\x19\xA9V[`\0[\x82\x81\x10\x15a\x1B\xF5W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa\x1B\xD0V[\x86\x83\x10\x15a\x1C\x12W\x84\x89\x01Qa\x1C\x0E`\x1F\x89\x16\x82a\x1B\x1BV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[\x7FOwnable: caller is not the owner`\0\x82\x01RPV[`\0a\x1C]` \x83a\x10\xC1V[\x91Pa\x1Ch\x82a\x1C'V[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1C\x8C\x81a\x1CPV[\x90P\x91\x90PV[`\0\x81Q\x90P\x91\x90PV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[`\0a\x1C\xBA\x82Qa\x11hV[\x80\x91PP\x91\x90PV[`\0a\x1C\xCE\x82a\x1C\x93V[\x82a\x1C\xD8\x84a\x1C\x9EV[\x90Pa\x1C\xE3\x81a\x1C\xAEV[\x92P` \x82\x10\x15a\x1D#Wa\x1D\x1E\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83` \x03`\x08\x02a\x19\xCEV[\x83\x16\x92P[PP\x91\x90PV[`\0`\xA0\x82\x01\x90Pa\x1D?`\0\x83\x01\x88a\x11rV[a\x1DL` \x83\x01\x87a\x11rV[a\x1DY`@\x83\x01\x86a\x11rV[a\x1Df``\x83\x01\x85a\x13TV[a\x1Ds`\x80\x83\x01\x84a\x13\xB9V[\x96\x95PPPPPPV[`\0`\x80\x82\x01\x90Pa\x1D\x92`\0\x83\x01\x87a\x11rV[a\x1D\x9F` \x83\x01\x86a\x16'V[a\x1D\xAC`@\x83\x01\x85a\x11rV[a\x1D\xB9``\x83\x01\x84a\x11rV[\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[\x7FECDSA: invalid signature\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x1E'`\x18\x83a\x10\xC1V[\x91Pa\x1E2\x82a\x1D\xF1V[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1EV\x81a\x1E\x1AV[\x90P\x91\x90PV[\x7FECDSA: invalid signature length\0`\0\x82\x01RPV[`\0a\x1E\x93`\x1F\x83a\x10\xC1V[\x91Pa\x1E\x9E\x82a\x1E]V[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1E\xC2\x81a\x1E\x86V[\x90P\x91\x90PV[\x7FECDSA: invalid signature 's' val`\0\x82\x01R\x7Fue\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\x1F%`\"\x83a\x10\xC1V[\x91Pa\x1F0\x82a\x1E\xC9V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x1FT\x81a\x1F\x18V[\x90P\x91\x90PV\xFE\xA2dipfsX\"\x12 \xA4\x98~d\x84\xAB\xDC\xFE\x8D\xA0\t\xA3G\"\x14\xA7\xE1\x99\xFAIs\xA7_\xE5\r\x9Dj.\xD7\x80v\x81dsolcC\0\x08\x12\x003";
    /// The deployed bytecode of the contract.
    pub static TREASURY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Treasury<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Treasury<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Treasury<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Treasury<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Treasury<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Treasury)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Treasury<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    TREASURY_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                TREASURY_ABI.clone(),
                TREASURY_BYTECODE.clone(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function
        pub fn domain_separator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositPermit` (0x6b9d159d) function
        pub fn deposit_permit(
            &self,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([107, 157, 21, 157], (token, amount, deadline, v, r, s))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `eip712Domain` (0x84b0196e) function
        pub fn eip_712_domain(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                [u8; 1],
                ::std::string::String,
                ::std::string::String,
                ::ethers::core::types::U256,
                ::ethers::core::types::Address,
                [u8; 32],
                ::std::vec::Vec<::ethers::core::types::U256>,
            ),
        > {
            self.0
                .method_hash([132, 176, 25, 110], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonces` (0x7ecebe00) function
        pub fn nonces(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawPermit` (0xe9deb8ec) function
        pub fn withdraw_permit(
            &self,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [233, 222, 184, 236],
                    (token, amount, deadline, v, r, s, to),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Deposit` event
        pub fn deposit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositFilter> {
            self.0.event()
        }
        ///Gets the contract's `EIP712DomainChanged` event
        pub fn eip712_domain_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            Eip712DomainChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Withdraw` event
        pub fn withdraw_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            WithdrawFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TreasuryEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Treasury<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InvalidShortString` with signature `InvalidShortString()` and selector `0xb3512b0c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidShortString", abi = "InvalidShortString()")]
    pub struct InvalidShortString;
    ///Custom Error type `StringTooLong` with signature `StringTooLong(string)` and selector `0x305a27a9`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "StringTooLong", abi = "StringTooLong(string)")]
    pub struct StringTooLong {
        pub str: ::std::string::String,
    }
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum TreasuryErrors {
        InvalidShortString(InvalidShortString),
        StringTooLong(StringTooLong),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for TreasuryErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded)
                = <InvalidShortString as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidShortString(decoded));
            }
            if let Ok(decoded)
                = <StringTooLong as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::StringTooLong(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for TreasuryErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InvalidShortString(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StringTooLong(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for TreasuryErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <InvalidShortString as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <StringTooLong as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for TreasuryErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InvalidShortString(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StringTooLong(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for TreasuryErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidShortString> for TreasuryErrors {
        fn from(value: InvalidShortString) -> Self {
            Self::InvalidShortString(value)
        }
    }
    impl ::core::convert::From<StringTooLong> for TreasuryErrors {
        fn from(value: StringTooLong) -> Self {
            Self::StringTooLong(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Deposit", abi = "Deposit(address,address,address,uint256)")]
    pub struct DepositFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "EIP712DomainChanged", abi = "EIP712DomainChanged()")]
    pub struct Eip712DomainChangedFilter;
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "Withdraw",
        abi = "Withdraw(address,address,address,uint256,uint256,uint256)"
    )]
    pub struct WithdrawFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub nonce: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum TreasuryEvents {
        DepositFilter(DepositFilter),
        Eip712DomainChangedFilter(Eip712DomainChangedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        WithdrawFilter(WithdrawFilter),
    }
    impl ::ethers::contract::EthLogDecode for TreasuryEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(TreasuryEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = Eip712DomainChangedFilter::decode_log(log) {
                return Ok(TreasuryEvents::Eip712DomainChangedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(TreasuryEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = WithdrawFilter::decode_log(log) {
                return Ok(TreasuryEvents::WithdrawFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for TreasuryEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DepositFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::Eip712DomainChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::WithdrawFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DepositFilter> for TreasuryEvents {
        fn from(value: DepositFilter) -> Self {
            Self::DepositFilter(value)
        }
    }
    impl ::core::convert::From<Eip712DomainChangedFilter> for TreasuryEvents {
        fn from(value: Eip712DomainChangedFilter) -> Self {
            Self::Eip712DomainChangedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for TreasuryEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawFilter> for TreasuryEvents {
        fn from(value: WithdrawFilter) -> Self {
            Self::WithdrawFilter(value)
        }
    }
    ///Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    ///Container type for all input parameters for the `depositPermit` function with signature `depositPermit(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0x6b9d159d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "depositPermit",
        abi = "depositPermit(address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct DepositPermitCall {
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `eip712Domain` function with signature `eip712Domain()` and selector `0x84b0196e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "eip712Domain", abi = "eip712Domain()")]
    pub struct Eip712DomainCall;
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `withdrawPermit` function with signature `withdrawPermit(address,uint256,uint256,uint8,bytes32,bytes32,address)` and selector `0xe9deb8ec`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "withdrawPermit",
        abi = "withdrawPermit(address,uint256,uint256,uint8,bytes32,bytes32,address)"
    )]
    pub struct WithdrawPermitCall {
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum TreasuryCalls {
        DomainSeparator(DomainSeparatorCall),
        DepositPermit(DepositPermitCall),
        Eip712Domain(Eip712DomainCall),
        Name(NameCall),
        Nonces(NoncesCall),
        Owner(OwnerCall),
        RenounceOwnership(RenounceOwnershipCall),
        TransferOwnership(TransferOwnershipCall),
        WithdrawPermit(WithdrawPermitCall),
    }
    impl ::ethers::core::abi::AbiDecode for TreasuryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DomainSeparator(decoded));
            }
            if let Ok(decoded)
                = <DepositPermitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DepositPermit(decoded));
            }
            if let Ok(decoded)
                = <Eip712DomainCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Eip712Domain(decoded));
            }
            if let Ok(decoded)
                = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded)
                = <NoncesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Nonces(decoded));
            }
            if let Ok(decoded)
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded)
                = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded)
                = <WithdrawPermitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WithdrawPermit(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for TreasuryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DomainSeparator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DepositPermit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Eip712Domain(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nonces(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawPermit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for TreasuryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositPermit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Eip712Domain(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonces(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawPermit(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for TreasuryCalls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<DepositPermitCall> for TreasuryCalls {
        fn from(value: DepositPermitCall) -> Self {
            Self::DepositPermit(value)
        }
    }
    impl ::core::convert::From<Eip712DomainCall> for TreasuryCalls {
        fn from(value: Eip712DomainCall) -> Self {
            Self::Eip712Domain(value)
        }
    }
    impl ::core::convert::From<NameCall> for TreasuryCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<NoncesCall> for TreasuryCalls {
        fn from(value: NoncesCall) -> Self {
            Self::Nonces(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for TreasuryCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for TreasuryCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for TreasuryCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<WithdrawPermitCall> for TreasuryCalls {
        fn from(value: WithdrawPermitCall) -> Self {
            Self::WithdrawPermit(value)
        }
    }
    ///Container type for all return fields from the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `eip712Domain` function with signature `eip712Domain()` and selector `0x84b0196e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Eip712DomainReturn {
        pub fields: [u8; 1],
        pub name: ::std::string::String,
        pub version: ::std::string::String,
        pub chain_id: ::ethers::core::types::U256,
        pub verifying_contract: ::ethers::core::types::Address,
        pub salt: [u8; 32],
        pub extensions: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct NoncesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
}

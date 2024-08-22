// SPDX-License-Identifier: Apache-2.0
//
// Copyright © 2017 Trust Wallet.

use tw_encoding::hex::ToHex;
use tw_hd_wallet::ton::mnemonic::TonMnemonic;
use tw_hd_wallet::ton::TonWallet;
use tw_hd_wallet::WalletError;
use tw_keypair::traits::KeyPairTrait;
use tw_misc::traits::ToBytesZeroizing;

struct MnemonicTest {
    mnemonic: &'static str,
    passphrase: &'static str,
    expected_private: &'static str,
    expected_public: &'static str,
}

fn mnemonic_to_keypair_impl(input: MnemonicTest) {
    let passphrase = if input.passphrase.is_empty() {
        None
    } else {
        Some(input.passphrase.to_string())
    };

    let mnemonic = TonMnemonic::new(input.mnemonic).unwrap();
    let wallet = TonWallet::new(mnemonic, passphrase).unwrap();
    let key_pair = wallet.to_key_pair();

    assert_eq!(
        key_pair.private().to_zeroizing_vec().to_hex(),
        input.expected_private,
        "Invalid private key"
    );
    assert_eq!(
        key_pair.public().to_bytes().to_hex(),
        input.expected_public,
        "Invalid public key"
    );
}

struct MnemonicErrorTest {
    mnemonic: &'static str,
    passphrase: &'static str,
}

fn mnemonic_to_keypair_error(input: MnemonicErrorTest) {
    let passphrase = if input.passphrase.is_empty() {
        None
    } else {
        Some(input.passphrase.to_string())
    };

    let mnemonic = TonMnemonic::new(input.mnemonic).unwrap();
    assert!(
        TonWallet::new(mnemonic, passphrase).is_err(),
        "Expected an error"
    );
}

/// All tests generated by using `https://github.com/toncenter/tonweb-mnemonic/`.
#[test]
fn test_mnemonic_to_keypair_no_passphrase() {
    mnemonic_to_keypair_impl(MnemonicTest {
        mnemonic: "document shield addict crime broom point story depend suit satisfy test chicken valid tail speak fortune sound drill seek cube cheap body music recipe",
        passphrase: "",
        expected_private: "112d4e2e700a468f1eae699329202f1ee671d6b665caa2d92dea038cf3868c18",
        expected_public: "4d656c35c830bf78d239d3225727dd1da051be0ec521c98e3012beafbb06f306",
    });
    mnemonic_to_keypair_impl(MnemonicTest {
        mnemonic: "slogan train glide measure mercy dizzy when satoshi vote change length pluck token walnut actress hollow guard soup solve rival summer vicious anxiety device",
        passphrase: "",
        expected_private: "ee11da8e64d17a8416c88a6a24a1e16569cc85a077b7b209528975c32a44a0c8",
        expected_public: "3bab20a5f77e277e39443fc16c64e0479b4a9db542bf9e11c638598384c853f1",
    });
    mnemonic_to_keypair_impl(MnemonicTest {
        mnemonic: "laundry myself fitness beyond prize piano match acid vacuum already abandon dance occur pause grocery company inject excuse weasel carpet fog grunt trick spike",
        passphrase: "",
        expected_private: "859cd74ab605afb7ce9f5316a1f6d59217a130b75b494efd249913be874c9d46",
        expected_public: "c9af50596bd5c1c5a15fb32bef8d4f1ee5244b287aea1f49f6023a79f9b2f055",
    });
    mnemonic_to_keypair_impl(MnemonicTest {
        mnemonic: "slim holiday tiny pizza donor egg round three verify post chat social offer mix rack soft loud code option learn this pipe mouse mango",
        passphrase: "",
        expected_private: "cdfd1e2a1f947701bddba2636c26a6d6d13efacba5e6fdc624254be9bf8cbc3b",
        expected_public: "c1266dc4e8040e462af34fa7da9130950caf8c8a1bab78c9b938d2eb6e3d9a69",
    });
}

/// All tests generated by using `https://github.com/toncenter/tonweb-mnemonic/`.
#[test]
fn test_mnemonic_to_keypair_with_passphrase() {
    mnemonic_to_keypair_impl(MnemonicTest {
        mnemonic: "afford skate husband stamp style affair jeans episode afraid mom pupil canal borrow artwork fetch excite shiver conduct acoustic rail crisp consider pave people",
        passphrase: ".",
        expected_private: "ddaed03c283c9e60883b6c7cda86af40a1a820a8181276900094db0d23b55144",
        expected_public: "46aacb0e9e1faba24e0e87d4bf7ee5e54beaa4142fa1bd608324d7c67d78070e",
    });
    mnemonic_to_keypair_impl(MnemonicTest {
        mnemonic: "mimic close sibling chair shuffle goat fashion chunk increase tennis scene ceiling divert cross treat happy soccer sample umbrella oyster advance quality perfect call",
        passphrase: "My passphrase",
        expected_private: "78a6d95981847d6b7fb6b85be43071fbd83f4b0cebc1fd0c75147fde3c88d9e2",
        expected_public: "452a6031290df95e972a269f3c042f5b18497ab27b8fe9915e5b5c94037382a6",
    });
    mnemonic_to_keypair_impl(MnemonicTest {
        mnemonic: "kind loan rifle gadget forward tortoise switch tuition orchard ball monkey glow gallery diary nature dynamic survey flush correct employ autumn wife disease coin",
        passphrase: "189r012 9jr90fj--901hr8921'0r912j90",
        expected_private: "ced0feac4f8cc46909a5a172d390f126afe46540c07c5163c194429269e6eb08",
        expected_public: "482c9619307639a4b1699e83d771d656e5cf7be3ef877d849940cfebd718783e",
    });
    mnemonic_to_keypair_impl(MnemonicTest {
        mnemonic: " predict pelican worry swallow brother real truck fiber trophy melody joy kitten luggage lake woman clutch frost crop about stumble frozen kitchen mutual food ",
        passphrase: "Foo Bar Zar",
        expected_private: "f93ed43c379cb8210754016ffa669880b259854160446c1a12c5858258c32601",
        expected_public: "f44d7f29bc8801f882097611544fcafe84cca05408006fa5b76b63f55464e4d0",
    });
}

#[test]
fn test_mnemonic_to_keypair_error_expected_passphrase() {
    // This mnemonic can only be used along with the "My passphrase" passphrase.
    let mnemonic = "mimic close sibling chair shuffle goat fashion chunk increase tennis scene ceiling divert cross treat happy soccer sample umbrella oyster advance quality perfect call";
    mnemonic_to_keypair_error(MnemonicErrorTest {
        mnemonic,
        passphrase: "",
    });
    mnemonic_to_keypair_error(MnemonicErrorTest {
        mnemonic,
        passphrase: "Unexpected passphrase",
    });
}

#[test]
fn test_mnemonic_to_keypair_error_expected_no_passphrase() {
    // This mnemonic can only be used without passphrase.
    let mnemonic = "slogan train glide measure mercy dizzy when satoshi vote change length pluck token walnut actress hollow guard soup solve rival summer vicious anxiety device";
    mnemonic_to_keypair_error(MnemonicErrorTest {
        mnemonic,
        passphrase: "Hello world",
    });
    mnemonic_to_keypair_error(MnemonicErrorTest {
        mnemonic,
        passphrase: "...",
    });
}

#[test]
fn test_invalid_mnemonic() {
    // 24 words mnemonic is supported only.
    let error = TonMnemonic::new("cost dash dress stove morning robust group affair stomach vacant route volume yellow salute laugh").unwrap_err();
    assert_eq!(error, WalletError::InvalidMnemonicWordCount);

    let error = TonMnemonic::new("foo bar oooo edit wash faint patient cancel roof edit silly battle half engine reunion hotel joy fan unhappy oil alone sense empty mesh").unwrap_err();
    assert_eq!(error, WalletError::InvalidMnemonicUnknownWord);

    // Upper-case mnemonic is not allowed.
    let error = TonMnemonic::new("TAIL SWING SUGGEST EDIT WASH FAINT PATIENT CANCEL ROOF EDIT SILLY BATTLE HALF ENGINE REUNION HOTEL JOY FAN UNHAPPY OIL ALONE SENSE EMPTY MESH").unwrap_err();
    assert_eq!(error, WalletError::InvalidMnemonicUnknownWord);
}

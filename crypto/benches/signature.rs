#![allow(non_snake_case)]

extern crate criterion;
use criterion::{criterion_group, criterion_main, Criterion};
extern crate crypto;
use crypto::signature::{Signature, WeDPRSecp256k1Recover, WeDPRSm2p256v1};
extern crate common;

const SM2P256V1_TEST_SECRET_KEY: &str = "DSuhTJaEnZcJmPZN4JkpkIhADpT2pw5esjfOxiem8c0=";
const SM2P256V1_TEST_PUBLIC_KEY: &str = "A16vukzFH4WAINU6RIkRM4fvm47xJNFzkLmNgJFBB7Gp";

const SECP256K1_TEST_SECRET_KEY: &str = "oimstFzyjrJ8x2lF5gqUqqdhdLiUrOdJW3pBgJ/xZlU=";
const SECP256K1_TEST_PUBLIC_KEY: &str = "BMi7n85lnamNFtDNx7KCOHNb7R/+nmE9zXxb/X2wdxQn/\
                     UHMRv7OYdVu3+dhMwlcRxlz8JIE2WMsm6DbRixG9Ks=";

// fn create_secp256k1_signature_helper(c: &mut Criterion) {
//    let label = format!("create secp256k1 signature helper");
//    let message_hash = "hello WeDPR";
//    // 10c1b07e0a6a0d0554b153bb8f1527892358c533e319c6db7fa7a291a0082a88
//    let sk_valid = tests::TEST_SECRET_KEY;
//    // 02906b9a4d0f1f13e9e0496f9e1966c0488092555d0db5fe32272a30be7dd5e0b6
//    let pk_valid = tests::TEST_PUBLIC_KEY;
//    let sign_valid = SIGNATURE.sign(&sk_valid, message_hash).unwrap();

//    c.bench_function(&label, move |b| {
//        b.iter(|| {
//            // storage verify argument
//            let verify_valid =
//                SIGNATURE.verify(&pk_valid, message_hash, &sign_valid);
//            assert_eq!(verify_valid, true);
//        });
//    });
// }

fn create_sm2p256v1_signature_helper(c: &mut Criterion) {
    let label = format!("create sm2p256v1 signature helper");
    let sign_obj = WeDPRSm2p256v1::default();
    let (_pk, sk_str) = sign_obj.generate_keypair();
    let message = "10c1b07e0a6a0d0554b153bb8f1527892358c533e319c6db7fa7a291a0082a88";
    //    let pk_valid = TEST_PUBLIC_KEY;

    c.bench_function(&label, move |b| {
        b.iter(|| {
            // storage verify argument
            let sign = sign_obj.sign(&sk_str, &message).unwrap();
        });
    });
}

use common::constant::SM2_CTX;
use libsm::sm2::signature::Signature as sm2Signature;
fn create_sm2p256v1_signature_slow_helper(c: &mut Criterion) {
    let label = format!("create sm2p256v1 signature helper");
    let sign_obj = WeDPRSm2p256v1::default();
    let (pk, sk_str) = sign_obj.generate_keypair();
    let message = "10c1b07e0a6a0d0554b153bb8f1527892358c533e319c6db7fa7a291a0082a88";
    //    let pk_valid = TEST_PUBLIC_KEY;
    // let private_key_vec = common::utils::string_to_bytes(&sk_str).unwrap();
    //     let new_sk = SM2_CTX.load_seckey(&private_key_vec[..]).unwrap();
    //     let pk = SM2_CTX.pk_from_sk(&new_sk);
    //     // let signature:sm2Signature = SM2_CTX.sign(msg.as_bytes(), &new_sk, &pk);
    //     let signature: sm2Signature = SM2_CTX.sign(&common::utils::string_to_bytes(message).unwrap(), &new_sk, &pk);
    //     let (r, s) = signature.hex_encode();
    //     let signature = r + &s;
    // let private_key_vec = common::utils::string_to_bytes(&sk_str).unwrap();
    // let new_sk = SM2_CTX.load_seckey(&private_key_vec[..]).unwrap();

    c.bench_function(&label, move |b| {
        b.iter(|| {
            // storage verify argument
            // let sign = sign_obj.sign(&sk_str, &message).unwrap();
            let sign = sign_obj.sign_with_pub(&sk_str, &pk, &message).unwrap();
            // println!("sign = {}", sign);
            // let signature:sm2Signature = SM2_CTX.sign(msg.as_bytes(), &new_sk, &pk);
            // let pk = SM2_CTX.pk_from_sk(&new_sk);
            // let pk = SM2_CTX.load_pubkey(&common::utils::string_to_bytes(&pk).unwrap()).unwrap();
            // let signature: sm2Signature = SM2_CTX.sign(&common::utils::string_to_bytes(message).unwrap(), &new_sk, &pk);
            // let (r, s) = signature.hex_encode();
            // let signature = r + &s;
        });
    });
}

fn create_sm2p256v1_signature_verify_helper(c: &mut Criterion) {
    let label = format!("create sm2p256v1 signature verify helper");
    let message = "10c1b07e0a6a0d0554b153bb8f1527892358c533e319c6db7fa7a291a0082a88";
    let signature = WeDPRSm2p256v1::default();
    let (pk_str, sk_str) = signature.generate_keypair();
    let sign = signature.sign(&sk_str, &message).unwrap();

    c.bench_function(&label, move |b| {
        b.iter(|| {
            // storage verify argument
            let result = signature.verify(&pk_str, &message, &sign);
            assert_eq!(result, true);
        });
    });
}

fn create_secp256k1_signature_helper(c: &mut Criterion) {
    let label = format!("create secp256k1 signature helper");
    let message = "10c1b07e0a6a0d0554b153bb8f1527892358c533e319c6db7fa7a291a0082a88";
    let signature = WeDPRSecp256k1Recover::default();
    let (_pk_str, sk_str) = signature.generate_keypair();
    c.bench_function(&label, move |b| {
        b.iter(|| {
            // storage verify argument
            let sign = signature.sign(&sk_str, &message).unwrap();
        });
    });
}

fn create_secp256k1_signature_verify_helper(c: &mut Criterion) {
    let label = format!("create secp256k1 signature helper");
    let message = "10c1b07e0a6a0d0554b153bb8f1527892358c533e319c6db7fa7a291a0082a88";
    let signature = WeDPRSecp256k1Recover::default();
    let (pk_str, sk_str) = signature.generate_keypair();
    let sign = signature.sign(&sk_str, &message).unwrap();

    c.bench_function(&label, move |b| {
        b.iter(|| {
            // storage verify argument
            let result = signature.verify(&pk_str, &message, &sign);
            assert_eq!(result, true);
        });
    });
}

criterion_group! {
    name = create_signature;
    config = Criterion::default().sample_size(10);
    targets =
    // create_sm2p256v1_signature_helper,
    // create_sm2p256v1_signature_verify_helper,
    // create_secp256k1_signature_helper,
    // create_secp256k1_signature_verify_helper,
    create_sm2p256v1_signature_slow_helper,

}
criterion_main!(create_signature);

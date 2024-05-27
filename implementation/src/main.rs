/*
* This is the start of an implementation of the MAYO algo 
* The function are the implementation of https://pqmayo.org/assets/specs/mayo.pdf
*/
#![allow(unused)]
#![warn(non_snake_case)]
#![allow(warnings)]

//To use function of other files
#[path = "constants.rs"]
mod cons;
#[path = "matrix_arithmetics.rs"]
mod matrix;
mod utils;
use array_init::array_init;

/*
* Create a MayoParams struct instance
* All the values are from MAYO_1 instance for a NIST Security Level of 1
*/
const mayo: cons::MayoParams = cons::MayoParams {
    m: 64,
    n: 66,
    o: 8,
    k: 9,
    q: 16,
    f_tail: &[0, 1, 2],
    O_bytes: 232,
    v_bytes: 29,
    P1_bytes: 54752,
    P2_bytes: 14848,
    P3_bytes: 1152,
    L_bytes: 14848 ,
    csk_bytes: 24,
    esk_bytes: 69856,
    cpk_bytes: 1168,
    epk_bytes: 70752,
    sig_bytes: 321,
    salt_bytes: 24,
    digest_bytes: 32,
    pk_seed_bytes: 16,
    sk_seed_bytes: 24,
};


fn main() {
    println!("Start\n");
    // Create Key pair
    let (mut public,mut secret) = mayo_keypair();
    
    // Expand Secret Key
    //let expand_sk = mayo_expand_sk(secret, 64);

    // Expand public Key
    //let expand_pk = mayo_expand_pk(public);

    println!("Finished\n");
}

/**
 * Mayo keypair generation.
 *
 * The implementation corresponds to Mayo.CompactKeyGen() in the Mayo spec.
 *
 * @return cpk: compact public key
 * @return csk: compact secret key
 */
fn mayo_keypair() -> (Vec<u8>,Vec<u8>) {
    const S_SIZE: usize = mayo.pk_seed_bytes + mayo.O_bytes;
    
    let P1_bytes: usize = mayo.P1_bytes;
    let P2_bytes: usize = mayo.P2_bytes;
    let P3_bytes: usize = mayo.P3_bytes;

    //Derive seed_pk and O from seed_sk.
    let mut seed_sk = utils::random_bytes(mayo.sk_seed_bytes);
    let mut s = utils::shake256(&seed_sk, S_SIZE);
    let mut seed_pk = (&s[0..mayo.pk_seed_bytes]).to_vec();

    let mut byte_slice = (&s[mayo.pk_seed_bytes..mayo.pk_seed_bytes + mayo.O_bytes]);
    let mut o = utils::decode_o(byte_slice, mayo.n - mayo.o, mayo.o);

    //Derive the P(1) and P(2) from seed_pk.
    let seed_pk_as_array: &[u8; 16] = seed_pk.as_slice().try_into().expect("slice with incorrect length");
    let mut p = utils::aes_ctr(&seed_pk_as_array,&(P1_bytes+P2_bytes));
    
    let mut p1 = utils::decode_bitsliced_matrices(&p[0..P1_bytes],mayo.m,mayo.n - mayo.o, mayo.n - mayo.o, true);
    let mut p2 = utils::decode_bitsliced_matrices(&p[P1_bytes..P1_bytes + P2_bytes],mayo.m,mayo.n - mayo.o, mayo.o, false);
    
    //Compute the P(3) : P(3)i ← Upper(−O_T*P(1)*O − O_T*P2)

    //Tranpose of o
    let mut o_t: Vec<Vec<u8>> = matrix::transpose(&o);

    let mut p3 = Vec::new();
    for i in 0..mayo.m {
        let mut p1_matrix = &p1[i];
        let mut p2_matrix = &p2[i];

        //−O_T*P(1)*O -> term1
        let p1_multiplied = matrix::matrix_multiply(&o_t,&p1_matrix);
        let p1_multiplied_o = matrix::matrix_multiply(&o,&p1_multiplied);
        let neg_p1_multiplied_o = matrix::element_wise_negation(&p1_multiplied_o);

        //O_T*P2 -> term2
        let p2_multiplied = matrix::matrix_multiply(&o_t,&p2_matrix);
       
        //println!("{:?}, {:?}, {:?}",neg_p1_multiplied_o.len(),neg_p1_multiplied_o[0].len(),neg_p1_multiplied_o[0][0].len());
        let padded_p2_multiplied = matrix::pad_matrix(&p2_multiplied,neg_p1_multiplied_o.len(),neg_p1_multiplied_o[0].len(),neg_p1_multiplied_o[0][0].len());

        //−O_T*P(1)*O − O_T*P2
        let term1_sub_term2 = matrix::element_wise_subtraction(&neg_p1_multiplied_o,&padded_p2_multiplied);

        //p3.push(p3_matrix);
        let mut term1_sub_term2_f64: Vec<Vec<Vec<f64>>> = term1_sub_term2.iter()
        .map(|row| row.iter()
            .map(|inner| inner.iter()
                .map(|&val| val as f64) // Convert i32 to f64
                .collect::<Vec<_>>())
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();
        
        //Upper
        let p3_matrix = matrix::upper(&term1_sub_term2_f64[1], mayo.o); 
        
        
        p3.push(p3_matrix);
    }

    let mut csk = seed_pk.clone();
    let mut cpk = seed_pk.clone(); 
    let encoded_p3 = utils::encode_bitsliced_matrices(p3.clone(), mayo.m, mayo.o, mayo.o, true);

    // Faut faire ||
    // Pas juste un extend
    cpk.extend(encoded_p3.clone());

    return (csk,cpk);
}


/**
 * MAYO signature generation.
 *
 * The implementation performs Mayo.expandSK() + Mayo.sign() in the Mayo spec.
 *
 */
fn mayo_sign()  {

    //To implement

}

/**
 * Mayo verify signature.
 *
 * The implementation corresponds Mayo.verify() in the Mayo spec.
 */
fn mayo_verify() {

    //To implement

 }


/**
 * Mayo expand secret key.
 *
 * The implementation corresponds to Mayo.expandSK() in the Mayo spec.
 */
fn mayo_expand_sk(csk: Vec<u8>, m: usize) ->Vec<u8> {

    let mut seed_sk = (&csk[0..mayo.sk_seed_bytes]); // type [u8] , need [u8; 16] for later
    let mut seed_sk_array: [u8; 16] = [0; 16]; 
    for (i, &byte) in seed_sk.iter().enumerate() {
        seed_sk_array[i] = byte;
    }

    //Derive seedpk and O from seedsk.
    let mut s = utils::shake256(&seed_sk, mayo.pk_seed_bytes + mayo.O_bytes);
    
    let mut seed_pk = (&s[0..mayo.pk_seed_bytes]); 
    //let mut o_bymayoring = (&s[mayo.pk_seed_bytes..mayo.pk_seed_bytes + mayo.O_bytes]);
    //let mut o = decode(o_bymayoring);

    //Derive the P(1)iand P(2)ifrom seedpk.
    let mut p = utils::aes_ctr(&seed_sk_array,&(mayo.P1_bytes+mayo.P2_bytes));
    /*
    let mut p1 = decode(&p[0..P1_bytes]);
    let mut p2 = decode(&p[P1_bytes..P1_bytes + P2_bytes]);

    //Compute the Li.
    let mut i = 0;
    let mut l: [u8; m]
    while i < m {
        l[i] = (P(1) + P(1))O + P(2)
    }
    //Encode the Li and output esk.
    let mut esk = seedsk + o_bymayoring + &p[0..P1_bytes] + encode(? ? ?);
    return esk;
    */
    return csk;
}


/**
 * Mayo expand public key.
 *
 * The implementation corresponds to Mayo.expandPK() in the Mayo spec.
 */
fn mayo_expand_pk(cpk: Vec<u8>) ->Vec<u8> {

    let mut seed_pk = (&cpk[0..mayo.pk_seed_bytes]);// type [u8] , need [u8; 16] for later
    let mut seed_pk_array: [u8; 16] = [0; 16]; 
    for (i, &byte) in seed_pk.iter().enumerate() {
        seed_pk_array[i] = byte;
    }

    let mut epk = utils::aes_ctr(&seed_pk_array,&(mayo.P1_bytes+mayo.P2_bytes)); 
    // epk = AES-128-CTR(seedpk, P1 bytes + P2 bytes) ∥ cpk[pk seed bytes : pk seed bytes + P3 bytes]

    return epk;
}

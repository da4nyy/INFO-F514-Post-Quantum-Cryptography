use aes::Aes128;
use ctr::cipher::{KeyIvInit, StreamCipher};
type AesCtr128 = ctr::Ctr128BE<Aes128>;
use rand::rngs::OsRng;
use rand::RngCore;
use sha3::{Shake256, digest::{ExtendableOutput, Update, XofReader}};

/**
 * SHA-3 standard
 * @param data : data to hash
 * @param output_len : Size of the output
 * @return output: Vec<u8> of length output_len
 */
pub fn shake256(data: &[u8], output_len: usize) -> Vec<u8>{

    let mut hasher = Shake256::default();
    hasher.update(data);
    let mut reader = hasher.finalize_xof();

    let mut output = vec![0u8; output_len];
    reader.read(&mut output);

    output
}

/**
 * AES-128-CTR
 * @param key : 16-byte seed
 * @param out_length : Size of the output
 * @return buffer:  Vec<u8> of length out_length
 */
pub fn aes_ctr<'a>(key: &'a [u8; 16], out_length: &'a usize) -> Vec<u8> {

    let nonce = [0u8; 16];
    let mut cipher = AesCtr128::new(key.into(), &nonce.into());
    let mut buffer = vec![0u8; *out_length];
    cipher.apply_keystream(&mut buffer) ;

    buffer
}

/**
 * Random byte generation.
 * @param xlen: Number of random bytes to be generated
 * @return rand: Vec<u8> of length xlen
 */
pub fn random_bytes(xlen: usize) -> Vec<u8>{
    let mut rand = vec![0; xlen];

    let mut rng = OsRng;
    rng.fill_bytes(&mut rand[..xlen]);

    rand
}



//EncodeBitslicedMatrices(r, c, {A}i∈[m], is_triangular)
//r, c, the number of rows and columns of the matrices
pub fn encode_bitsliced_matrices(a:Vec<Vec<Vec<f64>>>, m: usize, r:usize, c:usize, is_triangular:bool) -> Vec<u8> {
    let mut bytestring = Vec::new();

    for i in 0..r {
        for j in 0..c {
            if i <= j || !is_triangular {
                let mut vec_to_encode = Vec::with_capacity(a.len());
                for k in 0..a.len() {
                    vec_to_encode.push(a[k][i][j]);
                }
                bytestring.extend(encode_bitsliced_vector(&vec_to_encode));
            }
        }
    }
    return bytestring;
}

//Input: A vector v ∈ Fm16
//Output: A byte string bytestring ∈ Bm/2 that encodes v in a bitsliced format
pub fn encode_bitsliced_vector(v:&Vec<f64>) -> Vec<u8> {
    let m = v.len();
    let mut byte_string = vec![0u8; (m * 64) / 8]; 

    for i in 0..(m / 8) {
        let mut bit_columns = vec![0u8; 64];

        // Process each `f64` value and extract its bits
        for j in 0..8 {
            let bits = v[8 * i + j].to_bits(); 

            for k in 0..64 {
                let bit = ((bits >> k) & 1) as u8;
                bit_columns[k] |= bit << j;
            }
        }

        for (k, &column) in bit_columns.iter().enumerate() {
            byte_string[i * 64 + k] = column;
        }
    }

    byte_string
}

pub fn decode_bitsliced_matrices(bytestring: &[u8], m: usize,r: usize,c: usize,is_triangular: bool) -> Vec<Vec<Vec<Vec<u16>>>> {
    let mut a = vec![vec![vec![vec![0u16; 16]; c]; r]; m];
    let mut index = 0;
    
    for i in 0..r {
        for j in 0..c {
            if i <= j || !is_triangular {
                for k in 0..m { 
                    if index + (16 / 2) <= bytestring.len() {
                        let slice = &bytestring[index..index + (16 / 2)];
                        a[k][i][j] = decode_bitsliced_vector(slice, 16);
                        index += 16 / 2;
                    }
                    else {
                        break;
                    }
                }
            }
        }
    }

    return a;
}

pub fn decode_bitsliced_vector(bytestring: &[u8], m: usize) -> Vec<u16> {
    let mut v = vec![0u16; m];

    for i in 0..(m / 8) {
        let b0 = bytestring[i];
        let b1 = bytestring[(m / 8) + i];
        let b2 = bytestring[(2 * m / 8) + i];
        let b3 = bytestring[(3 * m / 8) + i];

        for j in 0..8 {
            let idx = i * 8 + j;
            v[idx] = ((b0 >> (7 - j)) & 0x1) as u16
                   | (((b1 >> (7 - j)) & 0x1) as u16) << 1
                   | (((b2 >> (7 - j)) & 0x1) as u16) << 2
                   | (((b3 >> (7 - j)) & 0x1) as u16) << 3;
        }
    }

    return v;
}

pub fn decode_o(bytes: &[u8], rows: usize, cols: usize) -> Vec<Vec<u8>> {
    let mut matrix: Vec<Vec<u8>> = Vec::with_capacity(rows);
    for r in 0..rows {
        let start = r * cols;
        let end = start + cols;
        if end <= bytes.len() {
            let row: Vec<u8> = bytes[start..end].to_vec();
            matrix.push(row);
        }
        else {
            break;
        }
    }
    matrix
}

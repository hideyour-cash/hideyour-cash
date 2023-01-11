use near_bigint::U256;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

mod internal;

// constants
const Q: &str = "21888242871839275222246405745257275088548364400416034343698204186575808495617";
const QF: &str = "21888242871839275222246405745257275088696311157297823662689037894645226208583";
const G1_X: &str = "1";
const G1_Y: &str = "2";
const G2_X1: &str = "10857046999023057135944570762232829481370756359578518086990519993285655852781";
const G2_X2: &str = "11559732032986387107991004021392285783925812861821192530917403151452391805634";
const G2_Y1: &str = "8495653923123431417604973247489272438418190587263600148770280649306958101930";
const G2_Y2: &str = "4082367875863433681332203403145435568316851327593401208105741076214120093531";
const PA: u16 = 32;
const PB: u16 = 96;
const PC: u16 = 160;
const PZ: u16 = 224;
const PT1: u16 = 288;
const PT2: u16 = 352;
const PT3: u16 = 416;
const P_WXI: u16 = 480;
const P_WXIW: u16 = 544;
const P_EVAL_A: u16 = 608;
const P_EVAL_B: u16 = 640;
const P_EVAL_C: u16 = 672;
const P_EVAL_S1: u16 = 704;
const P_EVAL_S2: u16 = 736;
const P_EVAL_ZW: u16 = 768;
const P_EVAL_R: u16 = 800;
const P_ALPHA: u16 = 0;
const P_BETA: u16 = 32;
const P_GAMMA: u16 = 64;
const P_XI: u16 = 96;
const P_XIN: u16 = 128;
const P_BETA_XI: u16 = 160;
const P_V1: u16 = 192;
const P_V2: u16 = 224;
const P_V3: u16 = 256;
const P_V4: u16 = 288;
const P_V5: u16 = 320;
const P_V6: u16 = 352;
const P_U: u16 = 384;
const P_PL: u16 = 416;
const P_EVAL_T: u16 = 448;
const P_A1: u16 = 480;
const P_B1: u16 = 544;
const P_ZH: u16 = 608;
const P_ZH_INV: u16 = 640;


/// Verifier struct implementing plonk verifier
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Verifier {
    pub n: u32,
    pub n_public: u16,
    pub n_lagrange: u16,

    pub qmx: U256,
    pub qmy: U256,
    pub qlx: U256,
    pub qly: U256,
    pub qrx: U256,
    pub qry: U256,
    pub qox: U256,
    pub qoy: U256,
    pub qcx: U256,
    pub qcy: U256,
    pub s1x: U256,
    pub s1y: U256,
    pub s2x: U256,
    pub s2y: U256,
    pub s3x: U256,
    pub s3y: U256,
    pub k1: U256,
    pub k2: U256,
    pub x2x1: U256,
    pub x2x2: U256,
    pub x2y1: U256,
    pub x2y2: U256,

    pub q: U256,
    pub qf: U256,
    pub w1: U256,
    
    pub g1x: U256,
    pub g1y: U256,
    pub g2x1: U256,
    pub g2x2: U256,
    pub g2y1: U256,
    pub g2y2: U256,
    
    pub p_a: u16,
    pub p_b: u16,
    pub p_c: u16,
    pub p_z: u16,
    pub p_t1: u16,
    pub p_t2: u16,
    pub p_t3: u16,
    pub p_wxi: u16,
    pub p_wxiw: u16,
    pub p_eval_a: u16,
    pub p_eval_b: u16,
    pub p_eval_c: u16,
    pub p_eval_s1: u16,
    pub p_eval_s2: u16,
    pub p_eval_zw: u16,
    pub p_eval_r: u16,


    pub p_alpha: u16,
    pub p_beta: u16,
    pub p_gamma: u16,
    pub p_xi: u16,
    pub p_xin: u16,
    pub p_beta_xi: u16,
    pub p_v1: u16,
    pub p_v2: u16,
    pub p_v3: u16,
    pub p_v4: u16,
    pub p_v5: u16,
    pub p_v6: u16,
    pub p_u: u16,
    pub p_pl: u16,
    pub p_eval_t: u16,
    pub p_a1: u16,
    pub p_b1: u16,
    pub p_zh: u16,
    pub p_zh_inv: u16,

    pub p_eval_l: Vec<u16>,
    pub p_last_mem: u16,
}

impl Verifier {
    pub fn new(
        n_public: u16,
        power: u32,
        qmx: U256,
        qmy: U256,
        qlx: U256,
        qly: U256,
        qrx: U256,
        qry: U256,
        qox: U256,
        qoy: U256,
        qcx: U256,
        qcy: U256,
        s1x: U256,
        s1y: U256,
        s2x: U256,
        s2y: U256,
        s3x: U256,
        s3y: U256,
        x2x1: U256,
        x2x2: U256,
        x2y1: U256,
        x2y2: U256,
        w: U256,
    ) -> Self {

        let n_lagrange = if n_public > 1 { n_public } else { 1 };

        let mut p_eval_l = Vec::new();

        let mut i = 1;
        while i <= n_lagrange {
            p_eval_l.push(640 + i * 32);
            i += 1;
        }

        Self {
            n: 2u32.pow(power),
            n_public,
            n_lagrange,
            qmx,
            qmy: if qmx == U256::zero() { U256::zero() } else { qmy },
            qlx,
            qly: if qlx == U256::zero() { U256::zero() } else { qly },
            qrx,
            qry: if qrx == U256::zero() { U256::zero() } else { qry },
            qox,
            qoy: if qox == U256::zero() { U256::zero() } else { qoy },
            qcx,
            qcy: if qcx == U256::zero() { U256::zero() } else { qcy },
            s1x,
            s1y: if s1x == U256::zero() { U256::zero() } else { s1y },
            s2x,
            s2y: if s2x == U256::zero() { U256::zero() } else { s2y },
            s3x,
            s3y: if s3x == U256::zero() { U256::zero() } else { s3y },
            k1: U256::from(2),
            k2: U256::from(3),
            x2x1,
            x2x2,
            x2y1,
            x2y2,
            q: U256::from_dec_str(Q).unwrap(),
            qf: U256::from_dec_str(QF).unwrap(),
            w1: w,
            g1x: U256::from_dec_str(G1_X).unwrap(),
            g1y: U256::from_dec_str(G1_Y).unwrap(),
            g2x1: U256::from_dec_str(G2_X1).unwrap(),
            g2x2: U256::from_dec_str(G2_X2).unwrap(),
            g2y1: U256::from_dec_str(G2_Y1).unwrap(),
            g2y2: U256::from_dec_str(G2_Y2).unwrap(),
            p_a: PA,
            p_b: PB,
            p_c: PC,
            p_z: PZ,
            p_t1: PT1,
            p_t2: PT2,
            p_t3: PT3,
            p_wxi: P_WXI,
            p_wxiw: P_WXIW,
            p_eval_a: P_EVAL_A,
            p_eval_b: P_EVAL_B,
            p_eval_c: P_EVAL_C,
            p_eval_s1: P_EVAL_S1,
            p_eval_s2: P_EVAL_S2,
            p_eval_zw: P_EVAL_ZW,
            p_eval_r: P_EVAL_R,
            p_alpha: P_ALPHA,
            p_beta: P_BETA,
            p_gamma: P_GAMMA,
            p_xi: P_XI,
            p_xin: P_XIN,
            p_beta_xi: P_BETA_XI,
            p_v1: P_V1,
            p_v2: P_V2,
            p_v3: P_V3,
            p_v4: P_V4,
            p_v5: P_V5,
            p_v6: P_V6,
            p_u: P_U,
            p_pl: P_PL,
            p_eval_t: P_EVAL_T,
            p_a1: P_A1,
            p_b1: P_B1,
            p_zh: P_ZH,
            p_zh_inv: P_ZH_INV,
            p_eval_l,
            p_last_mem: 672 + 32 * n_lagrange,

        }
    }

    pub fn verify(&self, input: Vec<U256>, proof: Vec<U256>) -> bool {
        true
    }
}

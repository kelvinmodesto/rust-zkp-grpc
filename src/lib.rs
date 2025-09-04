use num_bigint::{BigUint, RandBigInt};
use rand::{Rng, distributions::Alphanumeric, thread_rng};

pub struct ZKP {
    pub p: BigUint,
    pub q: BigUint,
    pub alpha: BigUint,
    pub beta: BigUint,
}

impl ZKP {
    /// output = (alpha^exp mod p, beta^exp mod p)
    pub fn compute_pair(&self, exp: &BigUint) -> (BigUint, BigUint) {
        let p1 = self.alpha.modpow(exp, &self.p);
        let p2 = self.beta.modpow(exp, &self.p);
        (p1, p2)
    }
    /// output => n^exp mod p
    pub fn exponentiate(n: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
        n.modpow(exponent, modulus)
    }

    /// output => s = k - c * x mod q
    pub fn solve(&self, k: &BigUint, c: &BigUint, x: &BigUint) -> BigUint {
        if *k >= c * x {
            return (k - c * x).modpow(&BigUint::from(1u32), &self.q);
        }
        &self.q - (c * x - k).modpow(&BigUint::from(1u32), &self.q)
    }
    /// cond1 => r1 = alpha^s * y1^c
    /// cond2 => r2 = beta^s * y2^c
    /// output =>
    pub fn verify(
        &self,
        r1: &BigUint,
        r2: &BigUint,
        y1: &BigUint,
        y2: &BigUint,
        c: &BigUint,
        s: &BigUint,
    ) -> bool {
        let cond1: bool = *r1
            == (&self.alpha.modpow(s, &self.p) * y1.modpow(c, &self.p))
                .modpow(&BigUint::from(1u32), &self.p);
        let cond2: bool = *r2
            == (&self.beta.modpow(s, &self.p) * y2.modpow(c, &self.p))
                .modpow(&BigUint::from(1u32), &self.p);
        cond1 && cond2
    }

    pub fn generate_random_lower_than(bound: &BigUint) -> BigUint {
        let mut rn = thread_rng();
        rn.gen_biguint_below(bound)
    }

    pub fn generate_random_string(size: usize) -> String {
        thread_rng()
            .sample_iter(Alphanumeric)
            .take(size)
            .map(char::from)
            .collect()
    }

    pub fn get_constants() -> (BigUint, BigUint, BigUint, BigUint) {
        let p = BigUint::from_bytes_be(&hex::decode("B10B8F96A080E01DDE92DE5EAE5D54EC52C99FBCFB06A3C69A6A9DCA52D23B616073E28675A23D189838EF1E2EE652C013ECB4AEA906112324975C3CD49B83BFACCBDD7D90C4BD7098488E9C219A73724EFFD6FAE5644738FAA31A4FF55BCCC0A151AF5F0DC8B4BD45BF37DF365C1A65E68CFDA76D4DA708DF1FB2BC2E4A4371").unwrap());
        let q = BigUint::from_bytes_be(
            &hex::decode("F518AA8781A8DF278ABA4E7D64B7CB9D49462353").unwrap(),
        );

        let alpha = BigUint::from_bytes_be(
                &hex::decode("A4D1CBD5C3FD34126765A442EFB99905F8104DD258AC507FD6406CFF14266D31266FEA1E5C41564B777E690F5504F213160217B4B01B886A5E91547F9E2749F4D7FBD7D3B9A92EE1909D0D2263F80A76A6A24C087A091F531DBF0A0169B6A28AD662A4D18E73AFA32D779D5918D08BC8858F4DCEF97C2A24855E6EEB22B3B2E5").unwrap(),
            );

        // beta = alpha^i is also a generator
        let exp = BigUint::from_bytes_be(&hex::decode("266FEA1E5C41564B777E69").unwrap());
        let beta = alpha.modpow(&exp, &p);

        (alpha, beta, p, q)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toy_example() {
        let alpha = BigUint::from(4u32);
        let beta = BigUint::from(9u32);
        let p = BigUint::from(23u32);
        let q = BigUint::from(11u32);

        let zkp = ZKP {
            p: p.clone(),
            q,
            alpha: alpha.clone(),
            beta: beta.clone(),
        };

        let x = BigUint::from(6u32);
        let k = BigUint::from(7u32);

        let c = BigUint::from(4u32);

        let y1 = ZKP::exponentiate(&alpha, &x, &p);
        let y2 = ZKP::exponentiate(&beta, &x, &p);

        assert_eq!(y1, BigUint::from(2u32));
        assert_eq!(y2, BigUint::from(3u32));

        let r1 = ZKP::exponentiate(&alpha, &k, &p);
        let r2 = ZKP::exponentiate(&beta, &k, &p);

        assert_eq!(r1, BigUint::from(8u32));
        assert_eq!(r2, BigUint::from(4u32));

        let s = zkp.solve(&k, &c, &x);

        assert_eq!(s, BigUint::from(5u32));

        let result: bool = zkp.verify(&r1, &r2, &y1, &y2, &c, &s);
        assert!(result);

        let x_fake = BigUint::from(7u32);
        let s_fake = zkp.solve(&k, &c, &x_fake);

        let result_fake: bool = zkp.verify(&r1, &r2, &y1, &y2, &c, &s_fake);
        assert!(!result_fake);
    }

    #[test]
    fn test_toy_example_with_random_numbers() {
        let alpha = BigUint::from(4u32);
        let beta = BigUint::from(9u32);

        let p = BigUint::from(23u32);
        let q = BigUint::from(11u32);

        let zkp = ZKP {
            p: p.clone(),
            q: q.clone(),
            alpha: alpha.clone(),
            beta: beta.clone(),
        };

        let x = BigUint::from(6u32);
        let k = ZKP::generate_random_lower_than(&q);

        let c = ZKP::generate_random_lower_than(&q);

        let y1 = ZKP::exponentiate(&alpha, &x, &p);
        let y2 = ZKP::exponentiate(&beta, &x, &p);

        assert_eq!(y1, BigUint::from(2u32));
        assert_eq!(y2, BigUint::from(3u32));

        let r1 = ZKP::exponentiate(&alpha, &k, &p);
        let r2 = ZKP::exponentiate(&beta, &k, &p);

        let s = zkp.solve(&k, &c, &x);

        let result: bool = zkp.verify(&r1, &r2, &y1, &y2, &c, &s);
        assert!(result);
    }

    #[test]
    fn test_1024_constants() {
        //
        //    Reference: https://www.rfc-editor.org/rfc/rfc5114#page-15
        //
        //    p=B10B8F96A080E01DDE92DE5EAE5D54EC52C99FBCFB06A3C69A6A9DCA52D23B616073E28675A23D189838EF1E2EE652C013ECB4AEA906112324975C3CD49B83BFACCBDD7D90C4BD7098488E9C219A73724EFFD6FAE5644738FAA31A4FF55BCCC0A151AF5F0DC8B4BD45BF37DF365C1A65E68CFDA76D4DA708DF1FB2BC2E4A4371
        //    q=F518AA8781A8DF278ABA4E7D64B7CB9D49462353
        //    alpha=A4D1CBD5C3FD34126765A442EFB99905F8104DD258AC507FD6406CFF14266D31266FEA1E5C41564B777E690F5504F213160217B4B01B886A5E91547F9E2749F4D7FBD7D3B9A92EE1909D0D2263F80A76A6A24C087A091F531DBF0A0169B6A28AD662A4D18E73AFA32D779D5918D08BC8858F4DCEF97C2A24855E6EEB22B3B2E5
        //
        let p = BigUint::from_bytes_be(&hex::decode(
            "B10B8F96A080E01DDE92DE5EAE5D54EC52C99FBCFB06A3C69A6A9DCA52D23B616073E28675A23D189838EF1E2EE652C013ECB4AEA906112324975C3CD49B83BFACCBDD7D90C4BD7098488E9C219A73724EFFD6FAE5644738FAA31A4FF55BCCC0A151AF5F0DC8B4BD45BF37DF365C1A65E68CFDA76D4DA708DF1FB2BC2E4A4371",
        ).unwrap());
        let q = BigUint::from_bytes_be(
            &hex::decode("F518AA8781A8DF278ABA4E7D64B7CB9D49462353").unwrap(),
        );

        let alpha = BigUint::from_bytes_be(&hex::decode(
            "A4D1CBD5C3FD34126765A442EFB99905F8104DD258AC507FD6406CFF14266D31266FEA1E5C41564B777E690F5504F213160217B4B01B886A5E91547F9E2749F4D7FBD7D3B9A92EE1909D0D2263F80A76A6A24C087A091F531DBF0A0169B6A28AD662A4D18E73AFA32D779D5918D08BC8858F4DCEF97C2A24855E6EEB22B3B2E5",
        ).unwrap());
        let beta = alpha.modpow(&ZKP::generate_random_lower_than(&q), &p);

        let zkp = ZKP {
            p: p.clone(),
            q: q.clone(),
            alpha: alpha.clone(),
            beta: beta.clone(),
        };

        let x = ZKP::generate_random_lower_than(&q);
        let k = ZKP::generate_random_lower_than(&q);

        let c = ZKP::generate_random_lower_than(&q);

        let y1 = ZKP::exponentiate(&alpha, &x, &p);
        let y2 = ZKP::exponentiate(&beta, &x, &p);

        let r1 = ZKP::exponentiate(&alpha, &k, &p);
        let r2 = ZKP::exponentiate(&beta, &k, &p);

        let s = zkp.solve(&k, &c, &x);

        let result: bool = zkp.verify(&r1, &r2, &y1, &y2, &c, &s);
        assert!(result);
    }

    #[test]
    fn test_2048_constants() {
        //
        //    Reference: https://www.rfc-editor.org/rfc/rfc5114#page-15
        //
        //    p=AD107E1E9123A9D0D660FAA79559C51FA20D64E5683B9FD1B54B1597B61D0A75E6FA141DF95A56DBAF9A3C407BA1DF15EB3D688A309C180E1DE6B85A1274A0A66D3F8152AD6AC2129037C9EDEFDA4DF8D91E8FEF55B7394B7AD5B7D0B6C12207C9F98D11ED34DBF6C6BA0B2C8BBC27BE6A00E0A0B9C49708B3BF8A317091883681286130BC8985DB1602E714415D9330278273C7DE31EFDC7310F7121FD5A07415987D9ADC0A486DCDF93ACC44328387315D75E198C641A480CD86A1B9E587E8BE60E69CC928B2B9C52172E413042E9B23F10B0E16E79763C9B53DCF4BA80A29E3FB73C16B8E75B97EF363E2FFA31F71CF9DE5384E71B81C0AC4DFFE0C10E64F
        //    q=801C0D34C58D93FE997177101F80535A4738CEBCBF389A99B36371EB
        //    alpha=AC4032EF4F2D9AE39DF30B5C8FFDAC506CDEBE7B89998CAF74866A08CFE4FFE3A6824A4E10B9A6F0DD921F01A70C4AFAAB739D7700C29F52C57DB17C620A8652BE5E9001A8D66AD7C17669101999024AF4D027275AC1348BB8A762D0521BC98AE247150422EA1ED409939D54DA7460CDB5F6C6B250717CBEF180EB34118E98D119529A45D6F834566E3025E316A330EFBB77A86F0C1AB15B051AE3D428C8F8ACB70A8137150B8EEB10E183EDD19963DDD9E263E4770589EF6AA21E7F5F2FF381B539CCE3409D13CD566AFBB48D6C019181E1BCFE94B30269EDFE72FE9B6AA4BD7B5A0F1C71CFFF4C19C418E1F6EC017981BC087F2A7065B384B890D3191F2BFA
        //
        let p = BigUint::from_bytes_be(&hex::decode(
            "AD107E1E9123A9D0D660FAA79559C51FA20D64E5683B9FD1B54B1597B61D0A75E6FA141DF95A56DBAF9A3C407BA1DF15EB3D688A309C180E1DE6B85A1274A0A66D3F8152AD6AC2129037C9EDEFDA4DF8D91E8FEF55B7394B7AD5B7D0B6C12207C9F98D11ED34DBF6C6BA0B2C8BBC27BE6A00E0A0B9C49708B3BF8A317091883681286130BC8985DB1602E714415D9330278273C7DE31EFDC7310F7121FD5A07415987D9ADC0A486DCDF93ACC44328387315D75E198C641A480CD86A1B9E587E8BE60E69CC928B2B9C52172E413042E9B23F10B0E16E79763C9B53DCF4BA80A29E3FB73C16B8E75B97EF363E2FFA31F71CF9DE5384E71B81C0AC4DFFE0C10E64F",
        ).unwrap());
        let q = BigUint::from_bytes_be(
            &hex::decode("801C0D34C58D93FE997177101F80535A4738CEBCBF389A99B36371EB").unwrap(),
        );

        let alpha = BigUint::from_bytes_be(&hex::decode(
            "AC4032EF4F2D9AE39DF30B5C8FFDAC506CDEBE7B89998CAF74866A08CFE4FFE3A6824A4E10B9A6F0DD921F01A70C4AFAAB739D7700C29F52C57DB17C620A8652BE5E9001A8D66AD7C17669101999024AF4D027275AC1348BB8A762D0521BC98AE247150422EA1ED409939D54DA7460CDB5F6C6B250717CBEF180EB34118E98D119529A45D6F834566E3025E316A330EFBB77A86F0C1AB15B051AE3D428C8F8ACB70A8137150B8EEB10E183EDD19963DDD9E263E4770589EF6AA21E7F5F2FF381B539CCE3409D13CD566AFBB48D6C019181E1BCFE94B30269EDFE72FE9B6AA4BD7B5A0F1C71CFFF4C19C418E1F6EC017981BC087F2A7065B384B890D3191F2BFA",
        ).unwrap());
        let beta = alpha.modpow(&ZKP::generate_random_lower_than(&q), &p);

        let zkp = ZKP {
            p: p.clone(),
            q: q.clone(),
            alpha: alpha.clone(),
            beta: beta.clone(),
        };

        let x = ZKP::generate_random_lower_than(&q);
        let k = ZKP::generate_random_lower_than(&q);

        let c = ZKP::generate_random_lower_than(&q);

        let y1 = ZKP::exponentiate(&alpha, &x, &p);
        let y2 = ZKP::exponentiate(&beta, &x, &p);

        let r1 = ZKP::exponentiate(&alpha, &k, &p);
        let r2 = ZKP::exponentiate(&beta, &k, &p);

        let s = zkp.solve(&k, &c, &x);

        let result: bool = zkp.verify(&r1, &r2, &y1, &y2, &c, &s);
        assert!(result);
    }

    #[test]
    fn test_compute_pair_basic() {
        let alpha = BigUint::from(4u32);
        let beta = BigUint::from(9u32);
        let p = BigUint::from(23u32);
        let q = BigUint::from(11u32);

        let zkp = ZKP {
            p: p.clone(),
            q,
            alpha,
            beta,
        };

        // Test with exponent = 1
        let exp = BigUint::from(1u32);
        let (p1, p2) = zkp.compute_pair(&exp);
        assert_eq!(p1, BigUint::from(4u32)); // alpha^1 mod p = 4
        assert_eq!(p2, BigUint::from(9u32)); // beta^1 mod p = 9

        // Test with exponent = 2
        let exp = BigUint::from(2u32);
        let (p1, p2) = zkp.compute_pair(&exp);
        assert_eq!(p1, BigUint::from(16u32)); // 4^2 mod 23 = 16
        assert_eq!(p2, BigUint::from(12u32)); // 9^2 mod 23 = 81 mod 23 = 12

        // Test with exponent = 0
        let exp = BigUint::from(0u32);
        let (p1, p2) = zkp.compute_pair(&exp);
        assert_eq!(p1, BigUint::from(1u32)); // alpha^0 mod p = 1
        assert_eq!(p2, BigUint::from(1u32)); // beta^0 mod p = 1
    }

    #[test]
    fn test_compute_pair_with_large_exponent() {
        let alpha = BigUint::from(4u32);
        let beta = BigUint::from(9u32);
        let p = BigUint::from(23u32);
        let q = BigUint::from(11u32);

        let zkp = ZKP {
            p: p.clone(),
            q,
            alpha: alpha.clone(),
            beta: beta.clone(),
        };

        // Test with a larger exponent
        let exp = BigUint::from(6u32);
        let (p1, p2) = zkp.compute_pair(&exp);

        // Verify by computing manually
        let expected_p1 = alpha.modpow(&exp, &p);
        let expected_p2 = beta.modpow(&exp, &p);

        assert_eq!(p1, expected_p1);
        assert_eq!(p2, expected_p2);
    }

    #[test]
    fn test_compute_pair_with_constants() {
        let (alpha, beta, p, q) = ZKP::get_constants();

        let zkp = ZKP {
            p: p.clone(),
            q: q.clone(),
            alpha: alpha.clone(),
            beta: beta.clone(),
        };

        let exp = BigUint::from(5u32);
        let (p1, p2) = zkp.compute_pair(&exp);

        // Verify the results match manual computation
        let expected_p1 = alpha.modpow(&exp, &p);
        let expected_p2 = beta.modpow(&exp, &p);

        assert_eq!(p1, expected_p1);
        assert_eq!(p2, expected_p2);

        // Ensure results are within modulus bounds
        assert!(p1 < p);
        assert!(p2 < p);
    }

    #[test]
    fn test_get_constants_values() {
        let (alpha, beta, p, q) = ZKP::get_constants();

        // Test that p matches the expected 1024-bit prime from RFC 5114
        let expected_p = BigUint::from_bytes_be(&hex::decode("B10B8F96A080E01DDE92DE5EAE5D54EC52C99FBCFB06A3C69A6A9DCA52D23B616073E28675A23D189838EF1E2EE652C013ECB4AEA906112324975C3CD49B83BFACCBDD7D90C4BD7098488E9C219A73724EFFD6FAE5644738FAA31A4FF55BCCC0A151AF5F0DC8B4BD45BF37DF365C1A65E68CFDA76D4DA708DF1FB2BC2E4A4371").unwrap());
        assert_eq!(p, expected_p);

        // Test that q matches the expected 160-bit prime from RFC 5114
        let expected_q = BigUint::from_bytes_be(
            &hex::decode("F518AA8781A8DF278ABA4E7D64B7CB9D49462353").unwrap(),
        );
        assert_eq!(q, expected_q);

        // Test that alpha matches the expected generator from RFC 5114
        let expected_alpha = BigUint::from_bytes_be(&hex::decode("A4D1CBD5C3FD34126765A442EFB99905F8104DD258AC507FD6406CFF14266D31266FEA1E5C41564B777E690F5504F213160217B4B01B886A5E91547F9E2749F4D7FBD7D3B9A92EE1909D0D2263F80A76A6A24C087A091F531DBF0A0169B6A28AD662A4D18E73AFA32D779D5918D08BC8858F4DCEF97C2A24855E6EEB22B3B2E5").unwrap());
        assert_eq!(alpha, expected_alpha);

        // Verify that beta is computed correctly (alpha^exp mod p)
        let exp = BigUint::from_bytes_be(&hex::decode("266FEA1E5C41564B777E69").unwrap());
        let expected_beta = alpha.modpow(&exp, &p);
        assert_eq!(beta, expected_beta);
    }

    #[test]
    fn test_get_constants_properties() {
        let (alpha, beta, p, q) = ZKP::get_constants();

        // Test that alpha and beta are within the correct range (1 < alpha, beta < p)
        assert!(alpha > BigUint::from(1u32));
        assert!(alpha < p);
        assert!(beta > BigUint::from(1u32));
        assert!(beta < p);

        // Test that q divides p-1 (this is a requirement for DSA parameters)
        let p_minus_1 = &p - BigUint::from(1u32);
        assert_eq!(&p_minus_1 % &q, BigUint::from(0u32));

        // Test that alpha^q mod p = 1 (alpha should have order q)
        assert_eq!(alpha.modpow(&q, &p), BigUint::from(1u32));

        // Test that beta^q mod p = 1 (beta should also have order q)
        assert_eq!(beta.modpow(&q, &p), BigUint::from(1u32));
    }

    #[test]
    fn test_get_constants_in_zkp_workflow() {
        let (alpha, beta, p, q) = ZKP::get_constants();

        let zkp = ZKP {
            p: p.clone(),
            q: q.clone(),
            alpha: alpha.clone(),
            beta: beta.clone(),
        };

        // Test that the constants work in a complete ZKP workflow
        let x = ZKP::generate_random_lower_than(&q);
        let k = ZKP::generate_random_lower_than(&q);
        let c = ZKP::generate_random_lower_than(&q);

        // Use compute_pair to generate the commitment values
        let (y1, y2) = zkp.compute_pair(&x);
        let (r1, r2) = zkp.compute_pair(&k);

        let s = zkp.solve(&k, &c, &x);
        let result = zkp.verify(&r1, &r2, &y1, &y2, &c, &s);

        // The ZKP protocol should work correctly with the constants
        assert!(result);
    }

    #[test]
    fn test_compute_pair_consistency() {
        let (alpha, beta, p, q) = ZKP::get_constants();

        let zkp = ZKP {
            p: p.clone(),
            q,
            alpha: alpha.clone(),
            beta: beta.clone(),
        };

        let exp1 = BigUint::from(3u32);
        let exp2 = BigUint::from(7u32);
        let combined_exp = &exp1 + &exp2; // exp1 + exp2 = 10

        let (p1_exp1, p2_exp1) = zkp.compute_pair(&exp1);
        let (p1_exp2, p2_exp2) = zkp.compute_pair(&exp2);
        let (p1_combined, p2_combined) = zkp.compute_pair(&combined_exp);

        // Test multiplicative property: (alpha^exp1 * alpha^exp2) mod p = alpha^(exp1+exp2) mod p
        let p1_product = (&p1_exp1 * &p1_exp2) % &p;
        let p2_product = (&p2_exp1 * &p2_exp2) % &p;

        assert_eq!(p1_combined, p1_product);
        assert_eq!(p2_combined, p2_product);
    }
}

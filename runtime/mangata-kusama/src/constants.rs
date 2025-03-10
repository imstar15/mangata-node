pub mod fee {
	use crate::{MangataExtrinsicBaseWeight, UNIT};
	use frame_support::weights::{
		constants::WEIGHT_PER_SECOND, WeightToFeeCoefficient, WeightToFeeCoefficients,
		WeightToFeePolynomial,
	};
	use mangata_types::Balance;
	use smallvec::smallvec;
	use sp_runtime::Perbill;

	pub const KSM_MGX_SCALE_FACTOR_UNADJUSTED: u128 = 10_000_000_000_u128; // 10_000 as KSM/MGX, with 6 decimals accounted for (12 - KSM, 18 - MGX)

	// on-chain fees are 10x more expensive then ~real rate
	pub const KSM_MGX_SCALE_FACTOR: u128 = 1000_000_000_u128; // 1000 as KSM/MGX, with 6 decimals accounted for (12 - KSM, 18 - MGX)
	pub const KAR_MGX_SCALE_FACTOR: u128 = KSM_MGX_SCALE_FACTOR / 100; // 100 as KAR/KSM
	pub const TUR_MGX_SCALE_FACTOR: u128 = KSM_MGX_SCALE_FACTOR; // 100 as TUR/KSM, with 2 decimals accounted for (10 - TUR, 12 - KSM)

	/// Handles converting a weight scalar to a fee value, based on the scale and granularity of the
	/// node's balance type.
	///
	/// This should typically create a mapping between the following ranges:
	///   - `[0, MAXIMUM_BLOCK_WEIGHT]`
	///   - `[Balance::min, Balance::max]`
	///
	/// Yet, it can be used for any other sort of change to weight-fee. Some examples being:
	///   - Setting it to `0` will essentially disable the weight fee.
	///   - Setting it to `1` will cause the literal `#[weight = x]` values to be charged.
	pub struct WeightToFee;
	impl WeightToFeePolynomial for WeightToFee {
		type Balance = Balance;
		fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
			// in Rococo, extrinsic base weight (smallest non-zero weight) is mapped to 1 MILLIUNIT:
			// in mangata, we map to 1/10 of that, or 1/10 MILLIUNIT
			let p = base_tx_in_mgx();
			let q = Balance::from(MangataExtrinsicBaseWeight::get());
			smallvec![WeightToFeeCoefficient {
				degree: 1,
				negative: false,
				coeff_frac: Perbill::from_rational(p % q, q),
				coeff_integer: p / q,
			}]
		}
	}

	pub fn base_tx_in_mgx() -> Balance {
		UNIT
	}

	pub fn mgx_per_second() -> u128 {
		let base_weight = Balance::from(MangataExtrinsicBaseWeight::get());
		let base_per_second = (WEIGHT_PER_SECOND.ref_time() / base_weight as u64) as u128;
		base_per_second * base_tx_in_mgx()
	}

	pub fn ksm_per_second() -> u128 {
		mgx_per_second() / KSM_MGX_SCALE_FACTOR_UNADJUSTED as u128
	}
}

pub mod parachains {
	pub mod mangata {
		pub const ID: u32 = 2110;
	}
	pub mod karura {
		pub const ID: u32 = 2000;
		pub const KAR_KEY: &[u8] = &[0, 128];
		pub const KUSD_KEY: &[u8] = &[0, 129];
		pub const LKSM_KEY: &[u8] = &[0, 131];
	}
	pub mod turing {
		pub const ID: u32 = 2114;
	}
	pub mod bifrost {
		pub const ID: u32 = 2001;
		pub const BNC_KEY: &[u8] = &[0, 1];
		pub const VSKSM_KEY: &[u8] = &[4, 4];
		pub const VKSM_KEY: &[u8] = &[1, 4];
	}
	pub mod imbue {
		pub const ID: u32 = 2121;
		pub const IMBU_KEY: &[u8] = &[0];
	}
	pub mod phala {
		pub const ID: u32 = 2004;
	}
}

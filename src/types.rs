use std::ffi::c_void;

pub type NfaTechnologyMask = u8;

pub const NFA_TECHNOLOGY_MASK_A: u8 = 0x01;
pub const NFA_TECHNOLOGY_MASK_B: u8 = 0x02;
pub const NFA_TECHNOLOGY_MASK_F: u8 = 0x04;
pub const NFA_TECHNOLOGY_MASK_V: u8 = 0x08;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct LnAtrResGenBytes([u8; 48]);

impl Default for LnAtrResGenBytes {
    fn default() -> Self {
        Self([0u8; 48])
    }
}

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct NfaListenGfg {
    pub la_enable: bool,
    pub la_bit_frame_sdd: u8,
    pub la_platform_config: u8,
    pub la_sel_info: u8,
    pub la_nfcid1_len: u8,
    pub la_nfcid1: [u8; 10],

    pub lb_enable: bool,
    pub lb_sensb_info: u8,
    pub lb_nfcid0_len: u8,
    pub lb_nfcid0: [u8; 4],
    pub lb_app_data: [u8; 4],
    pub lb_sfgi: u8,
    pub lb_adc_fo: u8,

    pub lf_enable: bool,
    pub lf_con_bitr_f: u8,
    pub lf_protocol_type: u8,
    pub lf_t3t_flags: u16,
    pub lf_t3t_identifier: [u8; 10],
    pub lf_t3t_pmm: [u8; 8],

    pub li_enable: bool,
    pub li_fwi: u8,
    pub la_hist_bytes_len: u8,
    pub la_hist_bytes: [u8; 15],
    pub lb_h_info_resp_len: u8,
    pub lb_h_info_resp: [u8; 15],

    pub ln_enable: bool,
    pub ln_wt: u8,
    pub ln_atr_res_gen_bytes_len: u8,
    pub ln_atr_res_gen_byte: LnAtrResGenBytes,
    pub ln_atr_res_config: u8,
}

pub type NfaCallback = extern "C" fn(u8, *const c_void);

pub extern "C" fn common_callback(ty: u8, ptr: *const c_void) {
    println!("ty: {}, ptr: {:?}", ty, ptr);
}

/* extern "C" { */
/* pub fn NFA_RequestExclusiveRfControl( */
/*     poll_mask: NfaTechnologyMask, */
/*     p_listen_cfg: *const NfaListenGfg, */
/*     p_conn_cback: NfaConnCallback, */
/*     p_ndef_cback: NfaNdefCallback, */
/* ) -> u8; */
/*  */
/* pub fn NFA_ReleaseExclusiveRfControl() -> u8; */
/* } */
